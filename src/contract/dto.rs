use crate::helper::PaginationParams;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Request structure for estimating contract template deployment fee
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateTemplateDeploymentFeeRequest {
    /// Template ID
    pub template_id: String,

    /// Blockchain network
    pub blockchain: String,

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
    /// Template ID
    pub template_id: String,

    /// Blockchain network
    pub blockchain: String,

    /// Constructor parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constructor_params: Option<Vec<serde_json::Value>>,

    /// Wallet ID for deployment
    pub wallet_id: String,

    /// UUID v4 for idempotency
    pub idempotency_key: String,

    /// Entity secret ciphertext
    pub entity_secret_ciphertext: String,

    /// Contract name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Reference ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
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

/// Fee estimation response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeEstimation {
    /// Estimated gas fee
    pub gas_fee: String,

    /// Gas limit
    pub gas_limit: String,

    /// Gas price
    pub gas_price: String,
}

/// Contract response structure
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    /// Unique contract identifier
    pub id: String,

    /// Contract address on blockchain
    pub address: String,

    /// Blockchain network
    pub blockchain: String,

    /// Creation timestamp
    pub create_date: DateTime<Utc>,

    /// Last update timestamp
    pub update_date: DateTime<Utc>,

    /// Contract name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Reference identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,

    /// Contract state
    pub state: String,

    /// Template ID used for deployment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,

    /// Deployment transaction hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_tx_hash: Option<String>,

    /// Contract ABI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi: Option<serde_json::Value>,

    /// Contract bytecode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytecode: Option<String>,
}

/// Contract deployment response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractDeploymentResponse {
    /// Deployed contract
    pub contract: Contract,

    /// Deployment transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<serde_json::Value>,
}

/// Response structure for listing contracts
#[derive(Debug, Deserialize)]
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
    pub blockchain: Option<String>,

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
