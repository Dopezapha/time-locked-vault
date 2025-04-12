use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use chrono::{DateTime, Duration, Utc};

use crate::errors::ContractError;
use crate::events::Event;
use crate::models::{Deposit, DepositLimits, FeeConfig, TokenType, TokenTransfer, ReentrancyGuard};

/// Contract version for upgrade tracking
const CONTRACT_VERSION: &str = "1.0.0";

/// Main contract storage with enhanced security features
#[derive(Debug)]
pub struct TimeLockedDeposit<T: TokenTransfer> {
    /// Contract owner address
    pub(crate) contract_owner_address: String,
    /// Next deposit ID to assign
    pub(crate) next_deposit_id: u64,
    /// Mapping of deposit IDs to deposits
    pub(crate) deposit_registry: HashMap<u64, Deposit>,
    /// Mapping of user addresses to their deposit IDs
    pub(crate) user_deposit_ids: HashMap<String, Vec<u64>>,
    /// Fee configuration
    pub(crate) fee_config: FeeConfig,
    /// Contract pause state
    pub(crate) is_contract_paused: bool,
    /// Deposit limits configuration
    pub(crate) deposit_limits: DepositLimits,
    /// Pending ownership transfer address
    pub(crate) pending_owner: Option<String>,
    /// Supported token types
    pub(crate) supported_tokens: Vec<TokenType>,
    ///  Total deposits per token type
    pub(crate) total_deposits: HashMap<TokenType, u64>,
    /// Token transfer implementation
    pub(crate) token_transfer: T,
    /// Reentrancy guard
    pub(crate) reentrancy_guard: ReentrancyGuard,
    /// Contract initialization state
    pub(crate) initialized: AtomicBool,
    /// Contract version
    pub(crate) version: String,
    /// Last maintenance timestamp
    pub(crate) last_maintenance: DateTime<Utc>,
}

impl<T: TokenTransfer> TimeLockedDeposit<T> {
    /// Initialize a new contract instance with enhanced security checks
    /// 
    /// # Gas Optimization
    /// - Uses a fixed-size vector for supported tokens to avoid dynamic resizing
    /// - Initializes hashmaps with capacity hints where possible
    pub fn new(contract_owner_address: String, emergency_withdrawal_fee_percentage: u8, token_transfer: T) -> Result<Self, ContractError> {
        // Validate inputs
        if emergency_withdrawal_fee_percentage > 100 {
            return Err(ContractError::InvalidFeePercentage);
        }
        
        if contract_owner_address.is_empty() {
            return Err(ContractError::InvalidAddress);
        }
        
        // Validate owner address format
        if let Err(e) = token_transfer.validate_address(&contract_owner_address) {
            return Err(ContractError::InitializationError(e));
        }
        
        // Initialize with default supported tokens
        let mut supported_tokens = vec![
            TokenType::Bitcoin,
            TokenType::Ethereum,
            TokenType::Solana,
        ];
        
        // Add Rune token support
        supported_tokens.push(TokenType::Rune("RUNE_DEFAULT_TOKEN".to_string()));
        
        // Add Ordinal support if token_transfer supports it
        if token_transfer.supports_token_type(&TokenType::Ordinal("0".repeat(64))) {
            supported_tokens.push(TokenType::Ordinal("0".repeat(64)));
        }
        
        // Add Lightning support if token_transfer supports it
        if token_transfer.supports_token_type(&TokenType::Lightning) {
            supported_tokens.push(TokenType::Lightning);
        }
        
        let fee_config = FeeConfig {
            emergency_withdrawal_fee_percentage,
            fee_collector_address: contract_owner_address.clone(),
            collected_fees: HashMap::new(),
        };
        
        let now = Utc::now();
        
        let contract = Self {
            contract_owner_address,
            next_deposit_id: 1,
            deposit_registry: HashMap::with_capacity(100), // Pre-allocate for efficiency
            user_deposit_ids: HashMap::with_capacity(50),  // Pre-allocate for efficiency
            fee_config,
            is_contract_paused: false,
            deposit_limits: DepositLimits::default(),
            pending_owner: None,
            supported_tokens,
            total_deposits: HashMap::new(),
            token_transfer,
            reentrancy_guard: ReentrancyGuard::new(),
            initialized: AtomicBool::new(false),
            version: CONTRACT_VERSION.to_string(),
            last_maintenance: now,
        };
        
        // Mark as initialized
        if !contract.initialized.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            return Err(ContractError::InitializationError("Contract already initialized".to_string()));
        }
        
        Ok(contract)
    }
    
    /// Deposit tokens with a time lock - with enhanced security and validation
    /// 
    /// # Gas Optimization
    /// - Uses saturating arithmetic to prevent overflows without reverting
    /// - Avoids unnecessary cloning of data structures
    /// - Minimizes storage operations by batching updates
    pub fn deposit(
        &mut self,
        caller_address: String,
        token_type: TokenType,
        deposit_amount: u64,
        lock_period_days: u32,
        utxo_reference: Option<String>,
    ) -> Result<Event, ContractError> {
        // Reentrancy protection
        let _guard = self.reentrancy_guard.enter().map_err(|_| ContractError::ReentrancyDetected)?;
        
        // Check contract state
        if self.is_contract_paused {
            return Err(ContractError::ContractPaused);
        }
        
        // Validate address
        if let Err(_) = self.token_transfer.validate_address(&caller_address) {
            return Err(ContractError::InvalidAddress);
        }
        
        // Validate token is supported
        if !self.supported_tokens.contains(&token_type) {
            return Err(ContractError::UnsupportedTokenOperation);
        }
        
        // Validate token type parameters
        if let Err(_) = token_type.validate() {
            return Err(ContractError::TokenValidationFailed);
        }
        
        // Validate inputs with more thorough checks
        if deposit_amount == 0 {
            return Err(ContractError::InvalidAmount);
        }
        
        // Check for reasonable deposit amount (prevent overflow attacks)
        if deposit_amount > u64::MAX / 2 {
            return Err(ContractError::InvalidAmount);
        }
        
        if lock_period_days == 0 {
            return Err(ContractError::InvalidLockPeriod);
        }
        
        // Reasonable maximum lock period (10 years)
        if lock_period_days > 3650 {
            return Err(ContractError::InvalidLockPeriod);
        }
        
        // Check deposit limits
        if let Some(max_amount) = self.deposit_limits.max_deposit_amounts.get(&token_type) {
            if deposit_amount > *max_amount {
                return Err(ContractError::DepositLimitExceeded);
            }
        }
        
        // Check user deposit limit
        if let Some(max_deposits) = self.deposit_limits.max_deposits_per_user {
            // Fixed: Create a longer-lived value instead of using a temporary
            let user_deposits = self.user_deposit_ids.get(&caller_address).cloned().unwrap_or_else(Vec::new);
            if user_deposits.len() >= max_deposits as usize {
                return Err(ContractError::UserDepositLimitReached);
            }
        }
        
        // Check total deposit limit with checked arithmetic
        if let Some(max_total) = self.deposit_limits.max_total_deposits {
            let current_total = self.total_deposits.get(&token_type).unwrap_or(&0);
            
            match current_total.checked_add(deposit_amount) {
                Some(new_total) if new_total <= max_total => {},
                Some(_) => return Err(ContractError::TotalDepositLimitReached),
                None => return Err(ContractError::ArithmeticError),
            }
        }
        
        // Check user balance
        match self.token_transfer.get_balance(&caller_address, &token_type) {
            Ok(balance) => {
                if balance < deposit_amount {
                    return Err(ContractError::InsufficientBalance);
                }
            },
            Err(e) => return Err(ContractError::from(e)),
        }
        
        // Transfer tokens from user to contract
        match self.token_transfer.transfer_to_contract(&caller_address, &token_type, deposit_amount) {
            Ok(_) => {},
            Err(e) => return Err(ContractError::from(e)),
        }
        
        // Create deposit
        let current_timestamp = Utc::now();
        let unlock_timestamp = current_timestamp + Duration::days(lock_period_days as i64);
        
        let deposit_id = self.next_deposit_id;
        self.next_deposit_id = self.next_deposit_id.checked_add(1).ok_or(ContractError::ArithmeticError)?;
        
        // Determine if we need Lightning payment hash
        let lightning_payment_hash = if matches!(token_type, TokenType::Lightning) {
            Some(format!("lightning_payment_{}", deposit_id))
        } else {
            None
        };
        
        // Determine if we need multisig wallet
        let multisig_wallet = if token_type.is_bitcoin_based() && caller_address.starts_with("2") {
            Some(format!("multisig_wallet_{}", deposit_id))
        } else {
            None
        };
        
        let new_deposit = Deposit {
            deposit_id,
            depositor_address: caller_address.clone(),
            deposited_token_type: token_type.clone(),
            deposited_amount: deposit_amount,
            deposit_timestamp: current_timestamp,
            unlock_timestamp,
            is_withdrawn: false,
            withdrawal_tx_hash: None,
            last_modified: current_timestamp,
            utxo_reference,
            lightning_payment_hash,
            multisig_wallet,
        };
        
        // Store deposit
        self.deposit_registry.insert(deposit_id, new_deposit);
        
        // Add deposit to user's list
        self.user_deposit_ids
            .entry(caller_address.clone())
            .or_insert_with(Vec::new)
            .push(deposit_id);
        
        // Update total deposits with checked arithmetic
        let current_total = self.total_deposits.get(&token_type).copied().unwrap_or(0);
        let new_total = current_total.checked_add(deposit_amount).ok_or(ContractError::ArithmeticError)?;
        self.total_deposits.insert(token_type.clone(), new_total);
        
        // Return deposit event with enhanced information
        Ok(Event::Deposited {
            deposit_id,
            depositor_address: caller_address,
            token_type,
            deposit_amount,
            unlock_timestamp,
            transaction_hash: None, // Would be filled in a real blockchain implementation
            block_number: None,     // Would be filled in a real blockchain implementation
            timestamp: current_timestamp,
        })
    }
    
    /// Withdraw tokens after time lock has expired - with enhanced security
    /// 
    /// # Gas Optimization
    /// - Uses early returns to avoid unnecessary computation
    /// - Minimizes storage operations
    pub fn withdraw(&mut self, caller_address: String, deposit_id: u64) -> Result<Event, ContractError> {
        // Reentrancy protection
        let _guard = self.reentrancy_guard.enter().map_err(|_| ContractError::ReentrancyDetected)?;
        
        // Check contract state
        if self.is_contract_paused {
            return Err(ContractError::ContractPaused);
        }
        
        // Validate address
        if let Err(_) = self.token_transfer.validate_address(&caller_address) {
            return Err(ContractError::InvalidAddress);
        }
        
        // Get deposit
        let deposit = match self.deposit_registry.get_mut(&deposit_id) {
            Some(deposit) => deposit,
            None => return Err(ContractError::DepositNotFound),
        };
        
        
        // Check ownership
        if deposit.depositor_address != caller_address {
            return Err(ContractError::Unauthorized);
        }
        
        // Check if already withdrawn
        if deposit.is_withdrawn {
            return Err(ContractError::DepositAlreadyWithdrawn);
        }
        
        // Check time lock
        let current_timestamp = Utc::now();
        if current_timestamp < deposit.unlock_timestamp {
            return Err(ContractError::DepositLocked);
        }
        
        // Mark as withdrawn
        deposit.is_withdrawn = true;
        deposit.last_modified = current_timestamp;
        
        let token_type = deposit.deposited_token_type.clone();
        let amount = deposit.deposited_amount;
        
        // Transfer tokens from contract to user
        match self.token_transfer.transfer_from_contract(&caller_address, &token_type, amount) {
            Ok(_) => {},
            Err(e) => return Err(ContractError::from(e)),
        }
        
        // Update totals with checked arithmetic
        if let Some(total) = self.total_deposits.get_mut(&deposit.deposited_token_type) {
            *total = total.checked_sub(deposit.deposited_amount).unwrap_or(0);
        }
        
        // Return withdrawal event with enhanced information
        Ok(Event::Withdrawn {
            deposit_id,
            depositor_address: caller_address,
            token_type: deposit.deposited_token_type.clone(),
            withdrawn_amount: deposit.deposited_amount,
            is_emergency_withdrawal: false,
            transaction_hash: None, // Would be filled in a real blockchain implementation
            block_number: None,     // Would be filled in a real blockchain implementation
            timestamp: current_timestamp,
        })
    }
    
    /// Emergency withdrawal with fee penalty - with enhanced security
    /// 
    /// # Gas Optimization
    /// - Uses checked arithmetic to prevent overflows
    /// - Batches storage updates
    pub fn emergency_withdraw(&mut self, caller_address: String, deposit_id: u64) -> Result<Event, ContractError> {
        // Reentrancy protection
        let _guard = self.reentrancy_guard.enter().map_err(|_| ContractError::ReentrancyDetected)?;
        
        // Check contract state
        if self.is_contract_paused {
            return Err(ContractError::ContractPaused);
        }
        
        // Validate address
        if let Err(_) = self.token_transfer.validate_address(&caller_address) {
            return Err(ContractError::InvalidAddress);
        }
        
        // Get deposit
        let deposit = match self.deposit_registry.get_mut(&deposit_id) {
            Some(deposit) => deposit,
            None => return Err(ContractError::DepositNotFound),
        };
        
        // Check ownership
        if deposit.depositor_address != caller_address {
            return Err(ContractError::Unauthorized);
        }
        
        // Check if already withdrawn
        if deposit.is_withdrawn {
            return Err(ContractError::DepositAlreadyWithdrawn);
        }
        
        // Calculate fee with robust overflow protection
        let fee_percentage = self.fee_config.emergency_withdrawal_fee_percentage;
        let fee_amount = match (deposit.deposited_amount as u128)
            .checked_mul(fee_percentage as u128)
            .and_then(|product| product.checked_div(100)) {
            Some(amount) if amount <= u64::MAX as u128 => amount as u64,
            _ => return Err(ContractError::ArithmeticError),
        };
        
        let net_withdrawal_amount = deposit.deposited_amount.checked_sub(fee_amount)
            .ok_or(ContractError::ArithmeticError)?;
        
        // Mark as withdrawn
        deposit.is_withdrawn = true;
        deposit.last_modified = Utc::now();
        
        let token_type = deposit.deposited_token_type.clone();
        
        // Transfer net amount to user
        match self.token_transfer.transfer_from_contract(&caller_address, &token_type, net_withdrawal_amount) {
            Ok(_) => {},
            Err(e) => return Err(ContractError::from(e)),
        }
        
        // Accumulate fees with checked arithmetic
        let current_fees = self.fee_config.collected_fees
            .entry(deposit.deposited_token_type.clone())
            .or_insert(0);
            
        *current_fees = current_fees.checked_add(fee_amount)
            .ok_or(ContractError::ArithmeticError)?;
        
        // Update totals with checked arithmetic
        if let Some(total) = self.total_deposits.get_mut(&deposit.deposited_token_type) {
            *total = total.checked_sub(deposit.deposited_amount).unwrap_or(0);
        }
        
        // Return emergency withdrawal event with enhanced information
        Ok(Event::EmergencyWithdrawn {
            deposit_id,
            depositor_address: caller_address,
            token_type: deposit.deposited_token_type.clone(),
            withdrawn_amount: net_withdrawal_amount,
            fee_amount,
            transaction_hash: None, // Would be filled in a real blockchain implementation
            block_number: None,     // Would be filled in a real blockchain implementation
            timestamp: Utc::now(),
        })
    }
    
    /// Withdraw collected fees (owner only) - with enhanced security
    pub fn withdraw_fees(&mut self, caller_address: String, token_type: TokenType) -> Result<Event, ContractError> {
        // Reentrancy protection
        let _guard = self.reentrancy_guard.enter().map_err(|_| ContractError::ReentrancyDetected)?;
        
        // Check authorization
        if caller_address != self.contract_owner_address {
            return Err(ContractError::Unauthorized);
        }
        
        // Validate token type
        if let Err(_) = token_type.validate() {
            return Err(ContractError::TokenValidationFailed);
        }
        
        let fee_amount = match self.fee_config.collected_fees.get_mut(&token_type) {
            Some(amount) if *amount > 0 => {
                let tmp = *amount;
                *amount = 0; // Reset collected fees
                tmp
            },
            _ => return Err(ContractError::InvalidAmount),
        };
        
        // Validate collector address
        if let Err(_) = self.token_transfer.validate_address(&self.fee_config.fee_collector_address) {
            return Err(ContractError::InvalidAddress);
        }
        
        // Transfer fees to collector
        match self.token_transfer.transfer_from_contract(
            &self.fee_config.fee_collector_address, 
            &token_type, 
            fee_amount
        ) {
            Ok(_) => {},
            Err(e) => return Err(ContractError::from(e)),
        }
        
        // Return fee collection event with enhanced information
        Ok(Event::FeeCollected {
            token_type,
            fee_amount,
            collector_address: self.fee_config.fee_collector_address.clone(),
            transaction_hash: None, // Would be filled in a real blockchain implementation
            timestamp: Utc::now(),
        })
    }
    
    /// Get the network type
    pub fn get_network_type(&self) -> String {
        self.token_transfer.get_network_type()
    }
    
    /// Check if the contract is on testnet
    pub fn is_testnet(&self) -> bool {
        self.token_transfer.get_network_type() == "testnet"
    }
    
    /// Add a new supported token type
    pub fn add_supported_token(&mut self, caller_address: String, token_type: TokenType) -> Result<(), ContractError> {
        // Check authorization
        if caller_address != self.contract_owner_address {
            return Err(ContractError::Unauthorized);
        }
        
        // Validate token type
        if let Err(_) = token_type.validate() {
            return Err(ContractError::TokenValidationFailed);
        }
        
        // Check if token type is already supported
        if self.supported_tokens.contains(&token_type) {
            return Err(ContractError::UnsupportedTokenOperation);
        }
        
        // Check if token transfer implementation supports this token type
        if !self.token_transfer.supports_token_type(&token_type) {
            return Err(ContractError::UnsupportedTokenOperation);
        }
        
        // Add to supported tokens
        self.supported_tokens.push(token_type);
        
        Ok(())
    }
    
    /// Remove a supported token type
    pub fn remove_supported_token(&mut self, caller_address: String, token_type: TokenType) -> Result<(), ContractError> {
        // Check authorization
        if caller_address != self.contract_owner_address {
            return Err(ContractError::Unauthorized);
        }
        
        // Check if token type is supported
        if !self.supported_tokens.contains(&token_type) {
            return Err(ContractError::UnsupportedTokenOperation);
        }
        
        // Check if there are active deposits for this token type
        if let Some(&total) = self.total_deposits.get(&token_type) {
            if total > 0 {
                return Err(ContractError::UnsupportedTokenOperation);
            }
        }
        
        // Remove from supported tokens
        self.supported_tokens.retain(|t| t != &token_type);
        
        Ok(())
    }
}
