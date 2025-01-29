use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_client(
    stream: TcpStream,
    tx: broadcast::Sender<Message>,
    mut rx: broadcast::Receiver<Message>
) -> Result<(), Box<dyn std::error::Error>> {
    let ws_stream = accept_async(stream).await.expect("Error during websocket handshake");
    let (write, mut read) = ws_stream.split();
    let write = Arc::new(Mutex::new(write));

    {
        let write = write.clone();
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                let mut write = write.lock().await;
                if let Err(e) = write.send(msg).await {
                    eprintln!("Error sending message to client: {}", e);
                }
            }
        });
    }

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                println!("Received: {:?}", msg);
                tx.send(msg).ok(); // Broadcast the message to all clients
                let mut write = write.lock().await;
                write.send(Message::Text("Message delivered".into())).await?;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(Box::new(e));
            }
        }
    }
    Ok(())
}