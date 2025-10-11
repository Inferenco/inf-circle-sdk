use crate::dev_wallet::dto::{AbiParameter, EstimateContractExecutionFeeBody};

/// Builder for EstimateContractExecutionFeeBody
pub struct EstimateContractExecutionFeeBodyBuilder {
    contract_address: String,
    abi_function_signature: Option<String>,
    abi_parameters: Option<Vec<AbiParameter>>,
    call_data: Option<String>,
    amount: Option<String>,
    blockchain: Option<String>,
    source_address: Option<String>,
    wallet_id: Option<String>,
}

impl EstimateContractExecutionFeeBodyBuilder {
    /// Create a new builder with required contract address
    pub fn new(contract_address: impl Into<String>) -> Self {
        Self {
            contract_address: contract_address.into(),
            abi_function_signature: None,
            abi_parameters: None,
            call_data: None,
            amount: None,
            blockchain: None,
            source_address: None,
            wallet_id: None,
        }
    }

    /// Set the ABI function signature (e.g., "burn(uint256)")
    /// Cannot be used with call_data
    pub fn abi_function_signature(mut self, signature: Option<String>) -> Self {
        self.abi_function_signature = signature;
        self
    }

    /// Set the ABI parameters
    /// Should be used with abi_function_signature
    pub fn abi_parameters(mut self, parameters: Option<Vec<AbiParameter>>) -> Self {
        self.abi_parameters = parameters;
        self
    }

    /// Set the raw transaction call data (hexadecimal with 0x prefix)
    /// Mutually exclusive with abi_function_signature and abi_parameters
    pub fn call_data(mut self, data: Option<String>) -> Self {
        self.call_data = data;
        self
    }

    /// Set the amount of native token to send (for payable functions)
    pub fn amount(mut self, amount: Option<String>) -> Self {
        self.amount = amount;
        self
    }

    /// Set the blockchain
    /// Required with source_address if wallet_id is not provided
    pub fn blockchain(mut self, blockchain: Option<String>) -> Self {
        self.blockchain = blockchain;
        self
    }

    /// Set the source address
    /// Required with blockchain if wallet_id is not provided
    pub fn source_address(mut self, address: Option<String>) -> Self {
        self.source_address = address;
        self
    }

    /// Set the wallet ID
    /// Mutually exclusive with source_address and blockchain
    pub fn wallet_id(mut self, id: Option<String>) -> Self {
        self.wallet_id = id;
        self
    }

    /// Build the EstimateContractExecutionFeeBody
    pub fn build(self) -> EstimateContractExecutionFeeBody {
        EstimateContractExecutionFeeBody {
            contract_address: self.contract_address,
            abi_function_signature: self.abi_function_signature,
            abi_parameters: self.abi_parameters,
            call_data: self.call_data,
            amount: self.amount,
            blockchain: self.blockchain,
            source_address: self.source_address,
            wallet_id: self.wallet_id,
        }
    }
}
