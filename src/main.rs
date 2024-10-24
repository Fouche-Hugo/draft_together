use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new().route("/", get(|| async { "Hello world" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3100").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
