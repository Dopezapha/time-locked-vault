use std::collections::HashMap;
use std::cell::RefCell;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// Represents different types of tokens that can be deposited
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TokenType {
    /// Bitcoin
    Bitcoin,
    /// Ethereum
    Ethereum,
    /// Solana
    Solana,
    /// Rune token with identifier
    Rune(String),
    /// Ordinal inscription with identifier
    Ordinal(String),
    /// Lightning Network payment
    Lightning,
    /// Custom token with identifier
    Custom(String),
}

impl TokenType {
    /// Validate token type parameters
    pub fn validate(&self) -> Result<(), String> {
        match self {
            TokenType::Rune(id) => {
                if id.is_empty() {
                    return Err("Rune identifier cannot be empty".to_string());
                }
                
                // Check for valid characters (alphanumeric and underscore)
                if !id.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    return Err("Rune identifier can only contain alphanumeric characters and underscores".to_string());
                }
                
                // Check for minimum length
                if id.len() < 10 {
                    return Err("Rune identifier must be at least 10 characters long".to_string());
                }
                
                // Check for RUNE_ prefix
                if !id.starts_with("RUNE_") {
                    return Err("Rune identifier must start with 'RUNE_'".to_string());
                }
                
                Ok(())
            },
            TokenType::Ordinal(id) => {
                if id.is_empty() {
                    return Err("Ordinal identifier cannot be empty".to_string());
                }
                
                // Check for valid characters (hexadecimal)
                if !id.chars().all(|c| c.is_ascii_hexdigit()) {
                    return Err("Ordinal identifier must be a valid hexadecimal string".to_string());
                }
                
                // Check for minimum length
                if id.len() < 64 {
                    return Err("Ordinal identifier must be at least 64 characters long".to_string());
                }
                
                Ok(())
            },
            TokenType::Custom(id) => {
                if id.is_empty() {
                    return Err("Custom token identifier cannot be empty".to_string());
                }
                
                Ok(())
            },
            _ => Ok(()),
        }
    }
    
    /// Check if the token type is Bitcoin-based
    pub fn is_bitcoin_based(&self) -> bool {
        match self {
            TokenType::Bitcoin | TokenType::Rune(_) | TokenType::Ordinal(_) | TokenType::Lightning => true,
            _ => false,
        }
    }
    
    /// Get the token type name
    pub fn name(&self) -> String {
        match self {
            TokenType::Bitcoin => "Bitcoin".to_string(),
            TokenType::Ethereum => "Ethereum".to_string(),
            TokenType::Solana => "Solana".to_string(),
            TokenType::Rune(id) => format!("Rune({})", id),
            TokenType::Ordinal(id) => format!("Ordinal({})", id),
            TokenType::Lightning => "Lightning".to_string(),
            TokenType::Custom(id) => format!("Custom({})", id),
        }
    }
}

/// Represents a deposit in the contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deposit {
    /// Unique identifier for the deposit
    pub deposit_id: u64,
    /// Address of the depositor
    pub depositor_address: String,
    /// Type of token deposited
    pub deposited_token_type: TokenType,
    /// Amount of tokens deposited
    pub deposited_amount: u64,
    /// Timestamp when the deposit was made
    pub deposit_timestamp: DateTime<Utc>,
    /// Timestamp when the deposit can be withdrawn
    pub unlock_timestamp: DateTime<Utc>,
    /// Whether the deposit has been withdrawn
    pub is_withdrawn: bool,
    /// Transaction hash of the withdrawal, if any
    pub withdrawal_tx_hash: Option<String>,
    /// Last time the deposit was modified
    pub last_modified: DateTime<Utc>,
    /// UTXO reference for Bitcoin-based tokens
    pub utxo_reference: Option<String>,
    /// Lightning payment hash for Lightning deposits
    pub lightning_payment_hash: Option<String>,
    /// Multisig wallet name for multisig deposits
    pub multisig_wallet: Option<String>,
}

/// Configuration for fees in the contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeConfig {
    /// Percentage fee for emergency withdrawals (0-100)
    pub emergency_withdrawal_fee_percentage: u8,
    /// Address where fees are collected
    pub fee_collector_address: String,
    /// Accumulated fees per token type
    pub collected_fees: HashMap<TokenType, u64>,
}

/// Limits for deposits in the contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositLimits {
    /// Maximum amount per token type
    pub max_deposit_amounts: HashMap<TokenType, u64>,
    /// Maximum number of deposits per user
    pub max_deposits_per_user: Option<u32>,
    /// Maximum total deposits across all users
    pub max_total_deposits: Option<u64>,
}

impl DepositLimits {
    /// Create a new instance with default values
    pub fn default() -> Self {
        Self {
            max_deposit_amounts: HashMap::new(),
            max_deposits_per_user: None,
            max_total_deposits: None,
        }
    }
    
    /// Validate the limits
    pub fn validate(&self) -> Result<(), String> {
        // Check that max_deposits_per_user is reasonable
        if let Some(max_deposits) = self.max_deposits_per_user {
            if max_deposits == 0 {
                return Err("Maximum deposits per user cannot be zero".to_string());
            }
        }
        
        // Check that max_total_deposits is reasonable
        if let Some(max_total) = self.max_total_deposits {
            if max_total == 0 {
                return Err("Maximum total deposits cannot be zero".to_string());
            }
        }
        
        // Check that max_deposit_amounts are reasonable
        for (token_type, &amount) in &self.max_deposit_amounts {
            if amount == 0 {
                return Err(format!("Maximum deposit amount for {:?} cannot be zero", token_type));
            }
        }
        
        Ok(())
    }
}

/// Trait for token transfer operations
pub trait TokenTransfer {
    /// Transfer tokens from an address to the contract
    fn transfer_to_contract(&self, from_address: &str, token_type: &TokenType, amount: u64) -> Result<(), String>;
    
    /// Transfer tokens from the contract to an address
    fn transfer_from_contract(&self, to_address: &str, token_type: &TokenType, amount: u64) -> Result<(), String>;
    
    /// Get the balance of an address for a token type
    fn get_balance(&self, address: &str, token_type: &TokenType) -> Result<u64, String>;
    
    /// Validate an address
    fn validate_address(&self, address: &str) -> Result<(), String> {
        if address.is_empty() {
            return Err("Empty address".to_string());
        }
        Ok(())
    }
    
    /// Check if the implementation supports a token type
    fn supports_token_type(&self, token_type: &TokenType) -> bool;
    
    /// Get the network type (e.g., "testnet", "mainnet")
    fn get_network_type(&self) -> String;
}

/// Reentrancy guard to prevent reentrancy attacks
#[derive(Debug)]
pub struct ReentrancyGuard {
    entered: RefCell<bool>,
}

impl ReentrancyGuard {
    /// Create a new reentrancy guard
    pub fn new() -> Self {
        Self {
            entered: RefCell::new(false),
        }
    }
    
    /// Enter a guarded section
    pub fn enter(&self) -> Result<ReentrancyGuardEntered, String> {
        let mut entered = self.entered.borrow_mut();
        if *entered {
            return Err("Reentrancy detected".to_string());
        }
        
        *entered = true;
        Ok(ReentrancyGuardEntered { guard: self })
    }
    
    /// Exit a guarded section (called by ReentrancyGuardEntered's Drop implementation)
    fn exit(&self) {
        let mut entered = self.entered.borrow_mut();
        *entered = false;
    }
}

/// RAII guard for reentrancy protection
pub struct ReentrancyGuardEntered<'a> {
    guard: &'a ReentrancyGuard,
}

impl<'a> Drop for ReentrancyGuardEntered<'a> {
    fn drop(&mut self) {
        self.guard.exit();
    }
}