use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::any,
    Router,
};
use axum_extra::TypedHeader;

use std::{net::SocketAddr, path::PathBuf};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use axum::extract::connect_info::ConnectInfo;

use futures::{sink::SinkExt, stream::StreamExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
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
        );

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
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(socket: WebSocket, who: SocketAddr) {
    let (mut sender, mut receiver) = socket.split();

    let task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            // print message and break if instructed to do so
            println!("message received from {who}: {msg:?}");
            if let Message::Close(close_frame) = msg {
                println!("{who} closed connection: {close_frame:?}");
                break;
            } else {
                if let Err(e) = sender
                    .send(Message::Text("message received from server".to_string()))
                    .await
                {
                    println!("failed to send back message to client {who}: {e}");
                }
            }
        }
    });

    if let Err(error) = task.await {
        println!("error while receiving message from {who}: {error}");
    }

    println!("Websocket context {who} destroyed");
}
