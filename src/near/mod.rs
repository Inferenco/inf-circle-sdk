//! NEAR Protocol Support
//!
//! This module provides functionality for working with NEAR protocol,
//! including account balance queries, delegate action serialization, and public key parsing.
//!
//! # Main Components
//!
//! - [`dto`]: Data transfer objects (network identifiers, account balances, RPC types)
//! - [`handler`]: Helper functions for NEAR operations
//!
//! # Example - Get Account Balance
//!
//! ```rust,no_run
//! use inf_circle_sdk::near::{get_near_account_balance, dto::NearNetwork};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let balance = get_near_account_balance("guest-book.testnet", NearNetwork::Testnet).await?;
//! println!("Total balance: {} NEAR", balance.total);
//! println!("Available: {} NEAR", balance.available);
//! println!("Staked: {} NEAR", balance.staked);
//! # Ok(())
//! # }
//! ```
//!
//! # Example - Serialize Delegate Action
//!
//! ```rust,no_run
//! use inf_circle_sdk::near::serialize_near_delegate_action_to_base64;
//! use near_primitives::action::delegate::DelegateAction;
//!
//! # fn example(delegate_action: &DelegateAction) -> Result<(), Box<dyn std::error::Error>> {
//! let base64_encoded = serialize_near_delegate_action_to_base64(delegate_action)?;
//! println!("Serialized delegate action: {}", base64_encoded);
//! # Ok(())
//! # }
//! ```
//!
//! # Example - Parse Public Key
//!
//! ```rust,no_run
//! use inf_circle_sdk::near::parse_near_public_key;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Parse with ed25519: prefix
//! let pk1 = parse_near_public_key("ed25519:5tzF9KaC4uEJ9rZx2vXJ5J5J5J5J5J5J5J5J5J5J5J5J")?;
//!
//! // Parse without prefix (Circle format)
//! let pk2 = parse_near_public_key("5tzF9KaC4uEJ9rZx2vXJ5J5J5J5J5J5J5J5J5J5J5J5J")?;
//! # Ok(())
//! # }
//! ```

pub mod dto;
pub mod handler;

// Re-export commonly used items
pub use dto::{NearAccountBalance, NearNetwork};
pub use handler::{
    get_near_account_balance, parse_near_public_key, serialize_near_delegate_action_to_base64,
};
