use axum::{routing::any, Router};
use database::ChampionDatabaseInsertion;
use draft_together_data::Draft;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::sync::{broadcast, Mutex};
use tracing::{debug, error, info, trace};
use ws::WsEvent;

use std::{env, net::SocketAddr, path::PathBuf, sync::Arc, time::Duration};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use anyhow::Result;

mod database;
mod league_data;
mod ws;

#[derive(Debug, Clone)]
struct AppState {
    draft: Arc<Mutex<Draft>>,
    events_sender: Arc<broadcast::Sender<WsEvent>>,
    events_receiver: Arc<broadcast::Receiver<WsEvent>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=trace,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    let (draft_tx, draft_rx) = broadcast::channel(16);
    let app_state = AppState {
        events_receiver: Arc::new(draft_rx),
        events_sender: Arc::new(draft_tx),
        draft: Arc::new(Mutex::new(Draft::default())),
    };

    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route("/ws", any(ws::ws_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .with_state(app_state);

    let database_password = env::var("DATABASE_PASSWORD").unwrap_or("default_password".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&format!(
            "postgres://draft_together:{database_password}@database/draft_together"
        ))
        .await
        .unwrap();

    tokio::spawn(async move {
        loop {
            if let Err(e) = update_riot_data(&pool).await {
                error!("error while update riot data: {e}");
            }
            tokio::time::sleep(Duration::from_secs(60 * 60)).await
        }
    });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn update_riot_data(pool: &PgPool) -> Result<()> {
    let latest_version = league_data::get_latest_ddragon_version().await?;
    debug!("latest league of legends version: {latest_version:?}");

    let download_path = league_data::get_ddragon_path_or_download(&latest_version).await?;
    debug!("download result: {download_path:?}");

    let decompressed_path = PathBuf::from(format!("dragontail-{latest_version}"));
    let extracted_path = PathBuf::from(format!("dragontail-extracted-{latest_version}"));
    if !decompressed_path.exists() {
        debug!("decompressing tarball: {download_path:?}");
        let dragon_dir = league_data::decompress_tarball(download_path, &decompressed_path);
        debug!("decompression finished: {dragon_dir:?}");
    }
    if !extracted_path.exists() {
        let champions_data_dragon = league_data::extract_data_from_ddragon(
            decompressed_path,
            &extracted_path,
            &latest_version,
        )?;
        trace!(?champions_data_dragon);

        for champion in champions_data_dragon {
            let champion_exists = database::champion_exists(pool, &champion.riot_id).await?;

            if champion_exists {
                let champion_database = ChampionDatabaseInsertion {
                    riot_id: champion.riot_id,
                    name: champion.name,
                    default_skin_image_path: champion.default_skin_image_path,
                    centered_default_skin_image_path: champion.centered_default_skin_image_path,
                };
                database::insert_champion(pool, &champion_database).await?;
                trace!("{} inserted into database", champion_database.name);
            } else {
                trace!(
                    "champion {} already exists in database, skipping insertion",
                    champion.name
                );
            }
        }
    }

    info!("riot data successfully updated to version {latest_version}");

    Ok(())
}
