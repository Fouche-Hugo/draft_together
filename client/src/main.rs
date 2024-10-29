use draft_together_data::Draft;
use futures_util::{SinkExt, StreamExt};
use std::io::BufRead;
use tokio::runtime::Handle;
use tokio::task::spawn_blocking;

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

const SERVER: &str = "ws://127.0.0.1:3636/ws";

#[tokio::main]
async fn main() {
    spawn_client(1).await;
}

async fn spawn_client(who: usize) {
    let ws_stream = match connect_async(SERVER).await {
        Ok((stream, response)) => {
            println!("Handshake for client {who} has been completed");
            println!("Server response was {response:?}");
            stream
        }
        Err(e) => {
            println!("WebSocket handshake for client {who} failed with {e}!");
            return;
        }
    };

    let (mut sender, mut receiver) = ws_stream.split();

    let mut send_task = spawn_blocking(move || {
        let handle = Handle::current();

        let mut buffer = String::new();
        let mut stdin = std::io::stdin().lock();

        while stdin.read_line(&mut buffer).is_ok() {
            buffer.pop();
            println!("stdin: {}", buffer);

            let send_result =
                handle.block_on(async { sender.send(Message::Text(buffer.clone())).await });

            if let Err(e) = send_result {
                println!("failed to send data to server: {e}");
                break;
            }

            buffer.clear();
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            // print message and break if instructed to do so
            println!("Message received from server: {msg}");

            match msg {
                Message::Close(_) => break,
                Message::Text(msg) => {
                    let draft: Draft = match serde_json::from_str(&msg) {
                        Ok(draft) => draft,
                        Err(e) => {
                            println!("failed to deserialize message as draft: {e}");
                            break;
                        }
                    };
                    println!("current draft state: {}", draft.display());
                }
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        },
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }
}
