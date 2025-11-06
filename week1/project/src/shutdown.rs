use tokio::signal;

pub async fn graceful_shutdown() {
    signal::ctrl_c().await.expect("Failed to listen for ctrl_c");
    println!("Shutting down gracefully...");
}
