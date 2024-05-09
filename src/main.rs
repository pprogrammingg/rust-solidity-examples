// main.rs
use ether_test_rust::{event_listener_service, interactions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    event_listener_service::start_event_listener().await?;
    // interactions::read_write_data().await?;
    Ok(())
}