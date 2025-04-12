use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::models::TokenType;

/// Events emitted by the contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    /// Deposit event
    Deposited {
        /// Deposit ID
        deposit_id: u64,
        /// Depositor address
        depositor_address: String,
        /// Token type
        token_type: TokenType,
        /// Deposit amount
        deposit_amount: u64,
        /// Unlock timestamp
        unlock_timestamp: DateTime<Utc>,
        /// Transaction hash
        transaction_hash: Option<String>,
        /// Block number
        block_number: Option<u64>,
        /// Timestamp
        timestamp: DateTime<Utc>,
    },
    
    /// Withdrawal event
    Withdrawn {
        /// Deposit ID
        deposit_id: u64,
        /// Depositor address
        depositor_address: String,
        /// Token type
        token_type: TokenType,
        /// Withdrawn amount
        withdrawn_amount: u64,
        /// Whether this was an emergency withdrawal
        is_emergency_withdrawal: bool,
        /// Transaction hash
        transaction_hash: Option<String>,
        /// Block number
        block_number: Option<u64>,
        /// Timestamp
        timestamp: DateTime<Utc>,
    },
    
    /// Emergency withdrawal event
    EmergencyWithdrawn {
        /// Deposit ID
        deposit_id: u64,
        /// Depositor address
        depositor_address: String,
        /// Token type
        token_type: TokenType,
        /// Withdrawn amount
        withdrawn_amount: u64,
        /// Fee amount
        fee_amount: u64,
        /// Transaction hash
        transaction_hash: Option<String>,
        /// Block number
        block_number: Option<u64>,
        /// Timestamp
        timestamp: DateTime<Utc>,
    },
    
    /// Fee collection event
    FeeCollected {
        /// Token type
        token_type: TokenType,
        /// Fee amount
        fee_amount: u64,
        /// Collector address
        collector_address: String,
        /// Transaction hash
        transaction_hash: Option<String>,
        /// Timestamp
        timestamp: DateTime<Utc>,
    },
    
    /// Contract paused event
    ContractPaused {
        /// Pauser address
        pauser_address: String,
        /// Timestamp
        timestamp: DateTime<Utc>,
    },
    
    /// Contract unpaused event
    ContractUnpaused {
        /// Unpauser address
        unpauser_address: String,
        /// Timestamp
        timestamp: DateTime<Utc>,
    },
    
    /// Ownership transfer event
    OwnershipTransferred {
        /// Previous owner address
        previous_owner: String,
        /// New owner address
        new_owner: String,
        /// Timestamp
        timestamp: DateTime<Utc>,
    },
    
    /// Token support added event
    TokenSupportAdded {
        /// Token type
        token_type: TokenType,
        /// Timestamp
        timestamp: DateTime<Utc>,
    },
    
    /// Token support removed event
    TokenSupportRemoved {
        /// Token type
        token_type: TokenType,
        /// Timestamp
        timestamp: DateTime<Utc>,
    },
}

impl Event {
    /// Get the event name
    pub fn name(&self) -> &'static str {
        match self {
            Event::Deposited { .. } => "Deposited",
            Event::Withdrawn { .. } => "Withdrawn",
            Event::EmergencyWithdrawn { .. } => "EmergencyWithdrawn",
            Event::FeeCollected { .. } => "FeeCollected",
            Event::ContractPaused { .. } => "ContractPaused",
            Event::ContractUnpaused { .. } => "ContractUnpaused",
            Event::OwnershipTransferred { .. } => "OwnershipTransferred",
            Event::TokenSupportAdded { .. } => "TokenSupportAdded",
            Event::TokenSupportRemoved { .. } => "TokenSupportRemoved",
        }
    }
    
    /// Get the event timestamp
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            Event::Deposited { timestamp, .. } => *timestamp,
            Event::Withdrawn { timestamp, .. } => *timestamp,
            Event::EmergencyWithdrawn { timestamp, .. } => *timestamp,
            Event::FeeCollected { timestamp, .. } => *timestamp,
            Event::ContractPaused { timestamp, .. } => *timestamp,
            Event::ContractUnpaused { timestamp, .. } => *timestamp,
            Event::OwnershipTransferred { timestamp, .. } => *timestamp,
            Event::TokenSupportAdded { timestamp, .. } => *timestamp,
            Event::TokenSupportRemoved { timestamp, .. } => *timestamp,
        }
    }
}