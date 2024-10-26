use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::any,
    Router,
};
use axum_extra::TypedHeader;
use draft_together_data::{ChampionUpdate, Draft};
use tokio::sync::Mutex;
use tracing::info;

use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use axum::extract::connect_info::ConnectInfo;

use futures::{sink::SinkExt, stream::StreamExt};

mod league_data;

#[derive(Debug, Default, Clone)]
struct AppState {
    draft: Arc<Mutex<Draft>>,
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

    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route("/ws", any(ws_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .with_state(AppState::default());

    let latest_version = league_data::get_latest_ddragon_version().await;
    info!("latest league of legends version: {latest_version:?}");
    let latest_version = latest_version.unwrap();
    let download_path = league_data::get_ddragon_path_or_download(&latest_version).await;
    info!("download result: {download_path:?}");

    let decompressed_path = PathBuf::from(format!("dragontail-{latest_version}"));
    if !decompressed_path.exists() {
        info!("decompressing tarball: {download_path:?}");
        let dragon_dir =
            league_data::decompress_tarball(download_path.unwrap(), &decompressed_path);
        info!("decompression finished: {dragon_dir:?}");
    }
    let extracted_path = format!("dragontail-extracted-{latest_version}");
    let extract_data_result =
        league_data::extract_data_from_ddragon(decompressed_path, &extracted_path, &latest_version);
    info!(?extract_data_result);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr, app_state))
}

async fn handle_socket(socket: WebSocket, who: SocketAddr, app_state: AppState) {
    let (mut sender, mut receiver) = socket.split();

    let task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            // print message and break if instructed to do so
            println!("message received from {who}: {msg:?}");

            match msg {
                Message::Close(close_frame) => {
                    println!("{who} closed connection: {close_frame:?}");
                    break;
                }
                Message::Text(champion_update) => {
                    let champion_update: ChampionUpdate =
                        serde_json::from_str(&champion_update).unwrap();
                    println!("deserialized champion: {champion_update:?} from {who}");

                    let mut draft = app_state.draft.lock().await;
                    draft.update(champion_update);

                    if let Err(e) = sender
                        .send(Message::Text(
                            serde_json::to_string(&*draft)
                                .expect("serialization of draft should not fail"),
                        ))
                        .await
                    {
                        println!("failed to send back message to client {who}: {e}");
                    }
                }
                _ => {}
            }
        }
    });

    if let Err(error) = task.await {
        println!("error while receiving message from {who}: {error}");
    }

    println!("Websocket context {who} destroyed");
}
