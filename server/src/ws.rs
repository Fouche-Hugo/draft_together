use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use axum_extra::TypedHeader;
use draft_together_data::ChampionUpdate;
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tracing::{debug, error, info};

use crate::AppState;

#[derive(Debug, Clone, Copy)]
pub enum WsEvent {
    DraftUpdate,
}

pub async fn ws_handler(
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
    let draft_tx = app_state.events_sender.clone();
    let mut draft_rx = draft_tx.subscribe();

    let draft = app_state.draft.clone();
    let mut client_receiver_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            // print message and break if instructed to do so
            info!("message received from {who}: {msg:?}");

            match msg {
                Message::Close(close_frame) => {
                    debug!("{who} closed connection: {close_frame:?}");
                    break;
                }
                Message::Text(champion_update) => {
                    let champion_update: ChampionUpdate =
                        match serde_json::from_str(&champion_update) {
                            Ok(value) => value,
                            Err(e) => {
                                error!("failed to deserialize champion update: {e}");
                                continue;
                            }
                        };
                    debug!("deserialized champion: {champion_update:?} from {who}");

                    if app_state
                        .valid_champion_ids
                        .read()
                        .await
                        .contains(&champion_update.champion_id)
                    {
                        let mut draft = draft.write().await;
                        draft.update(&champion_update);
                        if let Err(e) = draft_tx.send(WsEvent::DraftUpdate) {
                            error!("failed to send draft update to draft_tx channel: {e}");
                        }
                    } else {
                        error!(
                            "champion update was not valid: champion_id: {} was not valid",
                            champion_update.champion_id
                        );
                    }
                }
                _ => {}
            }
        }
    });

    let mut send_draft_update_tasks = tokio::spawn(async move {
        while let Ok(event) = draft_rx.recv().await {
            if let WsEvent::DraftUpdate = event {
                let draft = app_state.draft.read().await;
                if let Err(e) = sender
                    .send(Message::Text(
                        serde_json::to_string(&*draft)
                            .expect("serialization of draft should not fail"),
                    ))
                    .await
                {
                    error!("failed to send back message to client {who}: {e}");
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut client_receiver_task) => {
            send_draft_update_tasks.abort();
        },
        _ = (&mut send_draft_update_tasks) => {
            client_receiver_task.abort();
        }
    }

    info!("Websocket context {who} destroyed");
}
