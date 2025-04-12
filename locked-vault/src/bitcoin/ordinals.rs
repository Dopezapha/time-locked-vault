use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::errors::ContractError;
use crate::bitcoin::rpc::BitcoinRpcClient;


/// Ordinal inscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inscription {
    /// Inscription ID
    pub id: String,
    /// Transaction ID
    pub txid: String,
    /// Output index
    pub vout: u32,
    /// Inscription number
    pub number: u64,
    /// Content type
    pub content_type: String,
    /// Content (hex encoded)
    pub content: String,
    /// Timestamp
    pub timestamp: u64,
    /// Owner address
    pub owner: String,
    /// Satoshi offset
    pub offset: u64,
}

/// Ordinals client
#[derive(Debug)]
pub struct OrdinalsClient {
    /// Bitcoin RPC client
    bitcoin_rpc: Arc<BitcoinRpcClient>,
    /// Ordinals API URL
    api_url: String,
    /// Inscriptions cache
    inscriptions: Arc<Mutex<HashMap<String, Inscription>>>,
    /// Last API call timestamp for rate limiting
    last_api_call: Arc<Mutex<Instant>>,
}

impl OrdinalsClient {
    /// Create a new Ordinals client
    pub fn new(
        bitcoin_rpc: Arc<BitcoinRpcClient>,
        api_url: String,
    ) -> Self {
        Self {
            bitcoin_rpc,
            api_url,
            inscriptions: Arc::new(Mutex::new(HashMap::new())),
            last_api_call: Arc::new(Mutex::new(Instant::now())),
        }
    }
    
    /// Make an API call with rate limiting
    fn rate_limit(&self) -> Result<(), ContractError> {
        let mut last_call = self.last_api_call.lock()
            .map_err(|_| ContractError::BitcoinTestnetError("Failed to acquire lock".to_string()))?;
        
        // Check rate limit
        let elapsed = last_call.elapsed();
        let min_interval = Duration::from_millis(200); // 5 calls per second
        
        if elapsed < min_interval {
            // Sleep to respect rate limit
            std::thread::sleep(min_interval - elapsed);
        }
        
        // Update last call timestamp
        *last_call = Instant::now();
        
        Ok(())
    }
    
    /// Get inscription by ID
    pub fn get_inscription(&self, inscription_id: &str) -> Result<Inscription, ContractError> {
        self.rate_limit()?;
        
        // Check cache
        {
            let inscriptions = self.inscriptions.lock()
                .map_err(|_| ContractError::BitcoinTestnetError("Failed to acquire lock".to_string()))?;
            
            if let Some(inscription) = inscriptions.get(inscription_id) {
                return Ok(inscription.clone());
            }
        }
        
        // In a real implementation, this would call the Ordinals API
        // For now, we'll simulate it
        
        let inscription = Inscription {
            id: inscription_id.to_string(),
            txid: format!("txid_{}", inscription_id),
            vout: 0,
            number: 12345,
            content_type: "image/png".to_string(),
            content: "...".to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            owner: "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
            offset: 0,
        };
        
        // Cache the inscription
        let mut inscriptions = self.inscriptions.lock()
            .map_err(|_| ContractError::BitcoinTestnetError("Failed to acquire lock".to_string()))?;
        
        inscriptions.insert(inscription_id.to_string(), inscription.clone());
        
        Ok(inscription)
    }
    
    /// Get inscriptions by address
    pub fn get_inscriptions_by_address(&self, address: &str) -> Result<Vec<Inscription>, ContractError> {
        self.rate_limit()?;
        
        // In a real implementation, this would call the Ordinals API
        // For now, we'll simulate it
        
        let mut inscriptions = Vec::new();
        
        for i in 0..3 {
            let id = format!("inscription_{}_for_{}", i, address);
            
            let inscription = Inscription {
                id: id.clone(),
                txid: format!("txid_{}", id),
                vout: 0,
                number: 12345 + i,
                content_type: "image/png".to_string(),
                content: "...".to_string(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                owner: address.to_string(),
                offset: 0,
            };
            
            inscriptions.push(inscription);
        }
        
        Ok(inscriptions)
    }
    
    /// Transfer an inscription
    pub fn transfer_inscription(
        &self,
        inscription_id: &str,
        from_address: &str,
        to_address: &str,
    ) -> Result<String, ContractError> {
        self.rate_limit()?;
        
        // Get the inscription
        let inscription = self.get_inscription(inscription_id)?;
        
        // Check ownership
        if inscription.owner != from_address {
            return Err(ContractError::Unauthorized);
        }
        
        // In a real implementation, this would create and send a transaction
        // For now, we'll simulate it
        
        let txid = format!("transfer_tx_{}", Instant::now().elapsed().as_nanos());
        
        // Update the inscription in cache
        let mut inscriptions = self.inscriptions.lock()
            .map_err(|_| ContractError::BitcoinTestnetError("Failed to acquire lock".to_string()))?;
        
        if let Some(inscription) = inscriptions.get_mut(inscription_id) {
            inscription.owner = to_address.to_string();
        }
        
        Ok(txid)
    }
    
    /// Create a new inscription
    pub fn create_inscription(
        &self,
        _content_type: &str,
        _content: &[u8],
        _fee_rate: f64,
    ) -> Result<String, ContractError> {
        self.rate_limit()?;
        
        // In a real implementation, this would create and send a transaction
        // For now, we'll simulate it
        
        let txid = format!("inscription_tx_{}", Instant::now().elapsed().as_nanos());
        
        Ok(txid)
    }
    
    /// Get the current fee to create an inscription
    pub fn get_inscription_fee(&self, content_size: usize, fee_rate: f64) -> Result<u64, ContractError> {
        // Estimate the size of the inscription transaction
        let tx_size = 200 + content_size; // Base size + content size
        
        // Calculate fee
        let fee = (tx_size as f64 * fee_rate / 1000.0) as u64;
        
        Ok(fee)
    }
}
