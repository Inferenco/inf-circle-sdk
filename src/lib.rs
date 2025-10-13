//! # Circle Rust SDK
//!
//! A comprehensive Rust SDK for [Circle's Web3 Services API](https://developers.circle.com/), designed with
//! a clean separation between write (`CircleOps`) and read (`CircleView`) operations.
//!
//! ## Architecture
//!
//! The SDK provides two main clients:
//!
//! - **[`CircleOps`](circle_ops::circler_ops::CircleOps)**: Handles all write operations (POST, PUT, PATCH)
//!   that require entity-level authentication. Uses an entity secret to sign requests.
//! - **[`CircleView`](circle_view::circle_view::CircleView)**: Handles all read operations (GET) that
//!   only require API key authentication.
//!
//! This separation ensures that read-only processes don't require access to sensitive entity secrets,
//! enhancing security and simplifying access control.
//!
//! ## Features
//!
//! - ✅ **Clean Separation**: Write operations require entity secret, reads only need API key
//! - ✅ **Async First**: Built on `tokio` for high-performance async I/O
//! - ✅ **Type Safety**: Strongly typed request/response structures prevent common errors
//! - ✅ **Fluent Builders**: Easy-to-use builders for complex API requests
//! - ✅ **Comprehensive Error Handling**: Detailed error types with helpful messages
//! - ✅ **Developer Wallets**: Create, manage, and transact with developer-controlled wallets
//! - ✅ **Smart Contracts**: Deploy, import, query, and interact with smart contracts
//! - ✅ **Event Monitoring**: Create monitors for contract events and retrieve event logs
//! - ✅ **Webhook Support**: Subscribe to and manage webhook notifications
//!
//! ## Quick Start
//!
//! ### Environment Setup
//!
//! Create a `.env` file in your project root:
//!
//! ```bash
//! CIRCLE_BASE_URL="https://api.circle.com"
//! CIRCLE_API_KEY="YOUR_API_KEY"
//! CIRCLE_ENTITY_SECRET="YOUR_ENTITY_SECRET_HEX"
//! CIRCLE_PUBLIC_KEY="-----BEGIN PUBLIC KEY-----\n...\n-----END PUBLIC KEY-----"
//! CIRCLE_WALLET_SET_ID="YOUR_WALLET_SET_ID"
//! ```
//!
//! ### Create a Wallet
//!
//! ```rust,no_run
//! use inf_circle_sdk::{
//!     circle_ops::circler_ops::CircleOps,
//!     dev_wallet::{dto::AccountType, ops::create_dev_wallet::CreateDevWalletRequestBuilder},
//!     types::Blockchain,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let ops = CircleOps::new()?;
//!     let wallet_set_id = std::env::var("CIRCLE_WALLET_SET_ID")?;
//!
//!     let builder = CreateDevWalletRequestBuilder::new(
//!         wallet_set_id,
//!         vec![Blockchain::EthSepolia]
//!     )?
//!     .account_type(AccountType::Sca)
//!     .count(1)
//!     .build();
//!
//!     let response = ops.create_dev_wallet(builder).await?;
//!     println!("Created wallet: {}", response.wallets[0].address);
//!     Ok(())
//! }
//! ```
//!
//! ### Query Wallet Balance
//!
//! ```rust,no_run
//! use inf_circle_sdk::{
//!     circle_view::circle_view::CircleView,
//!     dev_wallet::views::query::QueryParamsBuilder,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let view = CircleView::new()?;
//!     
//!     let params = QueryParamsBuilder::new().build();
//!     let balances = view.get_token_balances("wallet-id", params).await?;
//!     
//!     for balance in balances.token_balances {
//!         println!("{}: {}", balance.token.symbol.unwrap_or_default(), balance.amount);
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ### Transfer Tokens
//!
//! ```rust,no_run
//! use inf_circle_sdk::{
//!     circle_ops::circler_ops::CircleOps,
//!     dev_wallet::{
//!         dto::FeeLevel,
//!         ops::create_transfer_transaction::CreateTransferTransactionRequestBuilder,
//!     },
//!     types::Blockchain,
//! };
//! use uuid::Uuid;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let ops = CircleOps::new()?;
//!
//!     let builder = CreateTransferTransactionRequestBuilder::new("wallet-id".to_string())
//!         .destination_address("0x1234...".to_string())
//!         .amounts(vec!["0.1".to_string()])
//!         .blockchain(Blockchain::EthSepolia)
//!         .fee_level(FeeLevel::Medium)
//!         .idempotency_key(Uuid::new_v4().to_string())
//!         .build();
//!
//!     let response = ops.create_dev_transfer_transaction(builder).await?;
//!     println!("Transaction ID: {}", response.id);
//!     Ok(())
//! }
//! ```
//!
//! ## Examples
//!
//! The `examples/` directory contains comprehensive examples for all major features:
//!
//! - **[circle_ops_example.rs](https://github.com/Inferenco/inf-circle-sdk/examples/circle_ops_example.rs)**: Wallet creation
//! - **[transfer_transaction_example.rs](https://github.com/Inferenco/inf-circle-sdk/examples/transfer_transaction_example.rs)**: Native and ERC-20 token transfers
//! - **[contract_interaction_example.rs](https://github.com/Inferenco/inf-circle-sdk/examples/contract_interaction_example.rs)**: Execute contract functions
//! - **[deploy_contract_example.rs](https://github.com/Inferenco/inf-circle-sdk/examples/deploy_contract_example.rs)**: Deploy custom contracts
//! - **[create_event_monitor_example.rs](https://github.com/Inferenco/inf-circle-sdk/examples/create_event_monitor_example.rs)**: Monitor contract events
//! - **[sign_message_example.rs](https://github.com/Inferenco/inf-circle-sdk/examples/sign_message_example.rs)**: Sign messages and typed data
//! - **[transaction_management_example.rs](https://github.com/Inferenco/inf-circle-sdk/examples/transaction_management_example.rs)**: Cancel and accelerate transactions
//!
//! Run any example with:
//! ```bash
//! cargo run --example circle_ops_example
//! ```
//!
//! ## Module Organization
//!
//! - [`circle_ops`]: Write operations requiring entity secret authentication
//! - [`circle_view`]: Read operations requiring only API key
//! - [`dev_wallet`]: Developer-controlled wallet operations and views
//! - [`contract`]: Smart contract deployment, import, and interaction
//! - [`types`]: Common types used across the SDK (blockchains, etc.)
//! - [`helper`]: Utility functions and error handling
//!
//! ## Error Handling
//!
//! The SDK uses a custom [`CircleError`](helper::CircleError) type for comprehensive error reporting:
//!
//! ```rust,no_run
//! # use inf_circle_sdk::circle_ops::circler_ops::CircleOps;
//! # use inf_circle_sdk::dev_wallet::ops::create_dev_wallet::CreateDevWalletRequestBuilder;
//! # use inf_circle_sdk::dev_wallet::dto::AccountType;
//! # use inf_circle_sdk::types::Blockchain;
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let ops = CircleOps::new()?;
//! # let wallet_set_id = "test".to_string();
//! # let builder = CreateDevWalletRequestBuilder::new(wallet_set_id, vec![Blockchain::EthSepolia])?.build();
//! match ops.create_dev_wallet(builder).await {
//!     Ok(response) => println!("Success!"),
//!     Err(e) => eprintln!("Error: {}", e),  // Detailed error message
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Testing
//!
//! See [TESTING.md](https://github.com/Inferenco/inf-circle-sdk/TESTING.md) for comprehensive testing guide.

pub mod circle_ops;
pub mod circle_view;
pub mod contract;
pub mod dev_wallet;
pub mod helper;
pub mod types;

// Re-export main types for convenience
pub use helper::{encrypt_entity_secret, CircleError, CircleResult};

// Re-export commonly used types
pub use serde::{Deserialize, Serialize};
pub use uuid::Uuid;
