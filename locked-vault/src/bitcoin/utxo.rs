use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::errors::ContractError;

/// Represents a Bitcoin UTXO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Utxo {
    /// Transaction ID
    pub txid: String,
    /// Output index
    pub vout: u32,
    /// Amount in satoshis
    pub amount: u64,
    /// Number of confirmations
    pub confirmations: u32,
    /// Script pubkey
    pub script_pubkey: String,
    /// Address
    pub address: String,
    /// Whether the UTXO is spendable
    pub spendable: bool,
}

impl Utxo {
    /// Get the UTXO reference string (txid:vout)
    pub fn reference(&self) -> String {
        format!("{}:{}", self.txid, self.vout)
    }
    
    /// Estimate the size of the input in a transaction
    pub fn estimate_input_size(&self) -> u64 {
        // P2PKH input size: ~148 bytes
        // P2SH input size: ~variable, but typically larger
        // For simplicity, we'll use a conservative estimate
        180
    }
}

/// A set of UTXOs
#[derive(Debug, Clone, Default)]
pub struct UtxoSet {
    /// UTXOs by reference (txid:vout)
    utxos: HashMap<String, Utxo>,
    /// Total amount in satoshis
    total_amount: u64,
}

impl UtxoSet {
    /// Create a new empty UTXO set
    pub fn new() -> Self {
        Self {
            utxos: HashMap::new(),
            total_amount: 0,
        }
    }
    
    /// Add a UTXO to the set
    pub fn add(&mut self, utxo: Utxo) {
        let reference = utxo.reference();
        
        // Add to total amount if not already in set
        if !self.utxos.contains_key(&reference) {
            self.total_amount += utxo.amount;
        } else {
            // Update total amount if replacing
            if let Some(existing) = self.utxos.get(&reference) {
                self.total_amount -= existing.amount;
                self.total_amount += utxo.amount;
            }
        }
        
        self.utxos.insert(reference, utxo);
    }
    
    /// Remove a UTXO from the set
    pub fn remove(&mut self, reference: &str) -> Option<Utxo> {
        if let Some(utxo) = self.utxos.remove(reference) {
            self.total_amount -= utxo.amount;
            Some(utxo)
        } else {
            None
        }
    }
    
    /// Get a UTXO by reference
    pub fn get(&self, reference: &str) -> Option<&Utxo> {
        self.utxos.get(reference)
    }
    
    /// Get all UTXOs
    pub fn get_all(&self) -> Vec<&Utxo> {
        self.utxos.values().collect()
    }
    
    /// Get the total amount in the set
    pub fn total_amount(&self) -> u64 {
        self.total_amount
    }
    
    /// Get the number of UTXOs in the set
    pub fn len(&self) -> usize {
        self.utxos.len()
    }
    
    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.utxos.is_empty()
    }
    
    /// Select UTXOs for a transaction
    /// Returns (selected_utxos, change_amount)
    pub fn select_utxos(&self, amount: u64, fee_rate: f64) -> Result<(Vec<Utxo>, u64), ContractError> {
        if self.total_amount < amount {
            return Err(ContractError::InsufficientBalance);
        }
        
        // Try coin selection algorithms in order of preference
        
        // 1. Try exact match first (most efficient)
        if let Some(result) = self.select_exact_match(amount, fee_rate) {
            return Ok(result);
        }
        
        // 2. Try single UTXO with change
        if let Some(result) = self.select_single_with_change(amount, fee_rate) {
            return Ok(result);
        }
        
        // 3. Try branch and bound algorithm
        if let Some(result) = self.select_branch_and_bound(amount, fee_rate) {
            return Ok(result);
        }
        
        // 4. Fallback to knapsack algorithm
        self.select_knapsack(amount, fee_rate)
    }
    
    /// Try to find a single UTXO that exactly matches the amount plus fees
    fn select_exact_match(&self, amount: u64, fee_rate: f64) -> Option<(Vec<Utxo>, u64)> {
        for utxo in self.utxos.values() {
            // Estimate fee for a transaction with this single input and two outputs
            // (one for payment, one for change)
            let tx_size = utxo.estimate_input_size() + 70; // 70 bytes for outputs and overhead
            let fee = (tx_size as f64 * fee_rate / 1000.0) as u64;
            
            // Check if this UTXO exactly matches amount + fee
            if utxo.amount == amount + fee {
                return Some((vec![utxo.clone()], 0));
            }
        }
        
        None
    }
    
    /// Try to find a single UTXO that can cover the amount plus fees with change
    fn select_single_with_change(&self, amount: u64, fee_rate: f64) -> Option<(Vec<Utxo>, u64)> {
        for utxo in self.utxos.values() {
            // Estimate fee for a transaction with this single input and two outputs
            let tx_size = utxo.estimate_input_size() + 70; // 70 bytes for outputs and overhead
            let fee = (tx_size as f64 * fee_rate / 1000.0) as u64;
            
            // Check if this UTXO can cover amount + fee
            if utxo.amount > amount + fee {
                let change = utxo.amount - amount - fee;
                return Some((vec![utxo.clone()], change));
            }
        }
        
        None
    }
    
    /// Branch and bound algorithm for coin selection
    fn select_branch_and_bound(&self, amount: u64, fee_rate: f64) -> Option<(Vec<Utxo>, u64)> {
        // Sort UTXOs by value, descending
        let mut sorted_utxos: Vec<&Utxo> = self.utxos.values().collect();
        sorted_utxos.sort_by(|a, b| b.amount.cmp(&a.amount));
        
        // Try to find a subset that minimizes waste
        let target = amount;
        let mut best_selection: Option<Vec<Utxo>> = None;
        let mut best_waste = u64::MAX;
        
        // Helper function for recursive search
        fn search(
            utxos: &[&Utxo],
            target: u64,
            current_sum: u64,
            current_selection: &mut Vec<Utxo>,
            best_selection: &mut Option<Vec<Utxo>>,
            best_waste: &mut u64,
            index: usize,
        ) {
            // If we've reached our target, check if this is better than our best
            if current_sum >= target {
                let waste = current_sum - target;
                if waste < *best_waste {
                    *best_waste = waste;
                    *best_selection = Some(current_selection.clone());
                }
                return;
            }
            
            // If we've gone through all UTXOs, return
            if index >= utxos.len() {
                return;
            }
            
            // Try including this UTXO
            current_selection.push(utxos[index].clone());
            search(
                utxos,
                target,
                current_sum + utxos[index].amount,
                current_selection,
                best_selection,
                best_waste,
                index + 1,
            );
            
            // Try excluding this UTXO
            current_selection.pop();
            search(
                utxos,
                target,
                current_sum,
                current_selection,
                best_selection,
                best_waste,
                index + 1,
            );
        }
        
        // Start recursive search
        let mut current_selection = Vec::new();
        search(
            &sorted_utxos,
            target,
            0,
            &mut current_selection,
            &mut best_selection,
            &mut best_waste,
            0,
        );
        
        // If we found a selection, calculate fees and change
        if let Some(selection) = best_selection {
            // Calculate total input amount
            let total_input = selection.iter().map(|utxo| utxo.amount).sum::<u64>();
            
            // Estimate fee
            let tx_size = selection.iter().map(|utxo| utxo.estimate_input_size()).sum::<u64>() + 70;
            let fee = (tx_size as f64 * fee_rate / 1000.0) as u64;
            
            // Calculate change
            if total_input > amount + fee {
                let change = total_input - amount - fee;
                return Some((selection, change));
            }
        }
        
        None
    }
    
    /// Knapsack algorithm for coin selection (fallback)
    fn select_knapsack(&self, amount: u64, fee_rate: f64) -> Result<(Vec<Utxo>, u64), ContractError> {
        // Sort UTXOs by value, ascending (to minimize the number of inputs)
        let mut sorted_utxos: Vec<&Utxo> = self.utxos.values().collect();
        sorted_utxos.sort_by(|a, b| a.amount.cmp(&b.amount));
        
        let mut selected = Vec::new();
        let mut total_selected = 0;
        
        // Keep adding UTXOs until we have enough
        for utxo in sorted_utxos {
            selected.push(utxo.clone());
            total_selected += utxo.amount;
            
            // Estimate fee
            let tx_size = selected.iter().map(|u| u.estimate_input_size()).sum::<u64>() + 70;
            let fee = (tx_size as f64 * fee_rate / 1000.0) as u64;
            
            // Check if we have enough
            if total_selected >= amount + fee {
                let change = total_selected - amount - fee;
                return Ok((selected, change));
            }
        }
        
        // If we get here, we don't have enough funds
        Err(ContractError::InsufficientBalance)
    }
}
