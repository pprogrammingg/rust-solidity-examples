// events.rs

use ethers::prelude::*;
use crate::utils::{init_env, get_env_var};
use ethers::core::types::BlockNumber;
use std::sync::Arc;


#[derive(Debug, Clone, EthEvent)]
pub struct UpdatedMessages {
    pub old_str: String,
    pub new_str: String
}

pub async fn start_event_listener() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize dotenv to load environment variables
    init_env();

    // setup provider
    let wss_url = format!("{}/{}", get_env_var("INFURA_WS_URL", ""), get_env_var("INFURA_API_KEY", ""));
    let client =  Provider::<Ws>::connect(wss_url).await.unwrap();
    
    // setup client
    let client = Arc::new(client);

    // get last block number
    let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
    println!("last_block is: {last_block}");   

    // get UpdatedMessages event
    let event = Contract::event_of_type::<UpdatedMessages>(client);

    // Set up an stream of events, read them and log them
    let mut stream = event.subscribe_with_meta().await?;

    while let Some(Ok((log, meta))) = stream.next().await {
        println!("log is {log:?}");
        println!("log meta is {meta:?}")
    }
    Ok(())
}