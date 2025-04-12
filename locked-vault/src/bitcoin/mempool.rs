use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use log::{debug, info, error};

use crate::errors::ContractError;
use crate::bitcoin::rpc::BitcoinRpcClient;

/// Mempool transaction
#[derive(Debug, Clone)]
pub struct MempoolTransaction {
    /// Transaction ID
    pub txid: String,
    /// First seen timestamp
    pub first_seen: Instant,
    /// Last seen timestamp
    pub last_seen: Instant,
    /// Fee rate (sat/vB)
    pub fee_rate: Option<f64>,
    /// Size in bytes
    pub size: Option<u64>,
    /// Whether the transaction is related to our contract
    pub is_related: bool,
}

/// Mempool monitor
#[derive(Debug)]
pub struct MempoolMonitor {
    /// Bitcoin RPC client
    bitcoin_rpc: Arc<BitcoinRpcClient>,
    /// Mempool transactions
    transactions: Arc<Mutex<HashMap<String, MempoolTransaction>>>,
    /// Addresses to monitor
    monitored_addresses: Arc<Mutex<HashSet<String>>>,
    /// Running flag
    running: Arc<Mutex<bool>>,
    /// Monitoring interval
    interval: Duration,
}

impl MempoolMonitor {
    /// Create a new mempool monitor
    pub fn new(bitcoin_rpc: Arc<BitcoinRpcClient>, interval: Duration) -> Self {
        Self {
            bitcoin_rpc,
            transactions: Arc::new(Mutex::new(HashMap::new())),
            monitored_addresses: Arc::new(Mutex::new(HashSet::new())),
            running: Arc::new(Mutex::new(false)),
            interval,
        }
    }
    
    /// Start monitoring
    pub fn start(&self) -> Result<(), ContractError> {
        let mut running = self.running.lock()
            .map_err(|_| ContractError::BitcoinTestnetError("Failed to acquire lock".to_string()))?;
        
        if *running {
            return Ok(());
        }
        
        *running = true;
        
        // Clone Arc references for the thread
        let bitcoin_rpc = self.bitcoin_rpc.clone();
        let transactions = self.transactions.clone();
        let monitored_addresses = self.monitored_addresses.clone();
        let running = self.running.clone();
        let interval = self.interval;
        
        // Spawn monitoring thread
        thread::spawn(move || {
            info!("Mempool monitoring started");
            
            while *running.lock().unwrap() {
                // Get mempool transactions
                match bitcoin_rpc.get_mempool_transactions() {
                    Ok(txids) => {
                        // Update transactions
                        let mut txs = transactions.lock().unwrap();
                        let _addresses = monitored_addresses.lock().unwrap();
                        
                        // Mark all as not seen in this iteration
                        for tx in txs.values_mut() {
                            tx.last_seen = Instant::now();
                        }
                        
                        // Process new transactions
                        for txid in txids {
                            if let Some(tx) = txs.get_mut(&txid) {
                                // Update existing transaction
                                tx.last_seen = Instant::now();
                            } else {
                                // New transaction
                                let now = Instant::now();
                                
                                // Check if related to monitored addresses
                                let is_related = false; // In a real implementation, check transaction outputs
                                
                                txs.insert(txid.clone(), MempoolTransaction {
                                    txid,
                                    first_seen: now,
                                    last_seen: now,
                                    fee_rate: None,
                                    size: None,
                                    is_related,
                                });
                            }
                        }
                        
                        // Remove transactions that haven't been seen for a while
                        txs.retain(|_, tx| tx.last_seen.elapsed() < Duration::from_secs(3600));
                        
                        debug!("Mempool: {} transactions", txs.len());
                    },
                    Err(e) => {
                        error!("Failed to get mempool transactions: {:?}", e);
                    }
                }
                
                // Sleep
                thread::sleep(interval);
            }
            
            info!("Mempool monitoring stopped");
        });
        
        Ok(())
    }
    
    /// Stop monitoring
    pub fn stop(&self) -> Result<(), ContractError> {
        let mut running = self.running.lock()
            .map_err(|_| ContractError::BitcoinTestnetError("Failed to acquire lock".to_string()))?;
        
        *running = false;
        
        Ok(())
    }
    
    /// Add an address to monitor
    pub fn add_monitored_address(&self, address: &str) -> Result<(), ContractError> {
        let mut addresses = self.monitored_addresses.lock()
            .map_err(|_| ContractError::BitcoinTestnetError("Failed to acquire lock".to_string()))?;
        
        addresses.insert(address.to_string());
        
        Ok(())
    }
    
    /// Remove an address from monitoring
    pub fn remove_monitored_address(&self, address: &str) -> Result<(), ContractError> {
        let mut addresses = self.monitored_addresses.lock()
            .map_err(|_| ContractError::BitcoinTestnetError("Failed to acquire lock".to_string()))?;
        
        addresses.remove(address);
        
        Ok(())
    }
    
    /// Get all mempool transactions
    pub fn get_transactions(&self) -> Result<Vec<MempoolTransaction>, ContractError> {
        let txs = self.transactions.lock()
            .map_err(|_| ContractError::BitcoinTestnetError("Failed to acquire lock".to_string()))?;
        
        Ok(txs.values().cloned().collect())
    }
    
    /// Get related mempool transactions
    pub fn get_related_transactions(&self) -> Result<Vec<MempoolTransaction>, ContractError> {
        let txs = self.transactions.lock()
            .map_err(|_| ContractError::BitcoinTestnetError("Failed to acquire lock".to_string()))?;
        
        Ok(txs.values().filter(|tx| tx.is_related).cloned().collect())
    }
    
    /// Check if a transaction is in the mempool
    pub fn is_in_mempool(&self, txid: &str) -> Result<bool, ContractError> {
        let txs = self.transactions.lock()
            .map_err(|_| ContractError::BitcoinTestnetError("Failed to acquire lock".to_string()))?;
        
        Ok(txs.contains_key(txid))
    }
}
