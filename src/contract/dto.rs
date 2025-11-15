use crate::{helper::PaginationParams, types::Blockchain};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Request structure for estimating contract template deployment fee
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateTemplateDeploymentFeeBody {
    /// Blockchain network
    pub blockchain: Blockchain,

    /// Constructor parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constructor_params: Option<Vec<serde_json::Value>>,

    /// Wallet ID for deployment
    pub wallet_id: String,
}

/// Request structure for deploying a contract from template
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployContractFromTemplateRequest {
    /// Entity secret ciphertext
    pub entity_secret_ciphertext: String,

    /// Contract name
    pub name: String,

    /// Wallet ID for deployment
    pub wallet_id: String,

    /// Blockchain network
    pub blockchain: String,

    /// UUID v4 for idempotency
    pub idempotency_key: String,

    /// Description of the contract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Template parameters for initialization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_parameters: Option<serde_json::Value>,

    /// Fee level (LOW, MEDIUM, HIGH)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_level: Option<String>,

    /// Gas limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<String>,

    /// Gas price (for non-EIP-1559)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,

    /// Max fee (for EIP-1559)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<String>,

    /// Priority fee (for EIP-1559)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_fee: Option<String>,

    /// Reference ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

/// Request structure for importing an existing contract
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportContractRequest {
    /// Blockchain network
    pub blockchain: Blockchain,

    /// Contract address on blockchain
    pub address: String,

    /// Contract name
    pub name: String,

    /// UUID v4 for idempotency
    pub idempotency_key: String,

    /// Description of the contract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request structure for updating a contract
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateContractRequest {
    /// Contract name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Reference ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

/// Fee level details
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeLevelEstimate {
    /// Gas limit
    pub gas_limit: String,

    /// Base fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_fee: Option<String>,

    /// Priority fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_fee: Option<String>,

    /// Max fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<String>,

    /// Gas price (for non-EIP-1559)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,

    /// Network fee - The estimated network fee (maximum amount in native token like ETH, SOL)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_fee: Option<String>,

    /// Network fee raw - Similar to network_fee but with lower buffer, closer to actual on-chain expense
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_fee_raw: Option<String>,
}

/// Fee estimation response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeEstimation {
    /// Low fee estimate
    pub low: FeeLevelEstimate,

    /// Medium fee estimate
    pub medium: FeeLevelEstimate,

    /// High fee estimate
    pub high: FeeLevelEstimate,
}

/// Contract response structure
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    /// Unique contract identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Contract address on blockchain (optional during deployment)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// Contract address on blockchain (alternative field name)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,

    /// Blockchain network
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<String>,

    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<DateTime<Utc>>,

    /// Last update timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<DateTime<Utc>>,

    /// Contract name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Reference identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,

    /// Contract state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Contract status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Template ID used for deployment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,

    /// Deployer wallet ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployer_wallet_id: Option<String>,

    /// Deployment transaction ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_transaction_id: Option<String>,

    /// Deployment transaction hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_tx_hash: Option<String>,

    /// Contract input type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_input_type: Option<String>,

    /// Whether the contract is archived
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,

    /// Contract ABI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi: Option<serde_json::Value>,

    /// Contract ABI JSON string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi_json: Option<String>,

    /// Contract bytecode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytecode: Option<String>,

    /// Contract functions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<serde_json::Value>,

    /// Contract events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<serde_json::Value>,

    /// Verification status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,

    /// Source code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code: Option<serde_json::Value>,

    /// Implementation contract (for proxy contracts)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementation_contract: Option<Box<Contract>>,
}

/// Template contract deployment response
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateContractDeploymentResponse {
    /// Unique identifiers of the created smart contracts
    pub contract_ids: Vec<String>,

    /// Unique identifier of the pending deployment transaction
    pub transaction_id: String,
}

/// Response from deploying a contract from bytecode
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractDeploymentResponse {
    /// Unique identifier of the created smart contract
    pub contract_id: String,

    /// Unique identifier of the deployment transaction
    pub transaction_id: String,
}

/// Response from querying a contract
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryContractResponse {
    /// Output values from the contract query
    /// Can be null if Circle couldn't decode the output
    #[serde(default)]
    pub output_values: Option<Vec<serde_json::Value>>,

    /// Output data in hex format
    pub output_data: String,
}

/// Request structure for deploying a contract from bytecode
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployContractRequest {
    /// Entity secret ciphertext
    pub entity_secret_ciphertext: String,

    /// Bytecode of the contract being deployed
    pub bytecode: String,

    /// The contract's ABI in a JSON stringified format
    pub abi_json: String,

    /// Wallet ID to use as deployment source
    pub wallet_id: String,

    /// Name for the contract (must be alphanumeric)
    pub name: String,

    /// Blockchain network
    pub blockchain: Blockchain,

    /// UUID v4 for idempotency
    pub idempotency_key: String,

    /// Description of the contract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Constructor parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constructor_parameters: Option<Vec<serde_json::Value>>,

    /// Fee level (LOW, MEDIUM, HIGH)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_level: Option<String>,

    /// Gas limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<String>,

    /// Gas price (for non-EIP-1559)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,

    /// Max fee (for EIP-1559)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<String>,

    /// Priority fee (for EIP-1559)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_fee: Option<String>,

    /// Reference ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

/// Contract import/single deployment response
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractResponse {
    /// Imported or deployed contract
    pub contract: Contract,
}

/// Response structure for listing contracts
#[derive(Debug, Deserialize, Serialize)]
pub struct ContractsResponse {
    pub contracts: Vec<Contract>,
}

/// Query parameters for listing contracts
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListContractsParams {
    /// Filter by contract address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// Filter by blockchain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<Blockchain>,

    /// Filter by template ID
    #[serde(rename = "templateId", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,

    /// Filter by reference ID
    #[serde(rename = "refId", skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,

    /// Filter by creation date (from)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<Utc>>,

    /// Filter by creation date (to)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<DateTime<Utc>>,

    /// Pagination parameters
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

/// Notification types for webhook subscriptions
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum NotificationType {
    /// All notification types (wildcard)
    #[serde(rename = "*")]
    All,

    /// All transaction notifications
    #[serde(rename = "transactions.*")]
    TransactionsAll,

    /// Inbound transaction notifications
    #[serde(rename = "transactions.inbound")]
    TransactionsInbound,

    /// Outbound transaction notifications
    #[serde(rename = "transactions.outbound")]
    TransactionsOutbound,

    /// All challenge notifications
    #[serde(rename = "challenges.*")]
    ChallengesAll,

    /// Accelerate transaction challenge
    #[serde(rename = "challenges.accelerateTransaction")]
    ChallengesAccelerateTransaction,

    /// Cancel transaction challenge
    #[serde(rename = "challenges.cancelTransaction")]
    ChallengesCancelTransaction,

    /// Change PIN challenge
    #[serde(rename = "challenges.changePin")]
    ChallengesChangePin,

    /// Contract execution challenge
    #[serde(rename = "challenges.contractExecution")]
    ChallengesContractExecution,

    /// Create transaction challenge
    #[serde(rename = "challenges.createTransaction")]
    ChallengesCreateTransaction,

    /// Create wallet challenge
    #[serde(rename = "challenges.createWallet")]
    ChallengesCreateWallet,

    /// Initialize challenge
    #[serde(rename = "challenges.initialize")]
    ChallengesInitialize,

    /// Restore PIN challenge
    #[serde(rename = "challenges.restorePin")]
    ChallengesRestorePin,

    /// Set PIN challenge
    #[serde(rename = "challenges.setPin")]
    ChallengesSetPin,

    /// Set security questions challenge
    #[serde(rename = "challenges.setSecurityQuestions")]
    ChallengesSetSecurityQuestions,

    /// All contract notifications
    #[serde(rename = "contracts.*")]
    ContractsAll,

    /// Contract event log notifications
    #[serde(rename = "contracts.eventLog")]
    ContractsEventLog,

    /// All modular wallet notifications
    #[serde(rename = "modularWallet.*")]
    ModularWalletAll,

    /// Modular wallet user operation
    #[serde(rename = "modularWallet.userOperation")]
    ModularWalletUserOperation,

    /// Modular wallet inbound transfer
    #[serde(rename = "modularWallet.inboundTransfer")]
    ModularWalletInboundTransfer,

    /// Modular wallet outbound transfer
    #[serde(rename = "modularWallet.outboundTransfer")]
    ModularWalletOutboundTransfer,

    /// All travel rule notifications
    #[serde(rename = "travelRule.*")]
    TravelRuleAll,

    /// Travel rule status update
    #[serde(rename = "travelRule.statusUpdate")]
    TravelRuleStatusUpdate,

    /// Travel rule deny
    #[serde(rename = "travelRule.deny")]
    TravelRuleDeny,

    /// Travel rule approve
    #[serde(rename = "travelRule.approve")]
    TravelRuleApprove,

    /// All ramp session notifications
    #[serde(rename = "rampSession.*")]
    RampSessionAll,

    /// Ramp session completed
    #[serde(rename = "rampSession.completed")]
    RampSessionCompleted,

    /// Ramp session deposit received
    #[serde(rename = "rampSession.depositReceived")]
    RampSessionDepositReceived,

    /// Ramp session expired
    #[serde(rename = "rampSession.expired")]
    RampSessionExpired,

    /// Ramp session failed
    #[serde(rename = "rampSession.failed")]
    RampSessionFailed,

    /// Ramp session KYC approved
    #[serde(rename = "rampSession.kycApproved")]
    RampSessionKycApproved,

    /// Ramp session KYC rejected
    #[serde(rename = "rampSession.kycRejected")]
    RampSessionKycRejected,

    /// Ramp session KYC submitted
    #[serde(rename = "rampSession.kycSubmitted")]
    RampSessionKycSubmitted,
}

impl NotificationType {
    /// Convert the enum to its string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "*",
            Self::TransactionsAll => "transactions.*",
            Self::TransactionsInbound => "transactions.inbound",
            Self::TransactionsOutbound => "transactions.outbound",
            Self::ChallengesAll => "challenges.*",
            Self::ChallengesAccelerateTransaction => "challenges.accelerateTransaction",
            Self::ChallengesCancelTransaction => "challenges.cancelTransaction",
            Self::ChallengesChangePin => "challenges.changePin",
            Self::ChallengesContractExecution => "challenges.contractExecution",
            Self::ChallengesCreateTransaction => "challenges.createTransaction",
            Self::ChallengesCreateWallet => "challenges.createWallet",
            Self::ChallengesInitialize => "challenges.initialize",
            Self::ChallengesRestorePin => "challenges.restorePin",
            Self::ChallengesSetPin => "challenges.setPin",
            Self::ChallengesSetSecurityQuestions => "challenges.setSecurityQuestions",
            Self::ContractsAll => "contracts.*",
            Self::ContractsEventLog => "contracts.eventLog",
            Self::ModularWalletAll => "modularWallet.*",
            Self::ModularWalletUserOperation => "modularWallet.userOperation",
            Self::ModularWalletInboundTransfer => "modularWallet.inboundTransfer",
            Self::ModularWalletOutboundTransfer => "modularWallet.outboundTransfer",
            Self::TravelRuleAll => "travelRule.*",
            Self::TravelRuleStatusUpdate => "travelRule.statusUpdate",
            Self::TravelRuleDeny => "travelRule.deny",
            Self::TravelRuleApprove => "travelRule.approve",
            Self::RampSessionAll => "rampSession.*",
            Self::RampSessionCompleted => "rampSession.completed",
            Self::RampSessionDepositReceived => "rampSession.depositReceived",
            Self::RampSessionExpired => "rampSession.expired",
            Self::RampSessionFailed => "rampSession.failed",
            Self::RampSessionKycApproved => "rampSession.kycApproved",
            Self::RampSessionKycRejected => "rampSession.kycRejected",
            Self::RampSessionKycSubmitted => "rampSession.kycSubmitted",
        }
    }
}

/// Notification subscription details
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSubscription {
    /// System-generated unique identifier of the subscription
    pub id: String,

    /// Name of the webhook notification subscription
    pub name: String,

    /// URL of the endpoint subscribing to notifications
    pub endpoint: String,

    /// Whether the subscription is enabled
    pub enabled: bool,

    /// Date and time the subscription was created
    pub create_date: DateTime<Utc>,

    /// Date and time the subscription was last updated
    pub update_date: DateTime<Utc>,

    /// The notification types on which a notification will be sent
    pub notification_types: Vec<NotificationType>,

    /// Whether the webhook is restricted to specific notification types
    pub restricted: bool,
}

/// Request structure for creating a notification subscription
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNotificationSubscriptionBody {
    /// URL of the endpoint to subscribe to notifications
    pub endpoint: String,

    /// The notification types to subscribe to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_types: Option<Vec<NotificationType>>,
}

/// Response structure for creating a notification subscription
/// Note: The API returns the subscription directly, not wrapped in a collection
pub type CreateNotificationSubscriptionResponse = NotificationSubscription;

/// Request structure for updating a notification subscription
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotificationSubscriptionBody {
    /// Whether the subscription is enabled. true indicates the subscription is active.
    pub enabled: bool,

    /// Name of the subscription
    pub name: String,
}

/// Response structure for updating a notification subscription
/// Note: The API returns the subscription directly, not wrapped in a collection
pub type UpdateNotificationSubscriptionResponse = NotificationSubscription;

/// Response structure for getting health of Circle API
#[derive(Debug, Deserialize, Serialize)]
pub struct PingResponse {
    /// Message
    pub message: String,
}

/// Event monitor details
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventMonitor {
    /// System-generated unique identifier of the event monitor
    pub id: String,

    /// Blockchain network
    pub blockchain: Blockchain,

    /// The on-chain address of the contract
    pub contract_address: String,

    /// The specific event signature being monitored
    pub event_signature: String,

    /// The hash of the event signature
    pub event_signature_hash: String,

    /// Whether the event monitor is enabled
    pub is_enabled: bool,

    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<DateTime<Utc>>,

    /// Last update timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<DateTime<Utc>>,
}

/// Request structure for creating an event monitor
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventMonitorRequest {
    /// UUID v4 for idempotency
    pub idempotency_key: String,

    /// The specific event signature to monitor (no spaces)
    pub event_signature: String,

    /// The on-chain address of the contract
    pub contract_address: String,

    /// Blockchain network
    pub blockchain: Blockchain,
}

/// Response structure for creating an event monitor
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventMonitorResponse {
    /// The created event monitor
    pub event_monitor: EventMonitor,
}

/// Request structure for updating an event monitor
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEventMonitorRequest {
    /// Indicates whether the event monitor should be active (true) or inactive (false)
    pub is_enabled: bool,
}

/// Response structure for listing event monitors
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventMonitorsResponse {
    /// List of event monitors that match criteria
    pub event_monitors: Vec<EventMonitor>,
}

/// Query parameters for listing event monitors
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListEventMonitorsParams {
    /// Filter contracts by address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,

    /// Filter by blockchain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<Blockchain>,

    /// Filter monitors by event signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_signature: Option<String>,

    /// Filter by creation date (from)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<Utc>>,

    /// Filter by creation date (to)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<DateTime<Utc>>,

    /// Pagination parameters
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

/// Event log details
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventLog {
    /// System-generated unique identifier of the event log
    pub id: String,

    /// Block hash where the event was emitted
    pub block_hash: String,

    /// Block height/number where the event was emitted
    pub block_height: i64,

    /// Blockchain network
    pub blockchain: Blockchain,

    /// The on-chain address of the contract that emitted the event
    pub contract_address: String,

    /// The event data in hex format
    pub data: String,

    /// The event signature
    pub event_signature: String,

    /// The hash of the event signature
    pub event_signature_hash: String,

    /// The log index within the transaction
    pub log_index: String,

    /// Array of indexed topics from the event
    pub topics: Vec<String>,

    /// Transaction hash where the event was emitted
    pub tx_hash: String,

    /// User operation hash (for account abstraction)
    pub user_op_hash: String,

    /// Timestamp when the event was first confirmed
    pub first_confirm_date: String,
}

/// Response structure for listing event logs
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventLogsResponse {
    /// List of event logs generated from monitored contract events
    pub event_logs: Vec<EventLog>,
}

/// Query parameters for listing event logs
#[derive(Debug, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListEventLogsParams {
    /// Filter contracts by address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,

    /// Filter by blockchain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<Blockchain>,

    /// Filter by creation date (from)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<Utc>>,

    /// Filter by creation date (to)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<DateTime<Utc>>,

    /// Pagination parameters
    #[serde(flatten)]
    pub pagination: PaginationParams,
}
