use std::str::FromStr;
use bitcoincore_rpc::bitcoin::{Address, Network};

/// Configuration for Bitcoin testnet
#[derive(Debug, Clone)]
pub struct BitcoinTestnetConfig {
    /// RPC URL for Bitcoin testnet node
    pub rpc_url: String,
    /// RPC username
    pub rpc_username: String,
    /// RPC password
    pub rpc_password: String,
    /// Contract wallet address
    pub contract_wallet_address: String,
    /// Maximum batch size for transactions
    pub max_batch_size: u32,
    /// Rate limit (calls per minute)
    pub rate_limit: u32,
    /// Minimum confirmations required
    pub min_confirmations: u32,
}

impl BitcoinTestnetConfig {
    /// Create a new Bitcoin testnet configuration
    pub fn new(
        rpc_url: String,
        rpc_username: String,
        rpc_password: String,
        contract_wallet_address: String,
    ) -> Self {
        Self {
            rpc_url,
            rpc_username,
            rpc_password,
            contract_wallet_address,
            max_batch_size: 10,
            rate_limit: 60,
            min_confirmations: 1,
        }
    }
    
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate RPC URL
        if self.rpc_url.is_empty() {
            return Err("RPC URL cannot be empty".to_string());
        }
        
        if !self.rpc_url.starts_with("http://") && !self.rpc_url.starts_with("https://") {
            return Err("RPC URL must start with http:// or https://".to_string());
        }
        
        // Validate RPC credentials
        if self.rpc_username.is_empty() {
            return Err("RPC username cannot be empty".to_string());
        }
        
        if self.rpc_password.is_empty() {
            return Err("RPC password cannot be empty".to_string());
        }
        
        // Validate contract wallet address
        if !utils::validate_testnet_address(&self.contract_wallet_address) {
            return Err("Invalid testnet address for contract wallet".to_string());
        }
        
        // Validate batch size
        if self.max_batch_size == 0 {
            return Err("Maximum batch size cannot be zero".to_string());
        }
        
        // Validate rate limit
        if self.rate_limit == 0 {
            return Err("Rate limit cannot be zero".to_string());
        }
        
        // Validate confirmations
        if self.min_confirmations == 0 {
            return Err("Minimum confirmations cannot be zero".to_string());
        }
        
        Ok(())
    }
}

/// Utility functions for Bitcoin testnet
pub mod utils {
    use super::*;
    
    /// Validate a Bitcoin testnet address
    pub fn validate_testnet_address(address: &str) -> bool {
        if address.is_empty() {
            return false;
        }
        
        // Check address format
        match Address::from_str(address) {
            Ok(addr) => {
                // Check network - in newer versions we need to check differently
                let network = addr.network.clone();
                match network {
                    Network::Testnet | Network::Regtest | Network::Signet => true,
                    _ => false,
                }
            },
            Err(_) => false,
        }
    }
    
    /// Convert satoshis to BTC
    pub fn satoshi_to_btc(satoshi: u64) -> f64 {
        satoshi as f64 / 100_000_000.0
    }
    
    /// Convert BTC to satoshis
    pub fn btc_to_satoshi(btc: f64) -> u64 {
        (btc * 100_000_000.0) as u64
    }
    
    /// Format a Bitcoin amount for display
    pub fn format_bitcoin_amount(satoshi: u64) -> String {
        format!("{:.8} BTC", satoshi_to_btc(satoshi))
    }
    
    /// Estimate transaction fee
    pub fn estimate_tx_fee(tx_size: u64, fee_rate: f64) -> u64 {
        (tx_size as f64 * fee_rate / 1000.0) as u64
    }
    
    /// Estimate transaction size
    pub fn estimate_tx_size(input_count: usize, output_count: usize) -> u64 {
        // Base transaction size
        let base_size: u64 = 10;
        
        // Input size (P2WPKH)
        let input_size: u64 = 148;
        
        // Output size
        let output_size: u64 = 34;
        
        base_size + (input_count as u64 * input_size) + (output_count as u64 * output_size)
    }
}
