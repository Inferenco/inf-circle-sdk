//! # Circle SDK
//!
//! A Rust SDK for Circle's API that cleanly separates write vs read flows.
//!
//! ## Features
//!
//! - **CircleOps**: Write operations (POST, PUT, PATCH) with entity secret authentication
//! - **CircleView**: Read operations (GET) with base URL configuration
//! - **Async/await support**: Built on tokio for async operations
//! - **Type safety**: Strongly typed request/response structures
//! - **Error handling**: Comprehensive error types with detailed messages
//!
//! ## Usage
//!
//! ```rust
//! use inf_circle_sdk::{circle_ops::circler_ops::CircleOps, circle_view::circle_view::CircleView};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // For write operations
//!     let ops = CircleOps::new()?;
//!     
//!     // For read operations  
//!     let view = CircleView::new()?;
//!     
//!     Ok(())
//! }
//! ```

pub mod circle_ops;
pub mod circle_view;
pub mod contract;
pub mod helper;
pub mod wallet;

// Re-export main types for convenience
pub use helper::{encrypt_entity_secret, CircleError, CircleResult};

// Re-export commonly used types
pub use serde::{Deserialize, Serialize};
pub use uuid::Uuid;
