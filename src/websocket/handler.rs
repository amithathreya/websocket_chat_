
use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};

pub async fn handle_client(stream:TcpStream)->Result<() , Box<dyn std::error::Error>> {
    let ws_stream = accept_async(stream).await.expect("Error suring websocket handshake");
        let (mut write, mut read) = ws_stream.split();
    
        write.send(Message::Text("Welcome to the WebSocket server!".into())).await?;
    
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => {
    
                    println!("Received: {:?}", msg);
                    write.send(msg).await?;
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return Err(Box::new(e));
                }
            }
        }
        Ok(())
    
}