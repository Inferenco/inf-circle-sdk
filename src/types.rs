//! Common types used across the SDK
//!
//! This module defines shared types, enums, and structures used throughout the SDK.

use serde::{Deserialize, Serialize};

/// Supported blockchain networks
///
/// Circle supports multiple blockchain networks for wallet and contract operations.
///
/// # Mainnets
///
/// - `Eth` - Ethereum mainnet
/// - `Avax` - Avalanche C-Chain mainnet
/// - `Matic` - Polygon mainnet
/// - `Sol` - Solana mainnet
/// - `Arb` - Arbitrum One mainnet
/// - `Near` - NEAR Protocol mainnet
/// - `Uni` - Unichain mainnet
/// - `Base` - Base mainnet
/// - `Op` - Optimism mainnet
/// - `Aptos` - Aptos mainnet
///
/// # Testnets
///
/// - `EthSepolia` - Ethereum Sepolia testnet
/// - `AvaxFuji` - Avalanche Fuji testnet
/// - `MaticAmoy` - Polygon Amoy testnet
/// - `SolDevnet` - Solana devnet
/// - `ArbSepolia` - Arbitrum Sepolia testnet
/// - `NearTestnet` - NEAR Protocol testnet
/// - `UniSepolia` - Unichain Sepolia testnet
/// - `BaseSepolia` - Base Sepolia testnet
/// - `OpSepolia` - Optimism Sepolia testnet
/// - `AptosTestnet` - Aptos testnet
/// - `ArcTestnet` - Arc testnet
///
/// # Example
///
/// ```rust
/// use inf_circle_sdk::types::Blockchain;
///
/// let blockchain = Blockchain::EthSepolia;
/// println!("Using blockchain: {}", blockchain.as_str());  // "ETH-SEPOLIA"
/// ```
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub enum Blockchain {
    #[serde(rename = "ETH")]
    Eth,
    #[serde(rename = "ETH-SEPOLIA")]
    EthSepolia,
    #[serde(rename = "AVAX")]
    Avax,
    #[serde(rename = "AVAX-FUJI")]
    AvaxFuji,
    #[serde(rename = "MATIC")]
    Matic,
    #[serde(rename = "MATIC-AMOY")]
    MaticAmoy,
    #[serde(rename = "SOL")]
    Sol,
    #[serde(rename = "SOL-DEVNET")]
    SolDevnet,
    #[serde(rename = "ARB")]
    Arb,
    #[serde(rename = "ARB-SEPOLIA")]
    ArbSepolia,
    #[serde(rename = "NEAR")]
    Near,
    #[serde(rename = "NEAR-TESTNET")]
    NearTestnet,
    #[serde(rename = "EVM")]
    Evm,
    #[serde(rename = "EVM-TESTNET")]
    EvmTestnet,
    #[serde(rename = "UNI")]
    Uni,
    #[serde(rename = "UNI-SEPOLIA")]
    UniSepolia,
    #[serde(rename = "BASE")]
    Base,
    #[serde(rename = "BASE-SEPOLIA")]
    BaseSepolia,
    #[serde(rename = "OP")]
    Op,
    #[serde(rename = "OP-SEPOLIA")]
    OpSepolia,
    #[serde(rename = "APTOS")]
    Aptos,
    #[serde(rename = "APTOS-TESTNET")]
    AptosTestnet,
    #[serde(rename = "ARC-TESTNET")]
    ArcTestnet,
}

impl Blockchain {
    pub fn as_str(&self) -> &'static str {
        match self {
            Blockchain::Eth => "ETH",
            Blockchain::EthSepolia => "ETH-SEPOLIA",
            Blockchain::Avax => "AVAX",
            Blockchain::AvaxFuji => "AVAX-FUJI",
            Blockchain::Matic => "MATIC",
            Blockchain::MaticAmoy => "MATIC-AMOY",
            Blockchain::Sol => "SOL",
            Blockchain::SolDevnet => "SOL-DEVNET",
            Blockchain::Arb => "ARB",
            Blockchain::ArbSepolia => "ARB-SEPOLIA",
            Blockchain::Near => "NEAR",
            Blockchain::NearTestnet => "NEAR-TESTNET",
            Blockchain::Evm => "EVM",
            Blockchain::EvmTestnet => "EVM-TESTNET",
            Blockchain::Uni => "UNI",
            Blockchain::UniSepolia => "UNI-SEPOLIA",
            Blockchain::Base => "BASE",
            Blockchain::BaseSepolia => "BASE-SEPOLIA",
            Blockchain::Op => "OP",
            Blockchain::OpSepolia => "OP-SEPOLIA",
            Blockchain::Aptos => "APTOS",
            Blockchain::AptosTestnet => "APTOS-TESTNET",
            Blockchain::ArcTestnet => "ARC-TESTNET",
        }
    }
}

impl Serialize for Blockchain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
