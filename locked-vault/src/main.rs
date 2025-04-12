#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

mod models;
mod errors;
mod events;
mod contract;
mod bitcoin;

use std::env;
use std::time::Duration;
use log::{debug, info, warn, error};
use env_logger::Env;

use bitcoin::testnet::{BitcoinTestnetConfig, utils};
use bitcoin::transfer::BitcoinTestnetTransfer;
use bitcoin::rpc::BitcoinRpcClient;
use bitcoin::mempool::MempoolMonitor;
use contract::contract_core::TimeLockedDeposit;
use models::TokenType;

fn main() -> Result<(), String> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    info!("Time-Locked Deposit Contract - Bitcoin Testnet Deployment");
    
    // Get configuration from environment variables
    let rpc_url = env::var("BITCOIN_TESTNET_RPC_URL")
        .unwrap_or_else(|_| "http://localhost:18332".to_string());
        
    let rpc_username = env::var("BITCOIN_TESTNET_RPC_USERNAME")
        .unwrap_or_else(|_| "testuser".to_string());
        
    let rpc_password = env::var("BITCOIN_TESTNET_RPC_PASSWORD")
        .unwrap_or_else(|_| "testpassword".to_string());
        
    let contract_wallet_address = env::var("BITCOIN_TESTNET_CONTRACT_WALLET")
        .unwrap_or_else(|_| "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string());
        
    let owner_address = env::var("BITCOIN_TESTNET_OWNER_ADDRESS")
        .unwrap_or_else(|_| "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string());
    
    let lightning_node_url = env::var("LIGHTNING_NODE_URL").ok();
    let ordinals_api_url = env::var("ORDINALS_API_URL").ok();
    
    // Create Bitcoin testnet configuration
    let config = BitcoinTestnetConfig::new(
        rpc_url,
        rpc_username,
        rpc_password,
        contract_wallet_address,
    );
    
    // Validate configuration
    config.validate()?;
    
    // Create Bitcoin testnet transfer implementation with all clients
    let transfer = BitcoinTestnetTransfer::new_with_clients(
        config.clone(),
        lightning_node_url,
        ordinals_api_url,
    ).map_err(|e| format!("Failed to initialize transfer: {:?}", e))?;
    
    // Create contract instance
    let mut contract = TimeLockedDeposit::new(
        owner_address,
        10, // 10% emergency withdrawal fee
        transfer,
    ).map_err(|e| format!("Failed to initialize contract: {:?}", e))?;
    
    // Print contract information
    info!("Contract initialized successfully!");
    info!("Network type: {}", contract.get_network_type());
    info!("Is testnet: {}", contract.is_testnet());
    
    // Example: Validate a testnet address
    let example_address = "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx";
    info!("Validating address {}: {}", example_address, utils::validate_testnet_address(example_address));
    
    // Example: Validate a Rune token
    let rune_token = TokenType::Rune("RUNE_TEST_TOKEN_123".to_string());
    match rune_token.validate() {
        Ok(_) => info!("Rune token validation successful"),
        Err(e) => warn!("Rune token validation failed: {}", e),
    }
    
    // Example: Validate an Ordinal token
    let ordinal_token = TokenType::Ordinal("0".repeat(64));
    match ordinal_token.validate() {
        Ok(_) => info!("Ordinal token validation successful"),
        Err(e) => warn!("Ordinal token validation failed: {}", e),
    }
    
    info!("Contract is ready for Bitcoin testnet operations");
    
    // Keep the application running
    loop {
        std::thread::sleep(Duration::from_secs(60));
        info!("Contract is running...");
    }
}
