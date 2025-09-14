use crate::helper::{serialize_bool_as_string, serialize_datetime_as_string, PaginationParams};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Request structure for creating wallets
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletRequest {
    /// System-generated unique identifier of the wallet set
    pub wallet_set_id: String,

    /// Base64 encrypted entity secret ciphertext
    pub entity_secret_ciphertext: String,

    /// Target blockchains for wallet creation
    pub blockchains: Vec<String>,

    /// UUID v4 for idempotency
    pub idempotency_key: String,

    /// Account type (SCA or EOA)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,

    /// Number of wallets per blockchain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    /// Metadata for wallets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<WalletMetadata>>,

    /// Wallet name/description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Reference identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

/// Wallet metadata structure
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WalletMetadata {
    /// Name or description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Reference identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

/// Request structure for updating a wallet
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWalletRequest {
    /// Wallet name/description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Reference identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

/// Wallet response structure
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletResponse {
    pub wallet: Wallet,
}

/// Wallet response structure
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    /// Unique wallet identifier
    pub id: String,

    /// Blockchain address
    pub address: String,

    /// Blockchain network
    pub blockchain: String,

    /// Creation timestamp
    pub create_date: DateTime<Utc>,

    /// Last update timestamp
    pub update_date: DateTime<Utc>,

    /// Custody type (DEVELOPER)
    pub custody_type: String,

    /// Wallet name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Reference identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,

    /// Wallet state
    pub state: String,

    /// User identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Wallet set identifier
    pub wallet_set_id: String,

    /// Initial public key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_public_key: Option<String>,

    /// Account type (EOA or SCA)
    pub account_type: String,
}

/// Response structure for wallet operations
#[derive(Debug, Deserialize)]
pub struct WalletsResponse {
    pub wallets: Vec<Wallet>,
}

/// Supported blockchain networks
pub enum Blockchain {
    Eth,
    EthSepolia,
    Avax,
    AvaxFuji,
    Matic,
    MaticAmoy,
    Sol,
    SolDevnet,
    Arb,
    ArbSepolia,
    Near,
    NearTestnet,
    Evm,
    EvmTestnet,
    Uni,
    UniSepolia,
    Base,
    BaseSepolia,
    Op,
    OpSepolia,
    Aptos,
    AptosTestnet,
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
        }
    }
}

/// Account type enum
pub enum AccountType {
    Eoa,
    Sca,
}

impl AccountType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccountType::Eoa => "EOA",
            AccountType::Sca => "SCA",
        }
    }
}

/// Query parameters for listing wallets
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletsParams {
    /// Filter by blockchain address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// Filter by blockchain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<String>,

    /// Filter by SCA version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sca_core: Option<String>,

    /// Filter by wallet set ID
    #[serde(rename = "walletSetId", skip_serializing_if = "Option::is_none")]
    pub wallet_set_id: Option<String>,

    /// Filter by reference ID
    #[serde(rename = "refId", skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,

    /// Filter by creation date (from)
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_datetime_as_string"
    )]
    pub from: Option<DateTime<Utc>>,

    /// Filter by creation date (to)
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_datetime_as_string"
    )]
    pub to: Option<DateTime<Utc>>,

    /// Pagination parameters
    #[serde(flatten)]
    pub pagination: PaginationParams,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

/// Query parameters for listing wallets with token balances
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletsWithBalancesParams {
    /// Required: Filter by blockchain
    pub blockchain: String,

    /// Filter by the blockchain address of the wallet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// Filter by SCA version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sca_core: Option<String>,

    /// Filter by wallet set ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_set_id: Option<String>,

    /// Filter by reference identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,

    /// Filters wallets with a balance greater than or equal to the specified amount.
    /// If tokenAddress is provided, the filter applies to the specified token;
    /// otherwise, it applies to the native token.
    #[serde(skip_serializing_if = "Option::is_none", rename = "amount__gte")]
    pub amount_gte: Option<String>,

    /// Filter by token address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,

    /// Queries items created since the specified date-time (inclusive) in ISO 8601 format
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_datetime_as_string"
    )]
    pub from: Option<DateTime<Utc>>,

    /// Queries items created before the specified date-time (inclusive) in ISO 8601 format
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_datetime_as_string"
    )]
    pub to: Option<DateTime<Utc>>,

    /// Pagination parameters
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

/// Query standard parameters
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    /// Return all resources with monitored and non-monitored tokens
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_bool_as_string"
    )]
    pub include_all: Option<bool>,

    /// Filter by token name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Filter by token address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,

    /// Filter by the token standard (ERC20/ERC721/ERC1155 for EVM chains,
    /// Fungible/FungibleAsset/NonFungible/NonFungibleEdition/ProgrammableNonFungible/ProgrammableNonFungibleEdition for Solana,
    /// FungibleAsset for Aptos)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<String>,

    /// Pagination parameters
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

/// Token balances data wrapper
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalancesResponse {
    pub token_balances: Vec<TokenBalance>,
}

/// Individual token balance
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalance {
    /// Balance amount as string
    pub amount: String,

    /// Token information
    pub token: Token,

    /// Last update timestamp
    pub update_date: DateTime<Utc>,
}

/// Token information
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    /// Unique token identifier
    pub id: String,

    /// Token name
    pub name: Option<String>,

    /// Token standard (e.g., ERC20)
    pub standard: Option<String>,

    /// Blockchain network
    pub blockchain: String,

    /// Number of decimals
    pub decimals: Option<u32>,

    /// Whether this is a native token
    pub is_native: bool,

    /// Token symbol
    pub symbol: Option<String>,

    /// Token contract address
    pub token_address: Option<String>,

    /// Last update timestamp
    pub update_date: DateTime<Utc>,

    /// Creation timestamp
    pub create_date: DateTime<Utc>,
}

/// Wallets with balances response structure
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletsWithBalancesResponse {
    pub wallets: Vec<WalletWithBalances>,
}

/// Wallet with balances
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletWithBalances {
    /// Unique wallet identifier
    pub id: String,

    /// Blockchain address
    pub address: String,

    /// Blockchain network
    pub blockchain: String,

    /// Creation timestamp
    pub create_date: DateTime<Utc>,

    /// Last update timestamp
    pub update_date: DateTime<Utc>,

    /// Custody type (DEVELOPER)
    pub custody_type: String,

    /// Wallet name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Reference identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,

    /// Wallet state
    pub state: String,

    /// User identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Wallet set identifier
    pub wallet_set_id: String,

    /// Initial public key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_public_key: Option<String>,

    /// Account type (EOA or SCA)
    pub account_type: String,

    /// Token balances
    pub token_balances: Vec<TokenBalance>,
}

/// NFTs response structure
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NftsResponse {
    pub nfts: Vec<Nft>,
}

/// Individual NFT
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Nft {
    /// NFT amount as string
    pub amount: String,

    /// NFT metadata URI
    pub metadata: Option<String>,

    /// NFT token ID
    pub nft_token_id: Option<String>,

    /// Token information
    pub token: Token,

    /// Last update timestamp
    pub update_date: DateTime<Utc>,
}
