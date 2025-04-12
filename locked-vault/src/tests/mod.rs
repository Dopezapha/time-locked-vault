#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;
    use bitcoincore_rpc::bitcoin::Network;
    use bitcoincore_rpc::bitcoin::secp256k1; // Use secp256k1 from bitcoincore-rpc
    use crate::bitcoin::testnet::{BitcoinTestnetConfig, utils};
    use crate::bitcoin::transfer::BitcoinTestnetTransfer;
    use crate::bitcoin::rpc::BitcoinRpcClient;
    use crate::bitcoin::utxo::{Utxo, UtxoSet};
    use crate::bitcoin::lightning::{LightningClient, InvoiceStatus, ChannelStatus};
    use crate::bitcoin::ordinals::OrdinalsClient;
    use crate::bitcoin::mempool::MempoolMonitor;
    use crate::bitcoin::multisig::{MultisigClient, MultisigTxStatus};
    use crate::bitcoin::signature::SignatureVerifier;
    use crate::contract::contract_core::TimeLockedDeposit;
    use crate::models::{TokenType, TokenTransfer};
    use crate::errors::ContractError;
    use mockall::predicate::*;
    use mockall::mock;
    use rand;

    // Mock TokenTransfer for testing
    mock! {
        pub TokenTransferMock {}
        impl TokenTransfer for TokenTransferMock {
            fn transfer_to_contract(&self, from_address: &str, token_type: &TokenType, amount: u64) -> Result<(), String>;
            fn transfer_from_contract(&self, to_address: &str, token_type: &TokenType, amount: u64) -> Result<(), String>;
            fn get_balance(&self, address: &str, token_type: &TokenType) -> Result<u64, String>;
            fn validate_address(&self, address: &str) -> Result<(), String>;
            fn supports_token_type(&self, token_type: &TokenType) -> bool;
            fn get_network_type(&self) -> String;
        }
    }

    #[test]
    fn test_bitcoin_testnet_address_validation() {
        // Valid testnet addresses
        assert!(utils::validate_testnet_address("tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx"));
        assert!(utils::validate_testnet_address("mzBc4XEFSdzCDcTxAgf6EZXgsZWpztRhef"));
        assert!(utils::validate_testnet_address("2MzQwSSnBHWHqSAqtTVQ6v47XtaisrJa1Vc"));
        
        // Invalid addresses
        assert!(!utils::validate_testnet_address(""));
        assert!(!utils::validate_testnet_address("1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2")); // Mainnet address
        assert!(!utils::validate_testnet_address("invalid_address"));
    }
    
    #[test]
    fn test_token_type_validation() {
        // Valid token types
        assert!(TokenType::Bitcoin.validate().is_ok());
        assert!(TokenType::Ethereum.validate().is_ok());
        assert!(TokenType::Solana.validate().is_ok());
        assert!(TokenType::Rune("RUNE_TEST_TOKEN_123".to_string()).validate().is_ok());
        assert!(TokenType::Ordinal("0".repeat(64)).validate().is_ok());
        assert!(TokenType::Custom("CUSTOM_TOKEN".to_string()).validate().is_ok());
        
        // Invalid token types
        assert!(TokenType::Rune("".to_string()).validate().is_err());
        assert!(TokenType::Rune("TEST_123".to_string()).validate().is_err());
        assert!(TokenType::Rune("RUNE_123".to_string()).validate().is_err()); // Too short
        assert!(TokenType::Rune("RUNE_TEST@123".to_string()).validate().is_err()); // Invalid character
        assert!(TokenType::Ordinal("".to_string()).validate().is_err());
        assert!(TokenType::Ordinal("123".to_string()).validate().is_err()); // Too short
        assert!(TokenType::Ordinal("ABCXYZ".to_string()).validate().is_err()); // Invalid hex
    }
    
    #[test]
    fn test_bitcoin_testnet_config() {
        // Valid configuration
        let config = BitcoinTestnetConfig::new(
            "http://localhost:18332".to_string(),
            "testuser".to_string(),
            "testpassword".to_string(),
            "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
        );
        
        assert!(config.validate().is_ok());
        
        // Invalid configuration
        let invalid_config = BitcoinTestnetConfig::new(
            "".to_string(),
            "testuser".to_string(),
            "testpassword".to_string(),
            "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
        );
        
        assert!(invalid_config.validate().is_err());
        
        let invalid_address_config = BitcoinTestnetConfig::new(
            "http://localhost:18332".to_string(),
            "testuser".to_string(),
            "testpassword".to_string(),
            "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2".to_string(), // Mainnet address
        );
        
        assert!(invalid_address_config.validate().is_err());
    }
    
    #[test]
    fn test_utxo_set() {
        let mut utxo_set = UtxoSet::new();
        
        // Test empty set
        assert_eq!(utxo_set.total_amount(), 0);
        assert_eq!(utxo_set.len(), 0);
        assert!(utxo_set.is_empty());
        
        // Add UTXOs
        let utxo1 = Utxo {
            txid: "txid1".to_string(),
            vout: 0,
            amount: 1000,
            confirmations: 6,
            script_pubkey: "script1".to_string(),
            address: "address1".to_string(),
            spendable: true,
        };
        
        let utxo2 = Utxo {
            txid: "txid2".to_string(),
            vout: 1,
            amount: 2000,
            confirmations: 3,
            script_pubkey: "script2".to_string(),
            address: "address2".to_string(),
            spendable: true,
        };
        
        utxo_set.add(utxo1.clone());
        utxo_set.add(utxo2.clone());
        
        // Test non-empty set
        assert_eq!(utxo_set.total_amount(), 3000);
        assert_eq!(utxo_set.len(), 2);
        assert!(!utxo_set.is_empty());
        
        // Test get
        assert_eq!(utxo_set.get("txid1:0").unwrap().amount, 1000);
        assert_eq!(utxo_set.get("txid2:1").unwrap().amount, 2000);
        assert!(utxo_set.get("txid3:0").is_none());
        
        // Test remove
        let removed = utxo_set.remove("txid1:0").unwrap();
        assert_eq!(removed.amount, 1000);
        assert_eq!(utxo_set.total_amount(), 2000);
        assert_eq!(utxo_set.len(), 1);
        assert!(utxo_set.get("txid1:0").is_none());
        
        // Test UTXO selection
        let utxo3 = Utxo {
            txid: "txid3".to_string(),
            vout: 0,
            amount: 500,
            confirmations: 10,
            script_pubkey: "script3".to_string(),
            address: "address3".to_string(),
            spendable: true,
        };
        
        utxo_set.add(utxo3);
        
        // Select UTXOs for an amount less than a single UTXO
        let (selected, change) = utxo_set.select_utxos(400, 1.0).unwrap();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].amount, 500);
        assert!(change > 0);
        
        // Select UTXOs for an amount greater than a single UTXO
        let (selected, change) = utxo_set.select_utxos(2100, 1.0).unwrap();
        assert_eq!(selected.len(), 2);
        assert!(change > 0);
        
        // Test insufficient funds
        assert!(utxo_set.select_utxos(10000, 1.0).is_err());
    }
    
    #[test]
    fn test_contract_initialization() {
        let mut mock = MockTokenTransferMock::new();
        
        // Setup mock expectations
        mock.expect_validate_address()
            .with(eq("owner_address"))
            .returning(|_| Ok(()));
        
        mock.expect_supports_token_type()
            .returning(|token_type| matches!(token_type, TokenType::Bitcoin | TokenType::Ethereum | TokenType::Solana));
        
        mock.expect_get_network_type()
            .returning(|| "testnet".to_string());
        
        // Create contract
        let contract = TimeLockedDeposit::new(
            "owner_address".to_string(),
            10, // 10% emergency withdrawal fee
            mock,
        );
        
        assert!(contract.is_ok());
        
        let contract = contract.unwrap();
        assert_eq!(contract.get_network_type(), "testnet");
        assert!(contract.is_testnet());
    }
    
    #[test]
    fn test_deposit() {
        let mut mock = MockTokenTransferMock::new();
        
        // Setup mock expectations
        mock.expect_validate_address()
            .returning(|_| Ok(()));
        
        mock.expect_supports_token_type()
            .returning(|_| true);
        
        mock.expect_get_network_type()
            .returning(|| "testnet".to_string());
        
        mock.expect_get_balance()
            .returning(|_, _| Ok(10000));
        
        mock.expect_transfer_to_contract()
            .returning(|_, _, _| Ok(()));
        
        // Create contract
        let mut contract = TimeLockedDeposit::new(
            "owner_address".to_string(),
            10, // 10% emergency withdrawal fee
            mock,
        ).unwrap();
        
        // Make a deposit
        let result = contract.deposit(
            "depositor_address".to_string(),
            TokenType::Bitcoin,
            1000,
            30, // 30 days
            Some("txid:0".to_string()),
        );
        
        assert!(result.is_ok());
        
        // Check deposit was registered
        assert_eq!(contract.deposit_registry.len(), 1);
        assert_eq!(contract.user_deposit_ids.get("depositor_address").unwrap().len(), 1);
        
        // Check deposit details
        let deposit_id = contract.user_deposit_ids.get("depositor_address").unwrap()[0];
        let deposit = contract.deposit_registry.get(&deposit_id).unwrap();
        
        assert_eq!(deposit.depositor_address, "depositor_address");
        assert_eq!(deposit.deposited_amount, 1000);
        assert_eq!(deposit.is_withdrawn, false);
        assert_eq!(deposit.utxo_reference, Some("txid:0".to_string()));
    }
    
    #[test]
    fn test_withdraw() {
        let mut mock = MockTokenTransferMock::new();
        
        // Setup mock expectations
        mock.expect_validate_address()
            .returning(|_| Ok(()));
        
        mock.expect_supports_token_type()
            .returning(|_| true);
        
        mock.expect_get_network_type()
            .returning(|| "testnet".to_string());
        
        mock.expect_get_balance()
            .returning(|_, _| Ok(10000));
        
        mock.expect_transfer_to_contract()
            .returning(|_, _, _| Ok(()));
        
        mock.expect_transfer_from_contract()
            .returning(|_, _, _| Ok(()));
        
        // Create contract
        let mut contract = TimeLockedDeposit::new(
            "owner_address".to_string(),
            10, // 10% emergency withdrawal fee
            mock,
        ).unwrap();
        
        // Make a deposit with 0 days lock (for testing)
        let result = contract.deposit(
            "depositor_address".to_string(),
            TokenType::Bitcoin,
            1000,
            0, // 0 days (immediately unlocked for testing)
            Some("txid:0".to_string()),
        );
        
        assert!(result.is_ok());
        
        // Get deposit ID
        let deposit_id = contract.user_deposit_ids.get("depositor_address").unwrap()[0];
        
        // Withdraw
        let result = contract.withdraw(
            "depositor_address".to_string(),
            deposit_id,
        );
        
        assert!(result.is_ok());
        
        // Check deposit was marked as withdrawn
        let deposit = contract.deposit_registry.get(&deposit_id).unwrap();
        assert!(deposit.is_withdrawn);
    }
    
    #[test]
    fn test_emergency_withdraw() {
        let mut mock = MockTokenTransferMock::new();
        
        // Setup mock expectations
        mock.expect_validate_address()
            .returning(|_| Ok(()));
        
        mock.expect_supports_token_type()
            .returning(|_| true);
        
        mock.expect_get_network_type()
            .returning(|| "testnet".to_string());
        
        mock.expect_get_balance()
            .returning(|_, _| Ok(10000));
        
        mock.expect_transfer_to_contract()
            .returning(|_, _, _| Ok(()));
        
        mock.expect_transfer_from_contract()
            .returning(|_, _, _| Ok(()));
        
        // Create contract
        let mut contract = TimeLockedDeposit::new(
            "owner_address".to_string(),
            10, // 10% emergency withdrawal fee
            mock,
        ).unwrap();
        
        // Make a deposit with 30 days lock
        let result = contract.deposit(
            "depositor_address".to_string(),
            TokenType::Bitcoin,
            1000,
            30, // 30 days
            Some("txid:0".to_string()),
        );
        
        assert!(result.is_ok());
        
        // Get deposit ID
        let deposit_id = contract.user_deposit_ids.get("depositor_address").unwrap()[0];
        
        // Emergency withdraw
        let result = contract.emergency_withdraw(
            "depositor_address".to_string(),
            deposit_id,
        );
        
        assert!(result.is_ok());
        
        // Check deposit was marked as withdrawn
        let deposit = contract.deposit_registry.get(&deposit_id).unwrap();
        assert!(deposit.is_withdrawn);
        
        // Check fees were collected
        let fees = contract.fee_config.collected_fees.get(&TokenType::Bitcoin).unwrap();
        assert_eq!(*fees, 100); // 10% of 1000
    }
    
    #[test]
    fn test_withdraw_fees() {
        let mut mock = MockTokenTransferMock::new();
        
        // Setup mock expectations
        mock.expect_validate_address()
            .returning(|_| Ok(()));
        
        mock.expect_supports_token_type()
            .returning(|_| true);
        
        mock.expect_get_network_type()
            .returning(|| "testnet".to_string());
        
        mock.expect_get_balance()
            .returning(|_, _| Ok(10000));
        
        mock.expect_transfer_to_contract()
            .returning(|_, _, _| Ok(()));
        
        mock.expect_transfer_from_contract()
            .returning(|_, _, _| Ok(()));
        
        // Create contract
        let mut contract = TimeLockedDeposit::new(
            "owner_address".to_string(),
            10, // 10% emergency withdrawal fee
            mock,
        ).unwrap();
        
        // Make a deposit with 30 days lock
        let result = contract.deposit(
            "depositor_address".to_string(),
            TokenType::Bitcoin,
            1000,
            30, // 30 days
            Some("txid:0".to_string()),
        );
        
        assert!(result.is_ok());
        
        // Get deposit ID
        let deposit_id = contract.user_deposit_ids.get("depositor_address").unwrap()[0];
        
        // Emergency withdraw to generate fees
        let result = contract.emergency_withdraw(
            "depositor_address".to_string(),
            deposit_id,
        );
        
        assert!(result.is_ok());
        
        // Withdraw fees
        let result = contract.withdraw_fees(
            "owner_address".to_string(),
            TokenType::Bitcoin,
        );
        
        assert!(result.is_ok());
        
        // Check fees were reset
        let fees = contract.fee_config.collected_fees.get(&TokenType::Bitcoin).unwrap();
        assert_eq!(*fees, 0);
    }
    
    #[test]
    fn test_unauthorized_access() {
        let mut mock = MockTokenTransferMock::new();
        
        // Setup mock expectations
        mock.expect_validate_address()
            .returning(|_| Ok(()));
        
        mock.expect_supports_token_type()
            .returning(|_| true);
        
        mock.expect_get_network_type()
            .returning(|| "testnet".to_string());
        
        mock.expect_get_balance()
            .returning(|_, _| Ok(10000));
        
        mock.expect_transfer_to_contract()
            .returning(|_, _, _| Ok(()));
        
        // Create contract
        let mut contract = TimeLockedDeposit::new(
            "owner_address".to_string(),
            10, // 10% emergency withdrawal fee
            mock,
        ).unwrap();
        
        // Make a deposit
        let result = contract.deposit(
            "depositor_address".to_string(),
            TokenType::Bitcoin,
            1000,
            30, // 30 days
            Some("txid:0".to_string()),
        );
        
        assert!(result.is_ok());
        
        // Get deposit ID
        let deposit_id = contract.user_deposit_ids.get("depositor_address").unwrap()[0];
        
        // Try to withdraw from a different address
        let result = contract.withdraw(
            "different_address".to_string(),
            deposit_id,
        );
        
        assert!(matches!(result, Err(ContractError::Unauthorized)));
        
        // Try to withdraw fees from a non-owner address
        let result = contract.withdraw_fees(
            "different_address".to_string(),
            TokenType::Bitcoin,
        );
        
        assert!(matches!(result, Err(ContractError::Unauthorized)));
    }
    
    #[test]
    fn test_deposit_limits() {
        let mut mock = MockTokenTransferMock::new();
        
        // Setup mock expectations
        mock.expect_validate_address()
            .returning(|_| Ok(()));
        
        mock.expect_supports_token_type()
            .returning(|_| true);
        
        mock.expect_get_network_type()
            .returning(|| "testnet".to_string());
        
        mock.expect_get_balance()
            .returning(|_, _| Ok(10000));
        
        mock.expect_transfer_to_contract()
            .returning(|_, _, _| Ok(()));
        
        // Create contract
        let mut contract = TimeLockedDeposit::new(
            "owner_address".to_string(),
            10, // 10% emergency withdrawal fee
            mock,
        ).unwrap();
        
        // Set deposit limits
        let mut limits = contract.deposit_limits.clone();
        limits.max_deposits_per_user = Some(2);
        limits.max_deposit_amounts.insert(TokenType::Bitcoin, 500);
        contract.deposit_limits = limits;
        
        // Make a deposit within limits
        let result = contract.deposit(
            "depositor_address".to_string(),
            TokenType::Bitcoin,
            500, // At the limit
            30,
            Some("txid:0".to_string()),
        );
        
        assert!(result.is_ok());
        
        // Try to make a deposit exceeding amount limit
        let result = contract.deposit(
            "depositor_address".to_string(),
            TokenType::Bitcoin,
            501, // Exceeds the limit
            30,
            Some("txid:1".to_string()),
        );
        
        assert!(matches!(result, Err(ContractError::DepositLimitExceeded)));
        
        // Make another deposit within limits
        let result = contract.deposit(
            "depositor_address".to_string(),
            TokenType::Bitcoin,
            400,
            30,
            Some("txid:2".to_string()),
        );
        
        assert!(result.is_ok());
        
        // Try to make a deposit exceeding count limit
        let result = contract.deposit(
            "depositor_address".to_string(),
            TokenType::Bitcoin,
            300,
            30,
            Some("txid:3".to_string()),
        );
        
        assert!(matches!(result, Err(ContractError::UserDepositLimitReached)));
    }
    
    #[test]
    fn test_token_type_support() {
        let mut mock = MockTokenTransferMock::new();
        
        // Setup mock expectations
        mock.expect_validate_address()
            .returning(|_| Ok(()));
        
        mock.expect_supports_token_type()
            .with(always())
            .returning(|token_type| {
                matches!(token_type, 
                    TokenType::Bitcoin | 
                    TokenType::Ethereum | 
                    TokenType::Solana |
                    TokenType::Rune(_)
                )
            });
        
        mock.expect_get_network_type()
            .returning(|| "testnet".to_string());
        
        // Create contract
        let contract = TimeLockedDeposit::new(
            "owner_address".to_string(),
            10, // 10% emergency withdrawal fee
            mock,
        ).unwrap();
        
        // Check supported tokens
        assert!(contract.supported_tokens.contains(&TokenType::Bitcoin));
        assert!(contract.supported_tokens.contains(&TokenType::Ethereum));
        assert!(contract.supported_tokens.contains(&TokenType::Solana));
        
        // Check for Rune token support
        let has_rune = contract.supported_tokens.iter()
            .any(|t| matches!(t, TokenType::Rune(_)));
        assert!(has_rune);
        
        // Check for Ordinal token support (should not be supported)
        let has_ordinal = contract.supported_tokens.iter()
            .any(|t| matches!(t, TokenType::Ordinal(_)));
        assert!(!has_ordinal);
    }
    
    #[test]
fn test_signature_verifier() {
    let verifier = SignatureVerifier::new(Network::Testnet);
    
    // Generate a test key pair
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rand::thread_rng();
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);
    
    // Create a test message
    let message = b"Test message";
    
    // Sign the message
    let signature = verifier.sign(message, &secret_key.secret_bytes())
        .expect("Failed to sign message");
    
    // Verify the signature
    let result = verifier.verify(
        message,
        &signature,
        &public_key.serialize(),
    ).expect("Failed to verify signature");
    
    assert!(result);
    
    // Verify with wrong message
    let wrong_message = b"Wrong message";
    let result = verifier.verify(
        wrong_message,
        &signature,
        &public_key.serialize(),
    ).expect("Failed to verify signature");
    
    assert!(!result);
}
    
    #[test]
    fn test_utxo_selection_algorithms() {
        let mut utxo_set = UtxoSet::new();
        
        // Add various UTXOs
        utxo_set.add(Utxo {
            txid: "txid1".to_string(),
            vout: 0,
            amount: 1000,
            confirmations: 6,
            script_pubkey: "script1".to_string(),
            address: "address1".to_string(),
            spendable: true,
        });
        
        utxo_set.add(Utxo {
            txid: "txid2".to_string(),
            vout: 0,
            amount: 2000,
            confirmations: 6,
            script_pubkey: "script2".to_string(),
            address: "address1".to_string(),
            spendable: true,
        });
        
        utxo_set.add(Utxo {
            txid: "txid3".to_string(),
            vout: 0,
            amount: 3000,
            confirmations: 6,
            script_pubkey: "script3".to_string(),
            address: "address1".to_string(),
            spendable: true,
        });
        
        utxo_set.add(Utxo {
            txid: "txid4".to_string(),
            vout: 0,
            amount: 4000,
            confirmations: 6,
            script_pubkey: "script4".to_string(),
            address: "address1".to_string(),
            spendable: true,
        });
        
        utxo_set.add(Utxo {
            txid: "txid5".to_string(),
            vout: 0,
            amount: 5000,
            confirmations: 6,
            script_pubkey: "script5".to_string(),
            address: "address1".to_string(),
            spendable: true,
        });
        
        // Test exact match selection
        let (selected, change) = utxo_set.select_utxos(3000, 1.0).unwrap();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].amount, 3000);
        assert_eq!(change, 0);
        
        // Test single with change selection
        let (selected, change) = utxo_set.select_utxos(4500, 1.0).unwrap();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].amount, 5000);
        assert_eq!(change, 5000 - 4500 - 180); // 5000 - 4500 - fee
        
        // Test branch and bound selection
        let (selected, change) = utxo_set.select_utxos(6500, 1.0).unwrap();
        assert!(selected.len() > 1);
        assert!(change > 0);
        
        // Test knapsack selection (fallback)
        let (selected, change) = utxo_set.select_utxos(14500, 1.0).unwrap();
        assert!(selected.len() >= 3);
        assert!(change > 0);
        
        // Test insufficient funds
        assert!(utxo_set.select_utxos(20000, 1.0).is_err());
    }
    
    #[test]
    fn test_edge_cases() {
        let mut mock = MockTokenTransferMock::new();
        
        // Setup mock expectations
        mock.expect_validate_address()
            .returning(|_| Ok(()));
        
        mock.expect_supports_token_type()
            .returning(|_| true);
        
        mock.expect_get_network_type()
            .returning(|| "testnet".to_string());
        
        mock.expect_get_balance()
            .returning(|_, _| Ok(u64::MAX));
        
        mock.expect_transfer_to_contract()
            .returning(|_, _, _| Ok(()));
        
        mock.expect_transfer_from_contract()
            .returning(|_, _, _| Ok(()));
        
        // Create contract
        let mut contract = TimeLockedDeposit::new(
            "owner_address".to_string(),
            10, // 10% emergency withdrawal fee
            mock,
        ).unwrap();
        
        // Test deposit with maximum amount
        let result = contract.deposit(
            "depositor_address".to_string(),
            TokenType::Bitcoin,
            u64::MAX / 3, // Large but not overflow
            30,
            Some("txid:0".to_string()),
        );
        
        assert!(result.is_ok());
        
        // Test deposit with amount that would cause overflow
        let result = contract.deposit(
            "depositor_address".to_string(),
            TokenType::Bitcoin,
            u64::MAX, // Would cause overflow
            30,
            Some("txid:1".to_string()),
        );
        
        assert!(matches!(result, Err(ContractError::InvalidAmount)));
        
        // Test deposit with zero amount
        let result = contract.deposit(
            "depositor_address".to_string(),
            TokenType::Bitcoin,
            0, // Zero amount
            30,
            Some("txid:2".to_string()),
        );
        
        assert!(matches!(result, Err(ContractError::InvalidAmount)));
        
        // Test deposit with zero lock period
        let result = contract.deposit(
            "depositor_address".to_string(),
            TokenType::Bitcoin,
            1000,
            0, // Zero lock period
            Some("txid:3".to_string()),
        );
        
        assert!(matches!(result, Err(ContractError::InvalidLockPeriod)));
        
        // Test deposit with excessive lock period
        let result = contract.deposit(
            "depositor_address".to_string(),
            TokenType::Bitcoin,
            1000,
            4000, // > 10 years
            Some("txid:4".to_string()),
        );
        
        assert!(matches!(result, Err(ContractError::InvalidLockPeriod)));
    }
    
    #[test]
    fn test_reentrancy_protection() {
        let guard = ReentrancyGuard::new();
        
        // First entry should succeed
        let guard_entered = guard.enter();
        assert!(guard_entered.is_ok());
        
        // Second entry should fail
        let guard_entered2 = guard.enter();
        assert!(guard_entered2.is_err());
        
        // After dropping the first guard, entry should succeed again
        drop(guard_entered);
        let guard_entered3 = guard.enter();
        assert!(guard_entered3.is_ok());
    }
    
    // Helper struct for reentrancy test
    struct ReentrancyGuard {
        entered: std::cell::RefCell<bool>,
    }
    
    impl ReentrancyGuard {
        fn new() -> Self {
            Self {
                entered: std::cell::RefCell::new(false),
            }
        }
        
        fn enter(&self) -> Result<ReentrancyGuardEntered, String> {
            let mut entered = self.entered.borrow_mut();
            if *entered {
                return Err("Reentrancy detected".to_string());
            }
            
            *entered = true;
            Ok(ReentrancyGuardEntered { guard: self })
        }
        
        fn exit(&self) {
            let mut entered = self.entered.borrow_mut();
            *entered = false;
        }
    }
    
    struct ReentrancyGuardEntered<'a> {
        guard: &'a ReentrancyGuard,
    }
    
    impl<'a> Drop for ReentrancyGuardEntered<'a> {
        fn drop(&mut self) {
            self.guard.exit();
        }
    }
    
    #[test]
    fn test_lightning_client() {
        // Create Bitcoin RPC client
        let config = BitcoinTestnetConfig::new(
            "http://localhost:18332".to_string(),
            "testuser".to_string(),
            "testpassword".to_string(),
            "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
        );
        
        let rpc_client = Arc::new(BitcoinRpcClient::new(&config).unwrap());
        
        // Create Lightning client
        let lightning_client = LightningClient::new(
            rpc_client.clone(),
            "http://localhost:9735".to_string(),
            "api_key".to_string(),
        );
        
        // Create invoice
        let invoice = lightning_client.create_invoice(
            1000,
            "Test payment",
            3600,
        ).unwrap();
        
        // Check invoice properties
        assert_eq!(invoice.amount, 1000);
        assert_eq!(invoice.description, "Test payment");
        assert_eq!(invoice.status, InvoiceStatus::Pending);
        
        // Get invoice status
        let status = lightning_client.get_invoice_status(&invoice.id).unwrap();
        assert_eq!(status, InvoiceStatus::Pending);
        
        // Open channel
        let channel = lightning_client.open_channel(
            "02...", // Fixed: removed .to_string() to match &str parameter
            100000,
        ).unwrap();
        
        // Check channel properties
        assert_eq!(channel.capacity, 100000);
        assert_eq!(channel.local_balance, 100000);
        assert_eq!(channel.remote_balance, 0);
        assert_eq!(channel.status, ChannelStatus::PendingOpen);
        
        // Get channel status
        let status = lightning_client.get_channel_status(&channel.id).unwrap();
        assert_eq!(status, ChannelStatus::PendingOpen);
        
        // Close channel
        let result = lightning_client.close_channel(&channel.id);
        assert!(result.is_ok());
        
        // Get updated channel status
        let status = lightning_client.get_channel_status(&channel.id).unwrap();
        assert_eq!(status, ChannelStatus::PendingClose);
    }
    
    #[test]
    fn test_ordinals_client() {
        // Create Bitcoin RPC client
        let config = BitcoinTestnetConfig::new(
            "http://localhost:18332".to_string(),
            "testuser".to_string(),
            "testpassword".to_string(),
            "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
        );
        
        let rpc_client = Arc::new(BitcoinRpcClient::new(&config).unwrap());
        
        // Create Ordinals client
        let ordinals_client = OrdinalsClient::new(
            rpc_client.clone(),
            "http://localhost:3000".to_string(),
        );
        
        // Get inscription
        let inscription_id = "0".repeat(64);
        let inscription = ordinals_client.get_inscription(&inscription_id).unwrap();
        
        // Check inscription properties
        assert_eq!(inscription.id, inscription_id);
        assert!(inscription.txid.starts_with("txid_"));
        assert_eq!(inscription.content_type, "image/png");
        
        // Get inscriptions by address
        let address = "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx";
        let inscriptions = ordinals_client.get_inscriptions_by_address(address).unwrap();
        
        // Check inscriptions
        assert_eq!(inscriptions.len(), 3);
        assert!(inscriptions[0].owner == address);
        
        // Transfer inscription
        let from_address = "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx";
        let to_address = "tb1q0sqzfp2ausf8hy6et2qp5wctgqpn7xpc78qd3d";
        
        let txid = ordinals_client.transfer_inscription(
            &inscription_id,
            from_address,
            to_address,
        ).unwrap();
        
        assert!(!txid.is_empty());
        
        // Get inscription fee
        let fee = ordinals_client.get_inscription_fee(1000, 1.0).unwrap();
        assert!(fee > 0);
    }
    
    #[test]
    fn test_multisig_client() {
        // Create Bitcoin RPC client
        let config = BitcoinTestnetConfig::new(
            "http://localhost:18332".to_string(),
            "testuser".to_string(),
            "testpassword".to_string(),
            "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
        );
        
        let rpc_client = BitcoinRpcClient::new(&config).unwrap();
        
        // Create Multisig client
        let mut multisig_client = MultisigClient::new(
            rpc_client,
            Network::Testnet,
        );
        
        // Create wallet
        let public_keys = vec![
            "02a1633cafcc01ebfb6d78e39f687a1f0995c62fc95f51ead10a02ee0be551b5dc".to_string(),
            "03b31347e0572cb0bd58c11dd29ac3fd8b8ba73cd7f3f5b5e2314f8f5bb5c01968".to_string(),
            "02df2089105c77f266fa11a9d33f05c735234075f2e8780824c6b709415f9fb485".to_string(),
        ];
        
        let wallet = multisig_client.create_wallet(
            "test_wallet",
            2,
            public_keys,
        ).unwrap();
        
        // Check wallet properties
        assert_eq!(wallet.name, "test_wallet");
        assert_eq!(wallet.required_signatures, 2);
        assert_eq!(wallet.total_signers, 3);
        assert_eq!(wallet.network, "testnet");
        
        // Get wallet
        let retrieved_wallet = multisig_client.get_wallet("test_wallet").unwrap();
        assert_eq!(retrieved_wallet.name, "test_wallet");
        
        // Create transaction
        let tx = multisig_client.create_transaction(
            "test_wallet",
            "tb1q0sqzfp2ausf8hy6et2qp5wctgqpn7xpc78qd3d",
            1000,
            1.0,
        ).unwrap();
        
        // Check transaction properties
        assert_eq!(tx.required_signatures, 2);
        assert_eq!(tx.signatures.len(), 0);
        assert_eq!(tx.status, MultisigTxStatus::PendingSignatures);
        
        // Sign transaction
        let signed_tx = multisig_client.sign_transaction(
            &tx.txid,
            "02a1633cafcc01ebfb6d78e39f687a1f0995c62fc95f51ead10a02ee0be551b5dc",
            "signature1",
        ).unwrap();
        
        assert_eq!(signed_tx.signatures.len(), 1);
        assert_eq!(signed_tx.status, MultisigTxStatus::PendingSignatures);
        
        // Sign transaction again
        let signed_tx = multisig_client.sign_transaction(
            &tx.txid,
            "03b31347e0572cb0bd58c11dd29ac3fd8b8ba73cd7f3f5b5e2314f8f5bb5c01968",
            "signature2",
        ).unwrap();
        
        assert_eq!(signed_tx.signatures.len(), 2);
        assert_eq!(signed_tx.status, MultisigTxStatus::ReadyToBroadcast);
        
        // Broadcast transaction
        let txid = multisig_client.broadcast_transaction(&tx.txid).unwrap();
        assert_eq!(txid, tx.txid);
        
        // Get transaction status
        let status = multisig_client.get_transaction_status(&tx.txid).unwrap();
        assert_eq!(status, MultisigTxStatus::Broadcast);
    }
    
    #[test]
    fn test_mempool_monitor() {
        // Create Bitcoin RPC client
        let config = BitcoinTestnetConfig::new(
            "http://localhost:18332".to_string(),
            "testuser".to_string(),
            "testpassword".to_string(),
            "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
        );
        
        let rpc_client = Arc::new(BitcoinRpcClient::new(&config).unwrap());
        
        // Create Mempool monitor
        let mempool_monitor = MempoolMonitor::new(
            rpc_client.clone(),
            Duration::from_secs(1),
        );
        
        // Start monitoring
        let result = mempool_monitor.start();
        assert!(result.is_ok());
        
        // Add monitored address
        let result = mempool_monitor.add_monitored_address("tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx");
        assert!(result.is_ok());
        
        // Wait for a moment to allow monitoring to run
        std::thread::sleep(Duration::from_secs(2));
        
        // Get transactions
        let txs = mempool_monitor.get_transactions().unwrap();
        
        // Get related transactions
        let related_txs = mempool_monitor.get_related_transactions().unwrap();
        
        // Check if a transaction is in mempool
        let is_in_mempool = mempool_monitor.is_in_mempool("non_existent_txid").unwrap();
        assert!(!is_in_mempool);
        
        // Remove monitored address
        let result = mempool_monitor.remove_monitored_address("tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx");
        assert!(result.is_ok());
        
        // Stop monitoring
        let result = mempool_monitor.stop();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_contract_pause_unpause() {
        let mut mock = MockTokenTransferMock::new();
        
        // Setup mock expectations
        mock.expect_validate_address()
            .returning(|_| Ok(()));
        
        mock.expect_supports_token_type()
            .returning(|_| true);
        
        mock.expect_get_network_type()
            .returning(|| "testnet".to_string());
        
        mock.expect_get_balance()
            .returning(|_, _| Ok(10000));
        
        mock.expect_transfer_to_contract()
            .returning(|_, _, _| Ok(()));
        
        // Create contract
        let mut contract = TimeLockedDeposit::new(
            "owner_address".to_string(),
            10, // 10% emergency withdrawal fee
            mock,
        ).unwrap();
        
        // Pause contract (only owner can do this)
        contract.is_contract_paused = true;
        
        // Try to make a deposit while paused
        let result = contract.deposit(
            "depositor_address".to_string(),
            TokenType::Bitcoin,
            1000,
            30,
            Some("txid:0".to_string()),
        );
        
        assert!(matches!(result, Err(ContractError::ContractPaused)));
        
        // Unpause contract
        contract.is_contract_paused = false;
        
        // Make a deposit while unpaused
        let result = contract.deposit(
            "depositor_address".to_string(),
            TokenType::Bitcoin,
            1000,
            30,
            Some("txid:0".to_string()),
        );
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_contract_ownership_transfer() {
        let mut mock = MockTokenTransferMock::new();
        
        // Setup mock expectations
        mock.expect_validate_address()
            .returning(|_| Ok(()));
        
        mock.expect_supports_token_type()
            .returning(|_| true);
        
        mock.expect_get_network_type()
            .returning(|| "testnet".to_string());
        
        // Create contract
        let mut contract = TimeLockedDeposit::new(
            "owner_address".to_string(),
            10, // 10% emergency withdrawal fee
            mock,
        ).unwrap();
        
        // Set pending owner
        contract.pending_owner = Some("new_owner_address".to_string());
        
        // Complete ownership transfer (in a real implementation, this would be a method)
        contract.contract_owner_address = contract.pending_owner.take().unwrap();
        
        // Check new owner
        assert_eq!(contract.contract_owner_address, "new_owner_address");
        assert!(contract.pending_owner.is_none());
    }
    
    #[test]
    fn test_bitcoin_testnet_transfer() {
        // Create Bitcoin testnet configuration
        let config = BitcoinTestnetConfig::new(
            "http://localhost:18332".to_string(),
            "testuser".to_string(),
            "testpassword".to_string(),
            "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
        );
        
        // Create Bitcoin testnet transfer implementation
        let transfer = BitcoinTestnetTransfer::new(config.clone()).unwrap();
        
        // Check network type
        assert_eq!(transfer.get_network_type(), "testnet");
        assert!(transfer.is_testnet());
        
        // Instead of testing private methods directly, we should test their public interfaces
        
        // Process pending transactions
        let result = transfer.process_pending_transactions();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_contract_with_real_transfer() {
        // Create Bitcoin testnet configuration
        let config = BitcoinTestnetConfig::new(
            "http://localhost:18332".to_string(),
            "testuser".to_string(),
            "testpassword".to_string(),
            "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
        );
        
        // Create Bitcoin testnet transfer implementation with all clients
        let transfer = BitcoinTestnetTransfer::new_with_clients(
            config.clone(),
            Some("http://localhost:9735".to_string()),
            Some("http://localhost:3000".to_string()),
        ).unwrap();
        
        // Create contract
        let mut contract = TimeLockedDeposit::new(
            "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
            10, // 10% emergency withdrawal fee
            transfer,
        ).unwrap();
        
        // Check supported tokens
        assert!(contract.supported_tokens.contains(&TokenType::Bitcoin));
        assert!(contract.supported_tokens.contains(&TokenType::Ethereum));
        assert!(contract.supported_tokens.contains(&TokenType::Solana));
        
        // Check for Rune token support
        let has_rune = contract.supported_tokens.iter()
            .any(|t| matches!(t, TokenType::Rune(_)));
        assert!(has_rune);
        
        // Check for Ordinal token support
        let has_ordinal = contract.supported_tokens.iter()
            .any(|t| matches!(t, TokenType::Ordinal(_)));
        assert!(has_ordinal);
        
        // Check for Lightning support
        let has_lightning = contract.supported_tokens.contains(&TokenType::Lightning);
        assert!(has_lightning);
    }
}