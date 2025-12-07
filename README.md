# Circle Rust SDK

[![Crates.io](https://img.shields.io/crates/v/inf-circle-sdk.svg)](https://crates.io/crates/inf-circle-sdk)
[![Docs.rs](https://docs.rs/inf-circle-sdk/badge.svg)](https://docs.rs/inf-circle-sdk)

A comprehensive Rust SDK for Circle's API, designed with a clean separation between write (`CircleOps`) and read (`CircleView`) operations. This library provides a type-safe, async-first interface to interact with Circle's powerful developer APIs.

## Architecture

The SDK is split into two main components:

- **`CircleOps`**: Handles all write operations (POST, PUT, PATCH) that require entity-level authentication. It uses an entity secret to sign requests and provides a secure way to create and manage resources.
- **`CircleView`**: Handles all read operations (GET) that do not require entity-level authentication. It provides a simple interface to query data from the Circle API.

This separation ensures that read-only processes do not require access to sensitive entity secrets, enhancing security and simplifying access control.

## Features

- ✅ **Clean Separation**: `CircleOps` for writes, `CircleView` for reads
- ✅ **Async First**: Built on `tokio` for non-blocking, asynchronous operations
- ✅ **Type-Safe**: Strongly typed request and response structures to prevent common errors
- ✅ **Fluent Builders**: Easy-to-use builders for constructing complex API requests
- ✅ **Comprehensive Error Handling**: Detailed error types to simplify debugging
- ✅ **Developer Wallets**: Create, manage, and transact with EOA and SCA wallets
- ✅ **Smart Contracts**: Deploy, import, query, and interact with smart contracts
- ✅ **Event Monitoring**: Create monitors for contract events and retrieve event logs
- ✅ **Webhook Notifications**: Subscribe to and manage webhook notifications
- ✅ **Transaction Management**: Cancel and accelerate pending transactions
- ✅ **Message Signing**: Sign messages, typed data (EIP-712), and transactions
- ✅ **Comprehensive Documentation**: 31 passing doc tests with working examples

## Built With This SDK

### Nova - AI Telegram Bot with Multi-Chain Wallet Support

[Nova](https://inferenco.com/app.html#nova) is a sophisticated AI-powered Telegram bot that leverages this Circle SDK for seamless multi-chain wallet management. Nova demonstrates the real-world capabilities of the Circle SDK in production.

**Key Features:**
- 🤖 **AI-Powered Conversations**: Advanced language models (GPT-5, GPT-5-mini) with tool-calling capabilities
- 🔗 **Multi-Chain Wallet**: Built with Circle SDK, supporting Aptos, Solana, and 20+ EVM-compatible blockchains
- 💰 **Token Management**: Send and receive tokens across multiple networks with automatic gas handling
- 📊 **Market Data**: Real-time cryptocurrency prices, trending pools, DEX data, and price predictions
- 🗳️ **DAO Integration**: On-chain proposals and voting capabilities
- 💸 **Automated Payments**: Scheduled payments and seamless token transfers between group members
- 🛡️ **Sentinel Protection**: Automated moderation and protection against bad actors

**Try Nova:**
- 🌐 **Website**: [https://inferenco.com/app.html#nova](https://inferenco.com/app.html#nova)
- 🤖 **Telegram Bot**: [@NovaInferencoBot](https://t.me/NovaInferencoBot)

Nova showcases how the Circle SDK enables transparent, pay-per-use blockchain applications with seamless Web3 integrations.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
inf-circle-sdk = "0.1.8"
```

## Usage

### Environment Variables

Create a `.env` file in your project root with the following variables:

```
CIRCLE_BASE_URL="https://api.circle.com"
CIRCLE_API_KEY="YOUR_API_KEY"
CIRCLE_ENTITY_SECRET="YOUR_ENTITY_SECRET_HEX"
CIRCLE_PUBLIC_KEY="-----BEGIN PUBLIC KEY-----\nYOUR_RSA_PUBLIC_KEY_IN_PEM_FORMAT\n-----END PUBLIC KEY-----"
CIRCLE_WALLET_SET_ID="YOUR_WALLET_SET_ID"
```

**Note:** 
- `CIRCLE_API_KEY` is used for API authentication in the Authorization header
- `CIRCLE_ENTITY_SECRET` should be the hex-encoded entity secret
- `CIRCLE_PUBLIC_KEY` should be the RSA public key in PEM format (PKCS#1 or PKCS#8)
- The entity secret is automatically encrypted using RSA-OAEP with SHA-256 at request time
- Each API call generates a fresh encryption and unique UUID for security

### Quick Start Examples

#### Create a Wallet

```rust
use inf_circle_sdk::{
    circle_ops::circler_ops::CircleOps,
    dev_wallet::{dto::AccountType, ops::create_dev_wallet::CreateDevWalletRequestBuilder},
    types::Blockchain,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ops = CircleOps::new(None)?;
    let wallet_set_id = std::env::var("CIRCLE_WALLET_SET_ID")?;

    let builder = CreateDevWalletRequestBuilder::new(
        wallet_set_id,
        vec![Blockchain::EthSepolia]
    )?
    .account_type(AccountType::Sca)
    .count(1)
    .build();

    let response = ops.create_dev_wallet(builder).await?;
    println!("Created wallet: {}", response.wallets[0].address);
    Ok(())
}
```

#### Query Wallet Balances

```rust
use inf_circle_sdk::{
    circle_view::circle_view::CircleView,
    dev_wallet::views::query::QueryParamsBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let view = CircleView::new()?;
    
    let params = QueryParamsBuilder::new().build();
    let balances = view.get_token_balances("wallet-id", params).await?;
    
    for balance in balances.token_balances {
        println!("{}: {}", balance.token.symbol.unwrap_or_default(), balance.amount);
    }
    Ok(())
}
```

#### Transfer Tokens

```rust
use inf_circle_sdk::{
    circle_ops::circler_ops::CircleOps,
    dev_wallet::{
        dto::FeeLevel,
        ops::create_transfer_transaction::CreateTransferTransactionRequestBuilder,
    },
    types::Blockchain,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ops = CircleOps::new(None)?;

    let builder = CreateTransferTransactionRequestBuilder::new("wallet-id".to_string())
        .destination_address("0x1234...".to_string())
        .amounts(vec!["0.1".to_string()])
        .blockchain(Blockchain::EthSepolia)
        .fee_level(FeeLevel::Medium)
        .idempotency_key(Uuid::new_v4().to_string())
        .build();

    let response = ops.create_dev_transfer_transaction(builder).await?;
    println!("Transaction ID: {}", response.id);
    Ok(())
}
```

## Examples

The SDK includes 12 comprehensive examples covering all major features:

### Wallet Operations
- **[circle_ops_example.rs](examples/circle_ops_example.rs)** - Create wallets (EOA and SCA)
- **[wallet_balances_example.rs](examples/wallet_balances_example.rs)** - Query token balances and NFTs
- **[transfer_transaction_example.rs](examples/transfer_transaction_example.rs)** - Transfer native tokens and ERC-20 tokens
- **[sign_message_example.rs](examples/sign_message_example.rs)** - Sign messages and EIP-712 typed data
- **[transaction_management_example.rs](examples/transaction_management_example.rs)** - Cancel and accelerate transactions

### Contract Operations
- **[deploy_contract_example.rs](examples/deploy_contract_example.rs)** - Deploy contracts from bytecode
- **[import_contract_example.rs](examples/import_contract_example.rs)** - Import existing contracts
- **[query_contract_example.rs](examples/query_contract_example.rs)** - Query contract state (read-only)
- **[contract_interaction_example.rs](examples/contract_interaction_example.rs)** - Execute contract functions
- **[estimate_contract_deployment_example.rs](examples/estimate_contract_deployment_example.rs)** - Estimate deployment fees

### Event Monitoring
- **[create_event_monitor_example.rs](examples/create_event_monitor_example.rs)** - Create, update, delete, and list event monitors

### General
- **[circle_view_example.rs](examples/circle_view_example.rs)** - Read operations overview

Run any example with:
```bash
cargo run --example wallet_balances_example
```

## Testing

The SDK includes comprehensive integration tests. To run them, you'll need to set up your environment:

### Required Environment Variables

```bash
CIRCLE_BASE_URL="https://api.circle.com"
CIRCLE_API_KEY="YOUR_API_KEY"
CIRCLE_ENTITY_SECRET="YOUR_ENTITY_SECRET_HEX"
CIRCLE_PUBLIC_KEY="-----BEGIN PUBLIC KEY-----\nYOUR_RSA_PUBLIC_KEY\n-----END PUBLIC KEY-----"
CIRCLE_WALLET_SET_ID="YOUR_WALLET_SET_ID"
CIRCLE_TEMPLATE_ID="YOUR_CONTRACT_TEMPLATE_ID"
```

### Webhook Testing

The notification subscription tests require a publicly accessible webhook endpoint because Circle validates endpoints by making HTTP requests to them.

**Quick Setup:**
```bash
# Use the helper script (auto-detects tunnelto or ngrok)
./scripts/start_webhook_server.sh

# Export the URL shown and run tests
export CIRCLE_TEST_WEBHOOK_URL="<url-from-script>"
cargo test test_notification_subscriptions_crud
```

**For detailed webhook setup options (tunnelto, ngrok, webhook.site, custom subdomains), see [scripts/README.md](scripts/README.md).**

If you don't set `CIRCLE_TEST_WEBHOOK_URL`, the notification subscription test will be automatically skipped.

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_notification_subscriptions_crud

# Run with output
cargo test -- --nocapture
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

