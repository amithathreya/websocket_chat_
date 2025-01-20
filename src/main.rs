mod websocket;
use tokio::net::{TcpListener,TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};
use crate::websocket::handler::handle_client;
#[tokio::main]
async fn main() ->std::io::Result<()>
{
     let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Error connecting");
    println!("Listening to socket{:?}", listener);

    loop {
        let(sock,addr) = listener.accept().await.expect("Error accepting socket");
        println!("Connected to: {}", addr);
        tokio::spawn(async move {
            if let Err(e) = handle_client(sock).await {
                eprintln!("Error handling client: {}", Box::new(e));
            }
        });
    };
}
