// main.rs

mod utils; 
use utils::{init_env, get_env_var};
use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use ethers_solc::Solc;
use std::{env, path::Path, sync::Arc};
use serde_json;

#[tokio::main]
async fn read_write_data() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize dotenv to load environment variables
    init_env();

    // Retrieve the value of INFURA_API_KEY environment variable
    let api_key = get_env_var("INFURA_API_KEY", "default_api_key");
    println!("Infura API Key: {}", api_key);
    
    abigen!(myContract, "./src/HelloWorld.json");

    let rpc_url = format!("{}/{}", env::var("INFURA_API_URL"), env::var("INFURA_API_KEY")?);
    let provider = Provider::<Http>::try_from(rpc_url.as_str())?;
    let provider = Arc::new(provider);

    // Create signing wallet
    let wallet: LocalWallet = env::var("PRIVATE_KEY")?
    .parse::<LocalWallet>()?
    .with_chain_id(Chain::OptimismSepolia); 

    // Create client from signer wallet and provider
    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));

    let contract_address =env::var("CONTRACT_ADDRESS")?.parse::<Address>()?;
    let contract = myContract::new(contract_address, client.clone());

    // first time read msg
    let msg = contract.message().call().await.unwrap();
    println!("First time message read from contract ====>  {}", msg);

    // update the mesg
    let tx = contract.update("New new 2".to_owned()).send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);


    // second time read msg
    let msg2 = contract.message().call().await.unwrap();
    println!("Second time message read from contract ====>  {}", msg2);

    Ok(())
}