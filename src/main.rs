mod websocket;
mod config;
use tokio::sync::broadcast;
use tokio::net::TcpListener;
use crate::websocket::shutdown::listen_for_shutdown;
use crate::websocket::shutdown::shutdown_listener; 
use crate::websocket::handler::handle_client;
#[allow(unused_imports)]
#[tokio::main]
async fn main() ->std::io::Result<()>
{
    let (shutdown_tx,_) = broadcast::channel(1);
    let (tx, _) = broadcast::channel(100); // Create a broadcast channel for messages
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
                let tx = tx.clone();
                let mut rx = tx.subscribe();

                tokio::spawn(async move {
                    if let Err(e) = handle_client(sock, tx, rx).await {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
        } => {},
    }

    Ok(())
}