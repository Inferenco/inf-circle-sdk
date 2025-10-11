use serde_json::Value;

use crate::types::Blockchain;

/// Builder for deploying a contract from bytecode
///
/// Deploy a smart contract on a specified blockchain using the contract's ABI and bytecode.
/// The deployment will originate from one of your Circle Wallets.
pub struct DeployContractRequestBuilder {
    // Required fields
    bytecode: String,
    abi_json: String,
    wallet_id: String,
    name: String,
    blockchain: Blockchain,

    // Optional fields
    description: Option<String>,
    constructor_parameters: Option<Vec<Value>>,
    fee_level: Option<String>,
    gas_limit: Option<String>,
    gas_price: Option<String>,
    max_fee: Option<String>,
    priority_fee: Option<String>,
    ref_id: Option<String>,
    idempotency_key: Option<String>,
}

impl DeployContractRequestBuilder {
    /// Create a new builder with required fields
    ///
    /// # Arguments
    /// * `bytecode` - Bytecode of the contract being deployed
    /// * `abi_json` - The contract's ABI in a JSON stringified format
    /// * `wallet_id` - Wallet ID to use as the deployment source
    /// * `name` - Name for the contract (must be alphanumeric [a-zA-Z0-9])
    /// * `blockchain` - The blockchain network
    pub fn new(
        bytecode: String,
        abi_json: String,
        wallet_id: String,
        name: String,
        blockchain: Blockchain,
    ) -> Self {
        Self {
            bytecode,
            abi_json,
            wallet_id,
            name,
            blockchain,
            description: None,
            constructor_parameters: None,
            fee_level: None,
            gas_limit: None,
            gas_price: None,
            max_fee: None,
            priority_fee: None,
            ref_id: None,
            idempotency_key: None,
        }
    }

    /// Set the description for the contract
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set the constructor parameters
    ///
    /// A list of arguments to pass to the contract's constructor function.
    /// Must be an empty array if there are no constructor parameters.
    pub fn constructor_parameters(mut self, parameters: Vec<Value>) -> Self {
        self.constructor_parameters = Some(parameters);
        self
    }

    /// Set the fee level (LOW, MEDIUM, or HIGH)
    ///
    /// A dynamic blockchain fee level setting that will be used to pay gas for the transaction.
    /// Cannot be used with gasPrice, priorityFee, or maxFee.
    pub fn fee_level(mut self, level: String) -> Self {
        self.fee_level = Some(level);
        self
    }

    /// Set the gas limit
    ///
    /// The maximum units of gas to use for the transaction.
    /// Required if feeLevel is not provided.
    pub fn gas_limit(mut self, limit: String) -> Self {
        self.gas_limit = Some(limit);
        self
    }

    /// Set the gas price (for non-EIP-1559 blockchains)
    ///
    /// The maximum price of gas, in gwei, to use per each unit of gas.
    /// Requires gasLimit. Cannot be used with feeLevel, priorityFee, or maxFee.
    pub fn gas_price(mut self, price: String) -> Self {
        self.gas_price = Some(price);
        self
    }

    /// Set the max fee (for EIP-1559 blockchains)
    ///
    /// The maximum price per unit of gas, in gwei.
    /// Requires priorityFee and gasLimit. Cannot be used with feeLevel or gasPrice.
    pub fn max_fee(mut self, fee: String) -> Self {
        self.max_fee = Some(fee);
        self
    }

    /// Set the priority fee (for EIP-1559 blockchains)
    ///
    /// The "tip", in gwei, to add to the base fee as an incentive for validators.
    /// Requires maxFee and gasLimit. Cannot be used with feeLevel or gasPrice.
    pub fn priority_fee(mut self, fee: String) -> Self {
        self.priority_fee = Some(fee);
        self
    }

    /// Set the reference ID
    ///
    /// Optional reference or description used to identify the transaction.
    pub fn ref_id(mut self, ref_id: String) -> Self {
        self.ref_id = Some(ref_id);
        self
    }

    /// Set the idempotency key
    ///
    /// Universally unique identifier (UUID v4) idempotency key.
    /// If not provided, a new UUID will be generated automatically.
    pub fn idempotency_key(mut self, key: String) -> Self {
        self.idempotency_key = Some(key);
        self
    }

    /// Build and return all fields for CircleOps to use
    pub fn build(self) -> DeployContractRequest {
        DeployContractRequest {
            bytecode: self.bytecode,
            abi_json: self.abi_json,
            wallet_id: self.wallet_id,
            name: self.name,
            blockchain: self.blockchain,
            description: self.description,
            constructor_parameters: self.constructor_parameters,
            fee_level: self.fee_level,
            gas_limit: self.gas_limit,
            gas_price: self.gas_price,
            max_fee: self.max_fee,
            priority_fee: self.priority_fee,
            ref_id: self.ref_id,
            idempotency_key: self.idempotency_key,
        }
    }
}

/// Internal request structure for deploy contract
pub struct DeployContractRequest {
    pub bytecode: String,
    pub abi_json: String,
    pub wallet_id: String,
    pub name: String,
    pub blockchain: Blockchain,
    pub description: Option<String>,
    pub constructor_parameters: Option<Vec<Value>>,
    pub fee_level: Option<String>,
    pub gas_limit: Option<String>,
    pub gas_price: Option<String>,
    pub max_fee: Option<String>,
    pub priority_fee: Option<String>,
    pub ref_id: Option<String>,
    pub idempotency_key: Option<String>,
}
