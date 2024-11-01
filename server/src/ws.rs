use anyhow::Result;
use axum::{
    extract::{
        self,
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use axum_extra::TypedHeader;
use draft_together_data::ChampionUpdate;
use futures::{stream::SplitSink, SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::sync::broadcast::Sender;
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::{database, get_current_draft, get_current_draft_mut, AppState};

#[derive(Debug, Clone, Copy)]
pub enum WsEvent {
    DraftUpdate,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    extract::Path(draft_client_id): extract::Path<Uuid>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");

    ws.on_upgrade(move |socket| handle_socket(socket, addr, draft_client_id, app_state))
}

async fn handle_socket(socket: WebSocket, who: SocketAddr, draft_id: Uuid, app_state: AppState) {
    update_draft_connected_clients(&app_state, draft_id);

    let (mut sender, mut receiver) = socket.split();
    let draft_tx = app_state.events_sender.clone();
    let mut draft_rx = draft_tx.subscribe();

    let app_state_receiver = app_state.clone();
    let mut client_receiver_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            info!("message received from {who}: {msg:?}");
            match msg {
                Message::Close(close_frame) => {
                    debug!("{who} closed connection: {close_frame:?}");
                }
                Message::Text(champion_update) => {
                    if let Err(e) = receive_draft_update(
                        &app_state_receiver,
                        &draft_tx,
                        &champion_update,
                        draft_id,
                        &who,
                    )
                    .await
                    {
                        error!("stopping web socket from {who} because of an error while receiving draft update from client: {e}");
                        break;
                    }
                }
                _ => {}
            }
        }
    });

    let app_state_sender = app_state.clone();
    let mut send_draft_update_tasks = tokio::spawn(async move {
        while let Ok(event) = draft_rx.recv().await {
            if let Err(e) = send_draft_update(event, &mut sender, &app_state_sender, draft_id).await
            {
                error!("stopping web socket because of an error while sending draft update to client: {e}");
                break;
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

    // check if it was the last client
    if let Err(e) = update_database_if_last_client(&app_state, draft_id, &who).await {
        error!("error while updating database: {e}");
    }

    info!("Websocket context {who} destroyed");
}

async fn send_draft_update(
    event: WsEvent,
    sender: &mut SplitSink<WebSocket, Message>,
    app_state: &AppState,
    draft_id: Uuid,
) -> Result<()> {
    if let WsEvent::DraftUpdate = event {
        let server_draft = get_current_draft(app_state, draft_id).await?;
        sender
            .send(Message::Text(
                serde_json::to_string(&server_draft.draft)
                    .expect("serialization of draft should not fail"),
            ))
            .await?;
    }
    Ok(())
}

async fn receive_draft_update(
    app_state: &AppState,
    draft_tx: &Sender<WsEvent>,
    champion_update: &str,
    draft_id: Uuid,
    who: &SocketAddr,
) -> Result<()> {
    let champion_update: ChampionUpdate = serde_json::from_str(champion_update)?;

    if app_state
        .valid_champion_ids
        .read()
        .await
        .contains(&champion_update.champion_id)
    {
        let mut server_draft = get_current_draft_mut(app_state, draft_id).await?;
        server_draft.draft.update(&champion_update);
        draft_tx.send(WsEvent::DraftUpdate)?;
        debug!("{who} updated draft {draft_id} with {champion_update:?}");
    } else {
        error!(
            "champion update was not valid: champion_id: {} was not valid",
            champion_update.champion_id
        );
    }

    Ok(())
}

fn update_draft_connected_clients(app_state: &AppState, draft_id: Uuid) {
    if app_state.drafts_connected_clients.contains_key(&draft_id) {
        app_state
            .drafts_connected_clients
            .alter(&draft_id, |_, value| value.saturating_add(1))
    } else {
        app_state.drafts_connected_clients.insert(draft_id, 1);
    }
}

async fn update_database_if_last_client(
    app_state: &AppState,
    draft_id: Uuid,
    who: &SocketAddr,
) -> Result<()> {
    app_state
        .drafts_connected_clients
        .alter(&draft_id, |_, value| value.saturating_sub(1));
    let connected_clients = app_state.drafts_connected_clients.get(&draft_id);
    if connected_clients.is_none() {
        error!("at the end of the socket with {who}, the number of connected client is None. Trying to save in database anyway...");
    }
    if connected_clients.is_none() || connected_clients.is_some_and(|value| *value == 0) {
        let server_draft = app_state.drafts.get(&draft_id);
        if let Some(server_draft) = server_draft {
            database::update_draft(&app_state.pool, &server_draft).await?;

            info!("draft with id {draft_id} was successfully updated in database");
            app_state.drafts_connected_clients.remove(&draft_id);
            app_state.drafts.remove(&draft_id);

            debug!("no clients connected for draft with id {draft_id}, draft was removed from hashmaps");
        } else {
            error!(
                "error while trying to save draft with id {draft_id} to database: draft was None"
            );
        }
    }
    Ok(())
}
