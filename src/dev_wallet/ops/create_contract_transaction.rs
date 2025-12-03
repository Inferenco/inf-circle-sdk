use crate::dev_wallet::dto::{AbiParameter, FeeLevel};

/// Builder for creating contract execution transaction requests
///
/// This builder helps construct requests to execute smart contract functions,
/// either using ABI encoding or raw call data.
///
/// # Example
///
/// ```rust,no_run
/// use inf_circle_sdk::dev_wallet::ops::create_contract_transaction::CreateContractExecutionTransactionRequestBuilder;
/// use inf_circle_sdk::dev_wallet::dto::{AbiParameter, FeeLevel};
/// use uuid::Uuid;
///
/// let builder = CreateContractExecutionTransactionRequestBuilder::new(
///     "wallet-id".to_string(),
///     "0xContractAddress".to_string(),
///     Uuid::new_v4().to_string()
/// )
/// .abi_function_signature("transfer(address,uint256)".to_string())
/// .abi_parameters(vec![
///     AbiParameter::String("0x...".to_string()),
///     AbiParameter::String("1000000".to_string()),
/// ])
/// .fee_level(FeeLevel::Medium)
/// .build();
/// ```
#[derive(Clone, Debug)]
pub struct CreateContractExecutionTransactionRequestBuilder {
    pub wallet_id: String,
    pub contract_address: String,
    pub idempotency_key: String,
    pub abi_function_signature: Option<String>,
    pub abi_parameters: Option<Vec<AbiParameter>>,
    pub call_data: Option<String>,
    pub amount: Option<String>,
    pub fee_level: Option<FeeLevel>,
    pub gas_limit: Option<String>,
    pub gas_price: Option<String>,
    pub max_fee: Option<String>,
    pub priority_fee: Option<String>,
    pub ref_id: Option<String>,
}

impl CreateContractExecutionTransactionRequestBuilder {
    /// Create a new builder with required fields
    pub fn new(wallet_id: String, contract_address: String, idempotency_key: String) -> Self {
        Self {
            wallet_id,
            contract_address,
            idempotency_key,
            abi_function_signature: None,
            abi_parameters: None,
            call_data: None,
            amount: None,
            fee_level: None,
            gas_limit: None,
            gas_price: None,
            max_fee: None,
            priority_fee: None,
            ref_id: None,
        }
    }

    /// Set the contract ABI function signature (e.g., "burn(uint256)")
    /// Cannot be used simultaneously with callData
    pub fn abi_function_signature(mut self, signature: String) -> Self {
        self.abi_function_signature = Some(signature);
        self
    }

    /// Set the contract ABI function signature parameters
    /// Should be used exclusively with abiFunctionSignature
    pub fn abi_parameters(mut self, parameters: Vec<AbiParameter>) -> Self {
        self.abi_parameters = Some(parameters);
        self
    }

    /// Set the raw transaction data (hexadecimal string with 0x prefix)
    /// Mutually exclusive with abiFunctionSignature and abiParameters
    pub fn call_data(mut self, call_data: String) -> Self {
        self.call_data = Some(call_data);
        self
    }

    /// Set the amount of native token to send (for payable functions only)
    pub fn amount(mut self, amount: String) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Set the fee level (LOW, MEDIUM, or HIGH)
    /// Cannot be used with gasPrice, priorityFee, or maxFee
    pub fn fee_level(mut self, fee_level: FeeLevel) -> Self {
        self.fee_level = Some(fee_level);
        self
    }

    /// Set the maximum units of gas to use for the transaction
    pub fn gas_limit(mut self, gas_limit: String) -> Self {
        self.gas_limit = Some(gas_limit);
        self
    }

    /// Set the maximum price of gas, in gwei
    /// Requires gasLimit. Cannot be used with feeLevel, priorityFee, or maxFee
    pub fn gas_price(mut self, gas_price: String) -> Self {
        self.gas_price = Some(gas_price);
        self
    }

    /// Set the maximum price per unit of gas, in gwei (EIP-1559)
    /// Requires priorityFee and gasLimit. Cannot be used with feeLevel or gasPrice
    pub fn max_fee(mut self, max_fee: String) -> Self {
        self.max_fee = Some(max_fee);
        self
    }

    /// Set the priority fee, in gwei (EIP-1559)
    /// Requires maxFee and gasLimit. Cannot be used with feeLevel or gasPrice
    pub fn priority_fee(mut self, priority_fee: String) -> Self {
        self.priority_fee = Some(priority_fee);
        self
    }

    /// Set the optional reference or description
    pub fn ref_id(mut self, ref_id: String) -> Self {
        self.ref_id = Some(ref_id);
        self
    }

    /// Build the CreateContractExecutionTransactionRequestBuilder
    pub fn build(self) -> CreateContractExecutionTransactionRequestBuilder {
        self
    }
}
