//! Smart contract operations
//!
//! This module provides functionality for deploying, importing, querying, and interacting
//! with smart contracts on supported blockchains.
//!
//! # Features
//!
//! - **Deploy Contracts**: Deploy contracts from bytecode or templates
//! - **Import Contracts**: Import existing contracts for monitoring and interaction
//! - **Query Contracts**: Read contract state without gas fees
//! - **Execute Contracts**: Call contract functions that modify state
//! - **Event Monitoring**: Create monitors for contract events and retrieve logs
//! - **Fee Estimation**: Estimate gas fees before deployment or execution
//!
//! # Main Components
//!
//! - [`contract_ops`]: Write operations (deploy, import contracts)
//! - [`contract_view`]: Read operations (query contracts, list contracts, event monitors)
//! - [`dto`]: Data transfer objects (request/response structures)
//! - [`ops`]: Builder modules for deployment and import operations
//! - [`views`]: Builder modules for query and view operations
//!
//! # Example - Deploy Contract
//!
//! ```rust,no_run
//! use inf_circle_sdk::{
//!     circle_ops::circler_ops::CircleOps,
//!     contract::ops::deploy_contract::DeployContractRequestBuilder,
//!     types::Blockchain,
//! };
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let ops = CircleOps::new()?;
//!
//! let bytecode = "0x608060405234801561001057600080fd5b50...";
//! let abi_json = r#"[{"inputs":[],"name":"getValue",...}]"#;
//!
//! let builder = DeployContractRequestBuilder::new(
//!     bytecode.to_string(),
//!     abi_json.to_string(),
//!     "wallet-id".to_string(),
//!     "MyContract".to_string(),
//!     Blockchain::EthSepolia,
//! );
//!
//! let response = ops.deploy_contract(builder).await?;
//! println!("Contract deployed: {}", response.contract_id);
//! # Ok(())
//! # }
//! ```
//!
//! # Example - Query Contract
//!
//! ```rust,no_run
//! use inf_circle_sdk::{
//!     circle_view::circle_view::CircleView,
//!     contract::views::query_contract_view::QueryContractViewBodyBuilder,
//!     types::Blockchain,
//! };
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let view = CircleView::new()?;
//!
//! let builder = QueryContractViewBodyBuilder::new(
//!     Blockchain::EthSepolia,
//!     "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238".to_string()
//! )
//! .abi_function_signature("balanceOf(address)".to_string())
//! .abi_parameters(vec![serde_json::json!("0x...")]);
//!
//! let response = view.query_contract(builder).await?;
//! println!("Result: {:?}", response.output_values);
//! # Ok(())
//! # }
//! ```

pub mod contract_ops;
pub mod contract_view;
pub mod dto;
pub mod ops;
pub mod views;
