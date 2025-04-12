//! Bitcoin-related functionality for the time-locked deposit contract
//! 
//! This module contains all Bitcoin-specific implementations, including
//! testnet support, RPC client, UTXO management, Lightning Network,
//! Ordinals, multi-signature, mempool monitoring, and signature verification.

// Re-export submodules
pub mod testnet;
pub mod rpc;
pub mod utxo;
pub mod lightning;
pub mod ordinals;
pub mod multisig;
pub mod mempool;
pub mod signature;
pub mod transfer;

// Re-export commonly used types
pub use testnet::BitcoinTestnetConfig;
pub use rpc::BitcoinRpcClient;
pub use utxo::{Utxo, UtxoSet};
pub use lightning::LightningClient;
pub use ordinals::OrdinalsClient;
pub use mempool::MempoolMonitor;
pub use multisig::MultisigClient;
pub use signature::SignatureVerifier;
pub use transfer::BitcoinTestnetTransfer;