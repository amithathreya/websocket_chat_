mod websocket;
mod config;
use std::net::Shutdown;

use tokio::signal;
use tokio::net::{TcpListener,TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};
use crate::websocket::handler::handle_client;
#[tokio::main]
async fn main() ->std::io::Result<()>
{
    let config = config::Config::load_file("config.toml").expect("Error loading config");
     let listener = TcpListener::bind(config.ip_address).await.expect("Error connecting");
    println!("Listening to socket{:?}", listener);
    let shutdown_signal = signal::ctrl_c();
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