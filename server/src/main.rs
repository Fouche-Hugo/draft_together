use axum::{
    extract::{self, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{any, get},
    Json, Router,
};
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};
use database::ChampionDatabaseInsertion;
use draft_together_data::{Champion, Draft};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, error, info, trace};
use uuid::Uuid;
use ws::WsEvent;

use std::{collections::HashSet, env, net::SocketAddr, path::PathBuf, sync::Arc, time::Duration};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use anyhow::Result;

mod database;
mod league_data;
mod ws;

#[derive(thiserror::Error, Debug)]
enum ApiError {
    #[error("error while fetching database: {0}")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        error!("an error has occured while fetching api: {self}");
        match self {
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

#[derive(Debug, Clone)]
struct AppState {
    pool: Arc<PgPool>,
    drafts: Arc<DashMap<Uuid, ServerDraft>>,
    drafts_connected_clients: Arc<DashMap<Uuid, u32>>,
    valid_champion_ids: Arc<RwLock<HashSet<i32>>>,
    events_sender: Arc<broadcast::Sender<WsEvent>>,
    _events_receiver: Arc<broadcast::Receiver<WsEvent>>,
}

#[derive(Debug)]
struct ServerDraft {
    id: i32,
    draft: Draft,
}

impl ServerDraft {
    pub fn new(id: i32, draft: Draft) -> Self {
        Self { id, draft }
    }
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

    let database_password = env::var("DATABASE_PASSWORD").unwrap_or("default_password".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&format!(
            "postgres://draft_together:{database_password}@database/draft_together"
        ))
        .await
        .unwrap();

    let champions = database::query_champions(&pool).await.unwrap();
    let valid_champion_ids = champions.iter().map(|champion| champion.id).collect();

    let (draft_tx, draft_rx) = broadcast::channel(16);
    let app_state = AppState {
        pool: Arc::new(pool),
        valid_champion_ids: Arc::new(RwLock::new(valid_champion_ids)),
        _events_receiver: Arc::new(draft_rx),
        events_sender: Arc::new(draft_tx),
        drafts: Arc::new(DashMap::new()),
        drafts_connected_clients: Arc::new(DashMap::default()),
    };

    {
        let app_state = app_state.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = update_riot_data(&app_state).await {
                    error!("error while updating riot data: {e}");
                }
                tokio::time::sleep(Duration::from_secs(60 * 60)).await
            }
        });
    }

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route("/ws/:draft_client_id", any(ws::ws_handler))
        .route("/draft/:client_id", get(get_draft))
        .route("/champions", get(get_champions))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn update_riot_data(app_state: &AppState) -> Result<()> {
    let pool = &app_state.pool;
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

            if !champion_exists {
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

        match database::query_champions(pool).await {
            Ok(champions_updated) => {
                let mut valid_champions_ids = app_state.valid_champion_ids.write().await;
                *valid_champions_ids = champions_updated
                    .iter()
                    .map(|champion| champion.id)
                    .collect();
            }
            Err(e) => error!("failed to get champion updated after data update: {e}"),
        }
    }

    info!("riot data successfully updated to version {latest_version}");

    Ok(())
}

async fn get_draft(
    extract::Path(client_id): extract::Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<Json<Draft>, ApiError> {
    get_current_draft(&app_state, client_id)
        .await
        .map(|server_draft| Json(server_draft.draft.clone()))
        .map_err(|e| e.into())
}

async fn get_champions(State(app_state): State<AppState>) -> Result<Json<Vec<Champion>>, ApiError> {
    let champions = database::query_champions(&app_state.pool).await?;
    let champions = champions
        .into_iter()
        .map(|champion| champion.into())
        .collect();

    Ok(Json(champions))
}

async fn get_current_draft(
    app_state: &AppState,
    draft_id: Uuid,
) -> Result<Ref<'_, Uuid, ServerDraft>, sqlx::Error> {
    let draft = app_state.drafts.get(&draft_id);

    if let Some(draft) = draft {
        Ok(draft)
    } else if database::draft_exists(&app_state.pool, draft_id).await? {
        let draft = database::query_draft_by_client_id(&app_state.pool, draft_id)
            .await?
            .into();
        app_state.drafts.insert(draft_id, draft);
        Ok(app_state
            .drafts
            .get(&draft_id)
            .expect("draft should be in the dashmap, as it was just inserted"))
    } else {
        let id = database::new_draft(&app_state.pool, draft_id).await?;
        let draft = Draft::default();
        let server_draft = ServerDraft::new(id, draft);
        app_state.drafts.insert(draft_id, server_draft);
        Ok(app_state
            .drafts
            .get(&draft_id)
            .expect("draft should be in the dashmap, as it was just inserted"))
    }
}

async fn get_current_draft_mut(
    app_state: &AppState,
    draft_id: Uuid,
) -> Result<RefMut<'_, Uuid, ServerDraft>, sqlx::Error> {
    let draft = app_state.drafts.get_mut(&draft_id);

    if let Some(draft) = draft {
        Ok(draft)
    } else if database::draft_exists(&app_state.pool, draft_id).await? {
        let draft = database::query_draft_by_client_id(&app_state.pool, draft_id)
            .await?
            .into();
        app_state.drafts.insert(draft_id, draft);
        Ok(app_state
            .drafts
            .get_mut(&draft_id)
            .expect("draft should be in the dashmap, as it was just inserted"))
    } else {
        let id = database::new_draft(&app_state.pool, draft_id).await?;
        let draft = Draft::default();
        let server_draft = ServerDraft::new(id, draft);
        app_state.drafts.insert(draft_id, server_draft);
        Ok(app_state
            .drafts
            .get_mut(&draft_id)
            .expect("draft should be in the dashmap, as it was just inserted"))
    }
}
