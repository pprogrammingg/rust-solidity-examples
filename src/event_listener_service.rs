// events.rs

use ethers::prelude::*;
use crate::utils::{init_env, get_env_var};
use ethers::core::{
        abi::AbiDecode,
        types::{BlockNumber, Filter},
    };
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
    let client =  Provider::<Ws>::connect(wss_url).await?;
    
    // setup client
    let client = Arc::new(client.clone());

    let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
    println!("last_block is: {last_block}");   

    // filter on last 10 blocks
    let updated_message_filter =
        Filter::new().from_block(last_block - 10).event("UpdatedMessages(string,string)");

    let mut stream = client.subscribe_logs(&updated_message_filter).await?.take(2);

    while let Some(log) = stream.next().await {
        println!(
            "block: {:?}, tx: {:?}, data: {:?}",
            log.block_number,
            log.transaction_hash,
            String::decode(log.data)
        );
    }
    Ok(())
}