//! NEAR Protocol Data Transfer Objects
//!
//! This module contains all data structures used for NEAR protocol operations,
//! including network identifiers, account balance information, and RPC response types.

use near_primitives::hash::CryptoHash;
use serde::{Deserialize, Serialize};

/// NEAR network identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NearNetwork {
    Mainnet,
    Testnet,
}

impl NearNetwork {
    /// Get the RPC endpoint URL for this network
    pub fn rpc_url(&self) -> &'static str {
        match self {
            NearNetwork::Mainnet => "https://rpc.mainnet.near.org",
            NearNetwork::Testnet => "https://rpc.testnet.near.org",
        }
    }
}

/// NEAR account balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAccountBalance {
    /// Total account balance in NEAR (as string to preserve precision)
    pub total: String,
    /// Available balance in NEAR (total - staked)
    pub available: String,
    /// Staked balance in NEAR
    pub staked: String,
    /// Account state hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_hash: Option<String>,
    /// Block hash when this balance was queried
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<CryptoHash>,
    /// Block height when this balance was queried
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<u64>,
}
