#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

//! Time-Locked Deposit Contract for Bitcoin Testnet
//! 
//! This library provides a time-locked deposit contract implementation
//! specifically designed for Bitcoin testnet. It supports Bitcoin, Rune tokens,
//! Ordinals, and Lightning Network payments, with secure deposit and withdrawal mechanisms.
//! 
//! # Features
//! 
//! - Bitcoin testnet support with real RPC integration
//! - Rune token support
//! - Ordinals support
//! - Lightning Network support
//! - Multi-signature wallet support
//! - Time-locked deposits
//! - Emergency withdrawals with fee
//! - Batch transaction processing
//! - UTXO management
//! - Mempool monitoring
//! - Dynamic fee estimation
//! - Signature verification
//! - Rate limiting for API calls
//! - Secure address validation
//! 
//! # Usage
//! 
//! ```no_run
//! use time_locked_deposit::bitcoin::testnet::BitcoinTestnetConfig;
//! use time_locked_deposit::bitcoin::transfer::BitcoinTestnetTransfer;
//! use time_locked_deposit::contract::contract_core::TimeLockedDeposit;
//! use time_locked_deposit::models::TokenType;
//! 
//! // Create Bitcoin testnet configuration
//! let config = BitcoinTestnetConfig::new(
//!     "http://localhost:18332".to_string(),
//!     "testuser".to_string(),
//!     "testpassword".to_string(),
//!     "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
//! );
//! 
//! // Create Bitcoin testnet transfer implementation
//! let transfer = BitcoinTestnetTransfer::new_with_clients(
//!     config,
//!     Some("http://localhost:9735".to_string()),
//!     Some("http://localhost:3000".to_string()),
//! ).unwrap();
//! 
//! // Create contract instance
//! let mut contract = TimeLockedDeposit::new(
//!     "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
//!     10, // 10% emergency withdrawal fee
//!     transfer,
//! ).unwrap();
//! 
//! // Deposit Bitcoin
//! let result = contract.deposit(
//!     "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
//!     TokenType::Bitcoin,
//!     100000, // 0.001 BTC in satoshis
//!     30, // 30 days lock period
//!     Some("txid:0".to_string()), // UTXO reference
//! );
//! ```

pub mod models;
pub mod errors;
pub mod events;
pub mod contract;
pub mod bitcoin;

// Re-export commonly used types
pub use models::{TokenType, TokenTransfer, Deposit};
pub use errors::ContractError;
pub use events::Event;
pub use contract::contract_core::TimeLockedDeposit;
pub use bitcoin::testnet::BitcoinTestnetConfig;
pub use bitcoin::transfer::BitcoinTestnetTransfer;
pub use bitcoin::rpc::BitcoinRpcClient;
pub use bitcoin::utxo::{Utxo, UtxoSet};
pub use bitcoin::lightning::LightningClient;
pub use bitcoin::ordinals::OrdinalsClient;
pub use bitcoin::mempool::MempoolMonitor;
pub use bitcoin::multisig::MultisigClient;
pub use bitcoin::signature::SignatureVerifier;

// Include the tests module
#[cfg(test)]
mod tests;