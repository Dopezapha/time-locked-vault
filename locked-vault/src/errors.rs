use thiserror::Error;

/// Error types for the contract
#[derive(Error, Debug)]
pub enum ContractError {
    /// Invalid address
    #[error("Invalid address")]
    InvalidAddress,
    
    /// Invalid amount
    #[error("Invalid amount")]
    InvalidAmount,
    
    /// Invalid lock period
    #[error("Invalid lock period")]
    InvalidLockPeriod,
    
    /// Invalid fee percentage
    #[error("Invalid fee percentage")]
    InvalidFeePercentage,
    
    /// Deposit not found
    #[error("Deposit not found")]
    DepositNotFound,
    
    /// Deposit already withdrawn
    #[error("Deposit already withdrawn")]
    DepositAlreadyWithdrawn,
    
    /// Deposit locked
    #[error("Deposit is still locked")]
    DepositLocked,
    
    /// Insufficient balance
    #[error("Insufficient balance")]
    InsufficientBalance,
    
    /// Unauthorized access
    #[error("Unauthorized access")]
    Unauthorized,
    
    /// Contract paused
    #[error("Contract is paused")]
    ContractPaused,
    
    /// Deposit limit exceeded
    #[error("Deposit limit exceeded")]
    DepositLimitExceeded,
    
    /// User deposit limit reached
    #[error("User deposit limit reached")]
    UserDepositLimitReached,
    
    /// Total deposit limit reached
    #[error("Total deposit limit reached")]
    TotalDepositLimitReached,
    
    /// Unsupported token operation
    #[error("Unsupported token operation")]
    UnsupportedTokenOperation,
    
    /// Token validation failed
    #[error("Token validation failed")]
    TokenValidationFailed,
    
    /// Arithmetic error
    #[error("Arithmetic error")]
    ArithmeticError,
    
    /// Reentrancy detected
    #[error("Reentrancy detected")]
    ReentrancyDetected,
    
    /// Initialization error
    #[error("Initialization error: {0}")]
    InitializationError(String),
    
    /// Bitcoin testnet error
    #[error("Bitcoin testnet error: {0}")]
    BitcoinTestnetError(String),
    
    /// Invalid Bitcoin transaction
    #[error("Invalid Bitcoin transaction")]
    InvalidBitcoinTransaction,
}

impl From<String> for ContractError {
    fn from(error: String) -> Self {
        ContractError::BitcoinTestnetError(error)
    }
}