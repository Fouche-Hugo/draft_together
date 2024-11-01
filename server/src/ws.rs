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
use dashmap::mapref::one::{Ref, RefMut};
use draft_together_data::{ChampionUpdate, Draft};
use futures::{stream::SplitSink, SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::sync::broadcast::Sender;
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::{database, AppState};

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

    let mut send_draft_update_tasks = tokio::spawn(async move {
        while let Ok(event) = draft_rx.recv().await {
            if let Err(e) = send_draft_update(event, &mut sender, &app_state, draft_id).await {
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

    info!("Websocket context {who} destroyed");
}

async fn get_current_draft(
    app_state: &AppState,
    draft_id: Uuid,
) -> Result<Ref<'_, Uuid, Draft>, sqlx::Error> {
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
        database::new_draft(&app_state.pool, draft_id).await?;
        let draft = Draft::default();
        app_state.drafts.insert(draft_id, draft);
        Ok(app_state
            .drafts
            .get(&draft_id)
            .expect("draft should be in the dashmap, as it was just inserted"))
    }
}

async fn get_current_draft_mut(
    app_state: &AppState,
    draft_id: Uuid,
) -> Result<RefMut<'_, Uuid, Draft>, sqlx::Error> {
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
        database::new_draft(&app_state.pool, draft_id).await?;
        let draft = Draft::default();
        app_state.drafts.insert(draft_id, draft);
        Ok(app_state
            .drafts
            .get_mut(&draft_id)
            .expect("draft should be in the dashmap, as it was just inserted"))
    }
}

async fn send_draft_update(
    event: WsEvent,
    sender: &mut SplitSink<WebSocket, Message>,
    app_state: &AppState,
    draft_id: Uuid,
) -> Result<()> {
    if let WsEvent::DraftUpdate = event {
        let draft = get_current_draft(app_state, draft_id).await?;
        sender
            .send(Message::Text(
                serde_json::to_string(draft.value())
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
        let mut draft = get_current_draft_mut(app_state, draft_id).await?;
        draft.update(&champion_update);
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
