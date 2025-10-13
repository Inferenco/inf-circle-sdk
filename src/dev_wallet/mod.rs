//! Developer-controlled wallet operations
//!
//! This module provides functionality for managing developer-controlled wallets,
//! including creating wallets, transferring tokens, signing messages, and managing transactions.
//!
//! # Wallet Types
//!
//! - **EOA (Externally Owned Account)**: Traditional wallets with a single private key
//! - **SCA (Smart Contract Account)**: Smart contract-based wallets with advanced features
//!
//! # Main Components
//!
//! - [`dev_wallet_ops`]: Write operations (create wallets, transfers, signing, etc.)
//! - [`dev_wallet_view`]: Read operations (list wallets, query balances, transactions, etc.)
//! - [`dto`]: Data transfer objects (request/response structures)
//! - [`ops`]: Builder modules for write operations
//! - [`views`]: Builder modules for read operations
//!
//! # Example
//!
//! ```rust,no_run
//! use inf_circle_sdk::{
//!     circle_ops::circler_ops::CircleOps,
//!     dev_wallet::ops::create_dev_wallet::CreateDevWalletRequestBuilder,
//!     dev_wallet::dto::AccountType,
//!     types::Blockchain,
//! };
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let ops = CircleOps::new()?;
//! let wallet_set_id = std::env::var("CIRCLE_WALLET_SET_ID")?;
//!
//! let builder = CreateDevWalletRequestBuilder::new(
//!     wallet_set_id,
//!     vec![Blockchain::EthSepolia]
//! )?
//! .account_type(AccountType::Sca)
//! .count(1)
//! .build();
//!
//! let response = ops.create_dev_wallet(builder).await?;
//! # Ok(())
//! # }
//! ```

pub mod dev_wallet_ops;
pub mod dev_wallet_view;
pub mod dto;
pub mod ops;
pub mod views;
