use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoincore_rpc::bitcoin::{Address, Amount, Transaction, Txid};
use std::str::FromStr;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use log::info;

use crate::bitcoin::testnet::BitcoinTestnetConfig;
use crate::bitcoin::utxo::{Utxo, UtxoSet};
use crate::errors::ContractError;

/// Bitcoin RPC client wrapper
#[derive(Debug, Clone)]
pub struct BitcoinRpcClient {
    /// Inner RPC client
    client: Arc<Client>,
    /// Configuration
    config: BitcoinTestnetConfig,
    /// Last API call timestamp for rate limiting
    last_api_call: Arc<Mutex<Instant>>,
    /// Fee estimates cache
    fee_estimates: Arc<Mutex<HashMap<u16, (f64, Instant)>>>,
}

impl BitcoinRpcClient {
    /// Create a new Bitcoin RPC client
    pub fn new(config: &BitcoinTestnetConfig) -> Result<Self, ContractError> {
        // Create auth from config
        let auth = Auth::UserPass(
            config.rpc_username.clone(),
            config.rpc_password.clone(),
        );
        
        // Create RPC client
        let client = Client::new(&config.rpc_url, auth)
            .map_err(|e| ContractError::BitcoinTestnetError(format!("Failed to create RPC client: {}", e)))?;
        
        // Test connection
        let blockchain_info = client.get_blockchain_info()
            .map_err(|e| ContractError::BitcoinTestnetError(format!("Failed to connect to Bitcoin node: {}", e)))?;
        
        // Verify we're on testnet
        if blockchain_info.chain != "test" {
            return Err(ContractError::BitcoinTestnetError(
                format!("Expected testnet, but connected to {} network", blockchain_info.chain)
            ));
        }
        
        info!("Connected to Bitcoin testnet node");
        
        Ok(Self {
            client: Arc::new(client),
            config: config.clone(),
            last_api_call: Arc::new(Mutex::new(Instant::now())),
            fee_estimates: Arc::new(Mutex::new(HashMap::new())),
        })
    }
    
    /// Make an API call with rate limiting
    fn rate_limit(&self) -> Result<(), ContractError> {
        let mut last_call = self.last_api_call.lock()
            .map_err(|_| ContractError::BitcoinTestnetError("Failed to acquire lock".to_string()))?;
        
        // Check rate limit
        let elapsed = last_call.elapsed();
        let min_interval = Duration::from_secs(60) / self.config.rate_limit as u32;
        
        if elapsed < min_interval {
            // Sleep to respect rate limit
            std::thread::sleep(min_interval - elapsed);
        }
        
        // Update last call timestamp
        *last_call = Instant::now();
        
        Ok(())
    }
    
    /// Get the balance of an address
    pub fn get_address_balance(&self, address: &str) -> Result<u64, ContractError> {
        self.rate_limit()?;
        
        // Convert address string to Address object
        let addr = Address::from_str(address)
            .map_err(|_| ContractError::InvalidAddress)?;
        
        // Get unspent outputs for address
        // In newer versions, we need to use the script_pubkey directly
        let script = addr.payload.script_pubkey();
        
        // Convert to checked address for the API
        let checked_addr = Address::from_script(&script, addr.network.clone())
            .map_err(|_| ContractError::InvalidAddress)?;
        
        let utxos = self.client.list_unspent(None, None, Some(&[&checked_addr]), None, None)
            .map_err(|e| ContractError::BitcoinTestnetError(format!("Failed to get UTXOs: {}", e)))?;
        
        // Sum the values
        let balance: u64 = utxos.iter()
            .map(|utxo| utxo.amount.to_sat())
            .sum();
        
        Ok(balance)
    }
    
    /// Get UTXOs for an address
    pub fn get_address_utxos(&self, address: &str) -> Result<UtxoSet, ContractError> {
        self.rate_limit()?;
        
        // Convert address string to Address object
        let addr = Address::from_str(address)
            .map_err(|_| ContractError::InvalidAddress)?;
        
        // Convert to checked address for the API
        let script = addr.payload.script_pubkey();
        let checked_addr = Address::from_script(&script, addr.network.clone())
            .map_err(|_| ContractError::InvalidAddress)?;
        
        // Get unspent outputs for address
        let utxos = self.client.list_unspent(None, None, Some(&[&checked_addr]), None, None)
            .map_err(|e| ContractError::BitcoinTestnetError(format!("Failed to get UTXOs: {}", e)))?;
        
        // Convert to our UTXO format
        let mut utxo_set = UtxoSet::new();
        
        for utxo in utxos {
            let utxo_entry = Utxo {
                txid: utxo.txid.to_string(),
                vout: utxo.vout,
                amount: utxo.amount.to_sat(),
                confirmations: utxo.confirmations,
                script_pubkey: utxo.script_pub_key.to_string(),
                address: address.to_string(),
                spendable: true,
            };
            
            utxo_set.add(utxo_entry);
        }
        
        Ok(utxo_set)
    }
    
    /// Get estimated fee rate
    pub fn get_fee_estimate(&self, target_blocks: u16) -> Result<f64, ContractError> {
        // Check cache first
        {
            let fee_estimates = self.fee_estimates.lock()
                .map_err(|_| ContractError::BitcoinTestnetError("Failed to acquire lock".to_string()))?;
            
            if let Some((fee, timestamp)) = fee_estimates.get(&target_blocks) {
                // Cache is valid for 10 minutes
                if timestamp.elapsed() < Duration::from_secs(600) {
                    return Ok(*fee);
                }
            }
        }
        
        self.rate_limit()?;
        
        // Get fee estimate from node
        let fee = self.client.estimate_smart_fee(target_blocks as u16, None)
            .map_err(|e| ContractError::BitcoinTestnetError(format!("Failed to estimate fee: {}", e)))?;
        
        let fee_rate = fee.fee_rate
            .ok_or_else(|| ContractError::BitcoinTestnetError("No fee estimate available".to_string()))?;
        
        // Convert to sat/vB - in newer versions we need to use to_sat() and divide
        let fee_rate_sat_vb = fee_rate.to_sat() as f64 / 1000.0;
        
        // Update cache
        {
            let mut fee_estimates = self.fee_estimates.lock()
                .map_err(|_| ContractError::BitcoinTestnetError("Failed to acquire lock".to_string()))?;
            
            fee_estimates.insert(target_blocks, (fee_rate_sat_vb, Instant::now()));
        }
        
        Ok(fee_rate_sat_vb)
    }
    
    /// Create and sign a transaction
    pub fn create_and_sign_transaction(
        &self,
        from_address: &str,
        to_address: &str,
        amount: u64,
        fee_rate: f64,
    ) -> Result<String, ContractError> {
        self.rate_limit()?;
        
        // Convert addresses
        let to_addr = Address::from_str(to_address)
            .map_err(|_| ContractError::InvalidAddress)?;
        
        // Get UTXOs for from_address
        let utxos = self.get_address_utxos(from_address)?;
        
        // Select UTXOs for the transaction
        let (selected_utxos, change) = utxos.select_utxos(amount, fee_rate)?;
        
        if selected_utxos.is_empty() {
            return Err(ContractError::InsufficientBalance);
        }
        
        // Create raw transaction inputs
        let mut inputs = Vec::new();
        for utxo in &selected_utxos {
            let txid = Txid::from_str(&utxo.txid)
                .map_err(|_| ContractError::InvalidBitcoinTransaction)?;
            
            inputs.push(bitcoincore_rpc::json::CreateRawTransactionInput {
                txid,
                vout: utxo.vout,
                sequence: None,
            });
        }
        
        // Create outputs
        let mut outputs = HashMap::new();
        
        // Main output - use debug formatting for address
        outputs.insert(
            format!("{:?}", to_addr),
            Amount::from_sat(amount),
        );
        
        // Change output if needed
        if change > 0 {
            let from_addr = Address::from_str(from_address)
                .map_err(|_| ContractError::InvalidAddress)?;
            
            outputs.insert(
                format!("{:?}", from_addr),
                Amount::from_sat(change),
            );
        }
        
        // Create raw transaction
        let raw_tx = self.client.create_raw_transaction(&inputs, &outputs, None, None)
            .map_err(|e| ContractError::BitcoinTestnetError(format!("Failed to create raw transaction: {}", e)))?;
        
        // Sign transaction
        let signed_tx = self.client.sign_raw_transaction_with_wallet(&raw_tx, None, None)
            .map_err(|e| ContractError::BitcoinTestnetError(format!("Failed to sign transaction: {}", e)))?;
        
        if !signed_tx.complete {
            return Err(ContractError::BitcoinTestnetError("Transaction signing incomplete".to_string()));
        }
        
        // Send transaction
        let txid = self.client.send_raw_transaction(&signed_tx.hex)
            .map_err(|e| ContractError::BitcoinTestnetError(format!("Failed to send transaction: {}", e)))?;
        
        Ok(txid.to_string())
    }
    
    /// Get transaction details
    pub fn get_transaction(&self, txid: &str) -> Result<Transaction, ContractError> {
        self.rate_limit()?;
        
        let tx_id = Txid::from_str(txid)
            .map_err(|_| ContractError::InvalidBitcoinTransaction)?;
        
        let _tx = self.client.get_transaction(&tx_id, None)
            .map_err(|e| ContractError::BitcoinTestnetError(format!("Failed to get transaction: {}", e)))?;
        
        let raw_tx = self.client.get_raw_transaction(&tx_id, None)
            .map_err(|e| ContractError::BitcoinTestnetError(format!("Failed to get raw transaction: {}", e)))?;
        
        Ok(raw_tx)
    }
    
    /// Get mempool transactions
    pub fn get_mempool_transactions(&self) -> Result<Vec<String>, ContractError> {
        self.rate_limit()?;
        
        let txids = self.client.get_raw_mempool()
            .map_err(|e| ContractError::BitcoinTestnetError(format!("Failed to get mempool: {}", e)))?;
        
        Ok(txids.iter().map(|txid| txid.to_string()).collect())
    }
    
    /// Check if transaction is in mempool
    pub fn is_in_mempool(&self, txid: &str) -> Result<bool, ContractError> {
        self.rate_limit()?;
        
        let tx_id = Txid::from_str(txid)
            .map_err(|_| ContractError::InvalidBitcoinTransaction)?;
        
        let mempool = self.client.get_raw_mempool()
            .map_err(|e| ContractError::BitcoinTestnetError(format!("Failed to get mempool: {}", e)))?;
        
        Ok(mempool.contains(&tx_id))
    }
    
    /// Get transaction confirmations
    pub fn get_transaction_confirmations(&self, txid: &str) -> Result<u32, ContractError> {
        self.rate_limit()?;
        
        let tx_id = Txid::from_str(txid)
            .map_err(|_| ContractError::InvalidBitcoinTransaction)?;
        
        let tx = self.client.get_transaction(&tx_id, None)
            .map_err(|e| ContractError::BitcoinTestnetError(format!("Failed to get transaction: {}", e)))?;
        
        Ok(tx.info.confirmations as u32)
    }
    
    /// Create a multi-signature address
    pub fn create_multisig_address(
        &self,
        required_signatures: u8,
        public_keys: &[String],
    ) -> Result<String, ContractError> {
        self.rate_limit()?;
        
        if required_signatures == 0 || required_signatures as usize > public_keys.len() {
            return Err(ContractError::BitcoinTestnetError(
                "Invalid multisig parameters".to_string()
            ));
        }
        
        // In a real implementation, this would use the appropriate RPC call
        // For now, we'll simulate it
        let address = format!("2N{}...{}", required_signatures, public_keys.len());
        
        Ok(address)
    }
    
    /// Verify a signature
    pub fn verify_signature(
        &self,
        _address: &str,
        _message: &str,
        _signature: &str,
    ) -> Result<bool, ContractError> {
        self.rate_limit()?;
        
        // In a real implementation, this would use the appropriate RPC call
        // For now, we'll simulate it
        
        Ok(true)
    }
}
