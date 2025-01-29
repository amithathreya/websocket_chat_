mod websocket;
mod config;
use tokio::sync::broadcast;
use tokio::net::{TcpListener,TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};
use crate::websocket::handler::handle_client;  
use crate::websocket::shutdown::listen_for_shutdown;
use crate::websocket::shutdown::shutdown_listener; 
#[allow(unused_imports)]
#[tokio::main]
async fn main() ->std::io::Result<()>
{
    let (shutdown_tx,_) = broadcast::channel(1);
    let config = config::Config::load_file("config.toml").expect("Error loading config");
     let listener = TcpListener::bind(config.ip_address).await.expect("Error connecting");
    println!("Listening to socket{:?}", listener);
    let shutdown_rx = shutdown_tx.subscribe();
    tokio::spawn(shutdown_listener(shutdown_rx));
    let shutdown_signal = listen_for_shutdown(shutdown_tx.clone());
        tokio::select! {
        _ = shutdown_signal => {
            println!("Shutdown signal received. Shutting down server...");
        }
        _ = async {
            loop {
                let (sock, addr) = listener.accept().await.expect("Error accepting socket");
                println!("Connected to: {}", addr);

                tokio::spawn(async move {
                    if let Err(e) = websocket::handler::handle_client(sock).await {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
        } => {},
    }

    Ok(())
}