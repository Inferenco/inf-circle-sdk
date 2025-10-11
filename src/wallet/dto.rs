use crate::{
    helper::{serialize_bool_as_string, serialize_datetime_as_string, PaginationParams},
    types::Blockchain,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum FeeLevel {
    Low,
    Medium,
    High,
}

impl FeeLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            FeeLevel::Low => "LOW",
            FeeLevel::Medium => "MEDIUM",
            FeeLevel::High => "HIGH",
        }
    }
}

/// Request structure for creating wallets
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletRequest {
    /// System-generated unique identifier of the wallet set
    pub wallet_set_id: String,

    /// Base64 encrypted entity secret ciphertext
    pub entity_secret_ciphertext: String,

    /// Target blockchains for wallet creation
    pub blockchains: Vec<Blockchain>,

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
    pub blockchain: Blockchain,

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

/// Request structure for signing a message
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignMessageRequest {
    /// A base64 string expression of the entity secret ciphertext. The entity secret should be encrypted by the entity public key. Circle mandates that the entity secret ciphertext is unique for each API request.
    pub entity_secret_ciphertext: String,

    /// The user friendly message that needs to be signed. If it is a hex string, encoded_by_hex needs to be TRUE. The hex string should start with "0x" and have even length.
    pub message: String,

    /// System-generated unique identifier of the resource.
    pub wallet_id: String,

    /// Indicator of whether the input message is encoded by hex. If TRUE, then the message should be a hex string. By default, it is False.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_by_hex: Option<bool>,

    /// The human readable explanation for this sign action. Useful for presenting with extra information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// Request structure for signing a data
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignDataRequest {
    /// A base64 string expression of the entity secret ciphertext. The entity secret should be encrypted by the entity public key. Circle mandates that the entity secret ciphertext is unique for each API request.
    pub entity_secret_ciphertext: String,

    /// The data that needs to be signed.
    pub data: String,

    /// System-generated unique identifier of the resource.
    pub wallet_id: String,

    /// The human readable explanation for this sign action. Useful for presenting with extra information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// Response structure for signing a transaction
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignTransactionRequest {
    /// A base64 string expression of the entity secret ciphertext. The entity secret should be encrypted by the entity public key. Circle mandates that the entity secret ciphertext is unique for each API request.
    pub entity_secret_ciphertext: String,

    /// The raw transaction that needs to be signed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_transaction: Option<String>,

    /// The transaction that needs to be signed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<String>,

    /// System-generated unique identifier of the resource.
    pub wallet_id: String,

    /// The human readable explanation for this sign action. Useful for presenting with extra information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// Response structure for signing a transaction
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignTransactionResponse {
    /// Each chain encode signatures in a different way, please refer to Signing APIs doc and the blockchain's document.
    pub signature: String,

    /// Signed transaction. Base64 encoded for NEAR and Solana chains. Hex encoded for EVM chains.
    pub signed_transaction: String,

    /// Blockchain-generated identifier of the transaction. Present if the wallet blockchain is not Solana.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
}

/// Request structure for signing a delegate action
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignDelegateRequest {
    /// A base64 string expression of the entity secret ciphertext. The entity secret should be encrypted by the entity public key. Circle mandates that the entity secret ciphertext is unique for each API request.
    pub entity_secret_ciphertext: String,

    /// Unsigned delegate action string that needs to be signed. Must be base64 encoded.
    pub unsigned_delegate_action: String,

    /// System-generated unique identifier of the resource.
    pub wallet_id: String,
}

/// Response structure for signing a delegate action
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignDelegateResponse {
    /// Each chain encode signatures in a different way, please refer to Signing APIs doc and the blockchain's document.
    pub signature: String,

    /// Signed delegate action is a base64 encoded string for NEAR.
    pub signed_delegate_action: String,
}

/// Response structure for wallet operations
#[derive(Debug, Deserialize)]
pub struct WalletsResponse {
    pub wallets: Vec<Wallet>,
}

/// Response structure for sign message
#[derive(Debug, Deserialize)]
pub struct SignatureResponse {
    /// Each chain encode signatures in a different way, please refer to Signing APIs doc and the blockchain's document.
    pub signature: String,
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

/// Parameters for listing transactions
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactionsParams {
    /// Filter by blockchain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<String>,

    /// Filter by the custody type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custody_type: Option<String>,

    /// Filter by the destination address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_address: Option<String>,

    /// Return all resources with monitored and non-monitored tokens
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_bool_as_string"
    )]
    pub include_all: Option<bool>,

    /// Filter by the operation of the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,

    /// Filter by the state of the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Filter on the transaction hash of the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,

    /// Filter by the transaction type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_type: Option<String>,

    /// Filter by the wallet IDs (comma separated list of ids)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_ids: Option<String>,

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

    /// Specifies the sort order of the collection by CreateDate
    /// Valid values: ASC, DESC (default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Pagination parameters
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

/// Parameters for get transaction
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionParams {
    /// Filter by the transaction type
    pub tx_type: String,
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

/// Transactions data wrapper
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsResponse {
    pub transactions: Vec<Transaction>,
}

/// Transaction response structure
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResponse {
    pub transaction: Transaction,
}

/// Individual transaction
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// System-generated unique identifier of the resource
    pub id: String,

    /// The contract ABI function signature or callData field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi_function_signature: Option<String>,

    /// The contract ABI function signature parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi_parameters: Option<Vec<serde_json::Value>>,

    /// Transfer amounts in decimal number format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts: Option<Vec<String>>,

    /// Transaction amount in USD decimal format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_in_usd: Option<String>,

    /// Identifier for the block that includes the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,

    /// Block height of the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<i64>,

    /// The blockchain network
    pub blockchain: String,

    /// The blockchain address of the contract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,

    /// Date and time the resource was created
    pub create_date: DateTime<Utc>,

    /// Describes who controls the digital assets in a wallet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custody_type: Option<String>,

    /// Blockchain generated unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_address: Option<String>,

    /// Description of the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,

    /// Additional detail associated with the corresponding transaction's error reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<String>,

    /// The estimated fee for the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_fee: Option<EstimatedFee>,

    /// Date the transaction was first confirmed in a block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_confirm_date: Option<DateTime<Utc>>,

    /// Gas fee, in native token, paid to the network for the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_fee: Option<String>,

    /// Gas fee, in USD, paid to the network for the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_fee_in_usd: Option<String>,

    /// List of NFTs associated with the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfts: Option<Vec<String>>,

    /// Operation type of the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,

    /// Optional reference or description used to identify the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,

    /// Blockchain generated unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_address: Option<String>,

    /// Current state of the transaction
    pub state: String,

    /// System-generated unique identifier of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,

    /// Transaction type
    pub transaction_type: String,

    /// Blockchain generated identifier of the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,

    /// Date and time the resource was last updated
    pub update_date: DateTime<Utc>,

    /// Unique system generated identifier for the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// System-generated unique identifier of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,

    /// Transaction screening evaluation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_screening_evaluation: Option<TransactionScreeningEvaluation>,
}

/// Estimated fee for the transaction
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimatedFee {
    /// The maximum units of gas to use for the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<String>,

    /// The maximum price of gas, in gwei, to use per each unit of gas
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,

    /// The maximum price per unit of gas for EIP-1559 support
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<String>,

    /// The "tip" to add to the base fee for EIP-1559 support
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_fee: Option<String>,

    /// The estimated base fee for EIP-1559 support
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_fee: Option<String>,

    /// The estimated network fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_fee: Option<String>,

    /// The estimated network fee with lower buffer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_fee_raw: Option<String>,

    /// Defines the blockchain fee level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_level: Option<String>,
}

/// Transaction screening evaluation
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionScreeningEvaluation {
    /// Name of the matched rule found in screening
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,

    /// Actions to take for the decision
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,

    /// Date and time the resource was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screening_date: Option<DateTime<Utc>>,

    /// Risk signals found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<RiskSignal>>,
}

/// Risk signal
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskSignal {
    /// Source of the risk signal
    pub source: String,

    /// Value of the source
    pub source_value: String,

    /// Risk score of the signal
    pub risk_score: String,

    /// List of risk categories for the signal
    pub risk_categories: Vec<String>,

    /// Type of the signal
    pub r#type: String,
}

/// Request structure for creating a transfer transaction
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransferTransactionRequest {
    /// Unique system generated identifier of the wallet. Required when sourceAddress and blockchain are not provided.
    pub wallet_id: String,

    /// A base64 string expression of the entity secret ciphertext. The entity secret should be encrypted by the entity public key.
    pub entity_secret_ciphertext: String,

    /// Blockchain generated unique identifier, associated with wallet (account), smart contract or other blockchain objects.
    pub destination_address: String,

    /// Universally unique identifier (UUID v4) idempotency key.
    pub idempotency_key: String,

    /// Transfer amounts in decimal number format. For ERC721 token transfer, the amounts field is required to be ["1"].
    pub amounts: Vec<String>,

    /// A dynamic blockchain fee level setting (LOW, MEDIUM, or HIGH) that will be used to pay gas for the transaction.
    /// Cannot be used with gasPrice, priorityFee, or maxFee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_level: Option<FeeLevel>,

    /// The maximum units of gas to use for the transaction. Required if feeLevel is not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<String>,

    /// For blockchains without EIP-1559 support, the maximum price of gas, in gwei, to use per each unit of gas.
    /// Requires gasLimit. Cannot be used with feeLevel, priorityFee, or maxFee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,

    /// For blockchains with EIP-1559 support, the maximum price per unit of gas, in gwei.
    /// Requires priorityFee and gasLimit. Cannot be used with feeLevel or gasPrice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<String>,

    /// For blockchains with EIP-1559 support, the "tip", in gwei, to add to the base fee as an incentive for validators.
    /// Requires maxFee and gasLimit. Cannot be used with feeLevel or gasPrice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_fee: Option<String>,

    /// List of NFT token IDs corresponding with the NFTs to transfer.
    /// Batch transfers are supported only for ERC-1155 tokens. The length of NFT token IDs must match the length of amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nft_token_ids: Option<Vec<String>>,

    /// Optional reference or description used to identify the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,

    /// System generated identifier of the token. Excluded with tokenAddress and tokenBlockchain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,

    /// Blockchain address of the transferred token. Empty for native tokens. Excluded with tokenId.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,

    /// Blockchain of the transferred token. Required if tokenId is not provided.
    /// The blockchain and tokenId fields are mutually exclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<Blockchain>,
}

/// Response structure for creating a transfer transaction
/// Note: The outer `data` wrapper is already unwrapped by HttpClient
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransferTransactionResponse {
    /// System-generated unique identifier of the resource
    pub id: String,

    /// Current state of the transaction
    pub state: String,
}

/// Request structure for validating an address
#[derive(Debug, Serialize)]
pub struct ValidateAddressBody {
    pub address: String,
}

/// Response structure for validating an address
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateAddressResponse {
    pub is_valid: bool,
}

/// ABI parameter types for contract execution
#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum AbiParameter {
    String(String),
    Integer(i64),
    Boolean(bool),
    Array(Vec<AbiParameter>),
}

/// Request structure for estimating contract execution fee
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateContractExecutionFeeBody {
    /// The blockchain address of the contract to be executed
    pub contract_address: String,

    /// The contract ABI function signature (e.g., "burn(uint256)")
    /// Cannot be used simultaneously with callData
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi_function_signature: Option<String>,

    /// The contract ABI function signature parameters
    /// Should be used exclusively with abiFunctionSignature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi_parameters: Option<Vec<AbiParameter>>,

    /// The raw transaction data (hexadecimal string with 0x prefix)
    /// Mutually exclusive with abiFunctionSignature and abiParameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_data: Option<String>,

    /// The amount of native token to send (optional, for payable functions only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,

    /// Blockchain associated with the transaction
    /// Required along with sourceAddress if walletId is not provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<String>,

    /// Source address of the transaction
    /// Required along with blockchain if walletId is not provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_address: Option<String>,

    /// Unique system generated identifier of the wallet
    /// Mutually exclusive with sourceAddress and blockchain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
}

/// Fee estimation data for contract execution
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateContractExecutionFeeResponse {
    /// High fee level estimation
    pub high: EstimatedFee,

    /// Low fee level estimation
    pub low: EstimatedFee,

    /// Medium fee level estimation
    pub medium: EstimatedFee,

    /// ERC-4337 gas field: amount of gas for main execution call (SCA only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_gas_limit: Option<String>,

    /// ERC-4337 gas field: amount of gas for verification step (SCA only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_gas_limit: Option<String>,

    /// ERC-4337 gas field: gas to compensate bundler for pre-verification (SCA only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_verification_gas: Option<String>,
}

/// Request structure for estimating transfer transaction fee
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateTransferFeeRequest {
    /// Blockchain address of the destination
    pub destination_address: String,

    /// Transfer amounts in decimal number format (at least one required)
    /// For ERC721 token transfer, must be ["1"]
    pub amounts: Vec<String>,

    /// List of NFT token IDs (batch transfers supported for ERC-1155 only)
    /// Length must match amounts length
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nft_token_ids: Option<Vec<String>>,

    /// Source address of the transaction
    /// Required with blockchain if walletId is not provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_address: Option<String>,

    /// System generated identifier of the token
    /// Excluded with tokenAddress and blockchain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,

    /// Blockchain address of the transferred token (empty for native tokens)
    /// Excluded with tokenId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,

    /// Blockchain of the transferred token
    /// Required if tokenId is not provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<String>,

    /// Unique system generated identifier of the wallet
    /// Mutually exclusive with sourceAddress and blockchain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
}

/// Response structure for estimating transfer transaction fee
/// Reuses the same structure as contract execution fee estimation
pub type EstimateTransferFeeResponse = EstimateContractExecutionFeeResponse;

/// ABI parameter types for contract queries
#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum ContractAbiParameter {
    String(String),
    Integer(i64),
    Boolean(bool),
    Array(Vec<ContractAbiParameter>),
}

/// Request structure for querying a contract
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryContractRequest {
    /// The blockchain network (required)
    pub blockchain: String,

    /// Address of the contract to be queried (required)
    pub address: String,

    /// The contract ABI function signature (e.g., "balanceOf(address)")
    /// Cannot be used simultaneously with callData
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi_function_signature: Option<String>,

    /// The contract ABI function signature parameters
    /// Should be used exclusively with abiFunctionSignature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi_parameters: Option<Vec<ContractAbiParameter>>,

    /// The contract's ABI in a JSON stringified format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi_json: Option<String>,

    /// CallData is input data that encodes method and parameters
    /// Mutually exclusive with abiFunctionSignature and abiParameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_data: Option<String>,

    /// FromAddress is the address that will populate msg.sender in the contract call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
}

/// Output value types for contract query results
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum ContractOutputValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

/// Response data for contract query
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryContractResponse {
    /// Output for the ABI interaction
    /// May be null if the contract call returns no values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_values: Option<Vec<ContractOutputValue>>,

    /// OutputData is output in hex format
    pub output_data: String,
}

/// SCA Core version enum for wallet upgrades
#[derive(Debug, Clone, Serialize)]
pub enum ScaCore {
    #[serde(rename = "circle_6900_singleowner_v3")]
    Circle6900SingleownerV3,
}

impl ScaCore {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScaCore::Circle6900SingleownerV3 => "circle_6900_singleowner_v3",
        }
    }
}

/// Request structure for creating a wallet upgrade transaction
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletUpgradeTransactionRequest {
    /// Unique system generated identifier of the wallet
    pub wallet_id: String,

    /// A base64 string expression of the entity secret ciphertext
    pub entity_secret_ciphertext: String,

    /// Version of the SCA available for upgrade
    pub new_sca_core: String,

    /// UUID v4 for idempotency
    pub idempotency_key: String,

    /// A dynamic blockchain fee level setting (LOW, MEDIUM, or HIGH)
    /// Cannot be used with gasPrice, priorityFee, or maxFee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_level: Option<FeeLevel>,

    /// The maximum units of gas to use for the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<String>,

    /// For blockchains without EIP-1559 support, the maximum price of gas, in gwei
    /// Requires gasLimit. Cannot be used with feeLevel, priorityFee, or maxFee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,

    /// For blockchains with EIP-1559 support, the maximum price per unit of gas, in gwei
    /// Requires priorityFee and gasLimit. Cannot be used with feeLevel or gasPrice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<String>,

    /// For blockchains with EIP-1559 support, the "tip", in gwei
    /// Requires maxFee and gasLimit. Cannot be used with feeLevel or gasPrice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_fee: Option<String>,

    /// Optional reference or description used to identify the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

/// Response structure for creating a wallet upgrade transaction
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletUpgradeTransactionResponse {
    /// System-generated unique identifier of the resource
    pub id: String,

    /// Current state of the transaction
    pub state: String,
}

/// Request structure for creating a contract execution transaction
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateContractExecutionTransactionRequest {
    /// Unique system generated identifier of the wallet
    pub wallet_id: String,

    /// A base64 string expression of the entity secret ciphertext
    pub entity_secret_ciphertext: String,

    /// The blockchain address of the contract to be executed
    pub contract_address: String,

    /// UUID v4 for idempotency
    pub idempotency_key: String,

    /// The contract ABI function signature (e.g., "burn(uint256)")
    /// Cannot be used simultaneously with callData
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi_function_signature: Option<String>,

    /// The contract ABI function signature parameters
    /// Should be used exclusively with abiFunctionSignature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi_parameters: Option<Vec<AbiParameter>>,

    /// The raw transaction data (hexadecimal string with 0x prefix)
    /// Mutually exclusive with abiFunctionSignature and abiParameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_data: Option<String>,

    /// The amount of native token to send (optional, for payable functions only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,

    /// A dynamic blockchain fee level setting (LOW, MEDIUM, or HIGH)
    /// Cannot be used with gasPrice, priorityFee, or maxFee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_level: Option<FeeLevel>,

    /// The maximum units of gas to use for the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<String>,

    /// For blockchains without EIP-1559 support, the maximum price of gas, in gwei
    /// Requires gasLimit. Cannot be used with feeLevel, priorityFee, or maxFee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,

    /// For blockchains with EIP-1559 support, the maximum price per unit of gas, in gwei
    /// Requires priorityFee and gasLimit. Cannot be used with feeLevel or gasPrice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<String>,

    /// For blockchains with EIP-1559 support, the "tip", in gwei
    /// Requires maxFee and gasLimit. Cannot be used with feeLevel or gasPrice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_fee: Option<String>,

    /// Optional reference or description used to identify the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

/// Response structure for creating a contract execution transaction
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateContractExecutionTransactionResponse {
    /// System-generated unique identifier of the resource
    pub id: String,

    /// Current state of the transaction
    pub state: String,
}

/// Request structure for canceling a transaction
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelTransactionRequest {
    /// A base64 string expression of the entity secret ciphertext
    pub entity_secret_ciphertext: String,

    /// UUID v4 for idempotency
    pub idempotency_key: String,
}

/// Response structure for canceling a transaction
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelTransactionResponse {
    /// System-generated unique identifier of the resource
    pub id: String,

    /// Current state of the transaction
    pub state: String,
}

/// Request structure for accelerating a transaction
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccelerateTransactionRequest {
    /// A base64 string expression of the entity secret ciphertext
    pub entity_secret_ciphertext: String,

    /// UUID v4 for idempotency
    pub idempotency_key: String,
}

/// Response structure for accelerating a transaction
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccelerateTransactionResponse {
    /// System-generated unique identifier of the resource
    pub id: String,
}

/// Request structure for requesting testnet tokens from faucet
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestTestnetTokensRequest {
    /// The testnet blockchain network
    pub blockchain: Blockchain,

    /// Blockchain address to receive tokens
    pub address: String,

    /// Request native testnet tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<bool>,

    /// Request USDC testnet tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usdc: Option<bool>,

    /// Request EURC testnet tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eurc: Option<bool>,
}
