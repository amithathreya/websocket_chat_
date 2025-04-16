use tokio::io::{self, AsyncBufReadExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use futures_util::{StreamExt, SinkExt};
use url::Url;

pub async fn connect_to_server() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("ws://0.0.0.0:8080")?;
    let (ws_stream, _) = connect_async(url).await?;
    println!("Connected to the server!");

    let (mut write, mut read) = ws_stream.split();

    // Spawn a task to send messages to the server
    tokio::spawn(async move {
        let mut reader = io::BufReader::new(io::stdin()).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            if write.send(Message::Text(line)).await.is_err() {
                break;
            }
        }
    });
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Received: {}", text);
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = connect_to_server().await {
        eprintln!("Application error: {}", e);
    }
}
