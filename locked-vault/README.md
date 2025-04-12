# Time-Locked Deposit Contract for Bitcoin Testnet

A secure, feature-rich time-locked deposit contract implementation for Bitcoin testnet with support for Bitcoin, Rune tokens, Ordinals, and Lightning Network payments.

## Features

- **Bitcoin Testnet Support**: Real Bitcoin RPC integration with testnet
- **Multiple Token Types**: Support for Bitcoin, Rune tokens, Ordinals, and Lightning Network payments
- **Time-Locked Deposits**: Lock funds for a specified period
- **Emergency Withdrawals**: Allow early withdrawals with a fee penalty
- **UTXO Management**: Efficient UTXO selection and management
- **Signature Verification**: Secure transaction signing and verification
- **Mempool Monitoring**: Track transactions in the mempool
- **Fee Estimation**: Dynamic fee estimation based on network conditions
- **Multi-Signature Support**: Create and manage multi-signature wallets
- **Batch Processing**: Efficient batch processing of transactions
- **Rate Limiting**: Protect against API abuse
- **Comprehensive Testing**: Extensive test coverage

## Architecture

- **Contract Core**: The main contract logic for deposits and withdrawals
- **Bitcoin RPC**: Integration with Bitcoin Core RPC API
- **UTXO Management**: Handling of unspent transaction outputs
- **Lightning Network**: Support for Lightning Network payments
- **Ordinals**: Support for Ordinal inscriptions
- **Multi-Signature**: Support for multi-signature wallets
- **Mempool Monitoring**: Tracking transactions in the mempool
- **Signature Verification**: Secure transaction signing and verification

## Getting Started

### Prerequisites

- Rust 1.56.0 or later
- Bitcoin Core node (testnet) with RPC enabled
- (Optional) Lightning Network node
- (Optional) Ordinals API endpoint

### Installation

1. Clone the repository:

```bash
git clone
cd repo
```

2. Build the project:

```bash
cargo build --release
```

### Configuration

Create a `.env` file in the project root with the following variables:

```
BITCOIN_TESTNET_RPC_URL=http://localhost:18332
BITCOIN_TESTNET_RPC_USERNAME=your_rpc_username
BITCOIN_TESTNET_RPC_PASSWORD=your_rpc_password
BITCOIN_TESTNET_CONTRACT_WALLET=your_testnet_wallet_address
BITCOIN_TESTNET_OWNER_ADDRESS=your_owner_address
LIGHTNING_NODE_URL=http://localhost:9735  # Optional
ORDINALS_API_URL=http://localhost:3000    # Optional
```

### Running

```bash
cargo run --release
```

## Usage Examples

### Creating a Deposit

```rust
use time_locked_deposit::bitcoin::testnet::BitcoinTestnetConfig;
use time_locked_deposit::bitcoin::transfer::BitcoinTestnetTransfer;
use time_locked_deposit::contract::contract_core::TimeLockedDeposit;
use time_locked_deposit::models::TokenType;

// Create Bitcoin testnet configuration
let config = BitcoinTestnetConfig::new(
    "http://localhost:18332".to_string(),
    "testuser".to_string(),
    "testpassword".to_string(),
    "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
);

// Create Bitcoin testnet transfer implementation
let transfer = BitcoinTestnetTransfer::new_with_clients(
    config,
    Some("http://localhost:9735".to_string()),
    Some("http://localhost:3000".to_string()),
).unwrap();

// Create contract instance
let mut contract = TimeLockedDeposit::new(
    "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
    10, // 10% emergency withdrawal fee
    transfer,
).unwrap();

// Deposit Bitcoin
let result = contract.deposit(
    "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
    TokenType::Bitcoin,
    100000, // 0.001 BTC in satoshis
    30, // 30 days lock period
    Some("txid:0".to_string()), // UTXO reference
);
```

### Withdrawing Funds

```rust
// Withdraw funds after lock period
let result = contract.withdraw(
    "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
    1, // deposit_id
);

// Emergency withdrawal (with fee)
let result = contract.emergency_withdraw(
    "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
    2, // deposit_id
);
```

### Working with Rune Tokens

```rust
// Deposit Rune token
let result = contract.deposit(
    "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
    TokenType::Rune("RUNE_TEST_TOKEN_123".to_string()),
    1000,
    30, // 30 days lock period
    None,
);
```

### Working with Ordinals

```rust
// Deposit Ordinal inscription
let result = contract.deposit(
    "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
    TokenType::Ordinal("0".repeat(64)),
    1,
    30, // 30 days lock period
    None,
);
```

### Working with Lightning Network

```rust
// Deposit via Lightning Network
let result = contract.deposit(
    "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(),
    TokenType::Lightning,
    10000,
    30, // 30 days lock period
    None,
);
```

## Testing

Run the comprehensive test suite:

```bash
cargo test
```

## Deployment on Testnet

1. Ensure you have a Bitcoin Core node running on testnet
2. Configure your RPC credentials in the `.env` file
3. Build and run the application
4. Monitor the logs for successful initialization

## Security Considerations

- **Private Keys**: Never expose private keys in code or environment variables
- **Rate Limiting**: The implementation includes rate limiting to prevent API abuse
- **Reentrancy Protection**: Guards against reentrancy attacks
- **Input Validation**: Thorough validation of all inputs
- **Error Handling**: Comprehensive error handling throughout the codebase
- **Arithmetic Safety**: Checked arithmetic to prevent overflows

## Acknowledgments

- Bitcoin Core developers
- Rust community
- Lightning Network developers
- Ordinals protocol developers