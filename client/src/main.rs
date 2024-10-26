use futures_util::{SinkExt, StreamExt};
use std::io::BufRead;
use tokio::runtime::Handle;
use tokio::task::spawn_blocking;

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

const SERVER: &str = "ws://127.0.0.1:3000/ws";

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

        while let Ok(_) = stdin.read_line(&mut buffer) {
            buffer.pop();
            println!("stdin: {}", buffer);

            handle.block_on(async {
                if let Err(e) = sender.send(Message::Text(buffer.clone())).await {
                    println!("Could not send {buffer} to server: {e}");
                }
            });
            buffer.clear();
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            // print message and break if instructed to do so
            println!("Message received from server: {msg}");

            if let Message::Close(_) = msg {
                break;
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
