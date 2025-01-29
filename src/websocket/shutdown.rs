use tokio::io::{self, AsyncBufReadExt};  // For async reading
use tokio::sync::broadcast;  // For broadcasting shutdown signal
#[allow(unused_imports)]
pub async fn shutdown_listener(mut shutdown_rx: broadcast::Receiver<()>) {
    // Wait for shutdown signal
    shutdown_rx.recv().await.ok();
    println!("Shutdown signal received.");
}

pub async fn listen_for_shutdown(shutdown_tx: tokio::sync::broadcast::Sender<()>) {

    let mut stdin = io::BufReader::new(io::stdin());  

    // Read lines asynchronously using `next_line` method
    let mut line = String::new();
    while let Ok(_) = stdin.read_line(&mut line).await {
        let input = line.trim();
        if input == "shutdown" {
            // Broadcast shutdown signal
            shutdown_tx.send(()).ok();
            println!("Shutdown initiated.");
            break;  // Exit the loop on 'shutdown'
        } else {
            println!("You typed: {}", input);
        }

        // Clear the buffer for the next line
        line.clear();
    }
}
