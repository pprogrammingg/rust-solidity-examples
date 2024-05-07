// src/events.rs

use ethers::prelude::*;
use std::sync::Arc;

const WSS_URL: &str = "wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27";

pub async fn start_event_listener() -> Result<()> {
    let provider = Provider::<Ws>::connect(WSS_URL).await?;
    let provider = Arc::new(provider);

    let event = Transfer::new::<_, Provider<Ws>>(Filter::new(), Arc::clone(&provider));
    let mut transfers = event.subscribe().await?.take(5);

    while let Some(log) = transfers.next().await {
        println!("Transfer Event: {:?}", log);
    }

    Ok(())
}
