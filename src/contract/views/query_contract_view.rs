use serde_json::Value;

use crate::types::Blockchain;

/// Builder for querying contract state
///
/// Execute a query function on a contract by providing the address and blockchain.
/// Query the state of a contract (read-only operations).
pub struct QueryContractViewBodyBuilder {
    /// Blockchain network (required)
    blockchain: Blockchain,

    /// Address of the contract to be queried (required)
    address: String,

    /// The contract ABI function signature (e.g., "balanceOf(address)")
    /// Mutually exclusive with callData
    abi_function_signature: Option<String>,

    /// Parameters for the ABI function signature
    /// Used exclusively with abiFunctionSignature
    abi_parameters: Option<Vec<Value>>,

    /// The contract's ABI in JSON stringified format (optional)
    abi_json: Option<String>,

    /// CallData - input data that encodes method and parameters
    /// Mutually exclusive with abiFunctionSignature
    call_data: Option<String>,

    /// FromAddress - the address that will populate msg.sender in the contract call
    from_address: Option<String>,
}

impl QueryContractViewBodyBuilder {
    /// Create a new builder with required fields
    ///
    /// # Arguments
    /// * `blockchain` - The blockchain network
    /// * `address` - Address of the contract to query
    pub fn new(blockchain: Blockchain, address: String) -> Self {
        Self {
            blockchain,
            address,
            abi_function_signature: None,
            abi_parameters: None,
            abi_json: None,
            call_data: None,
            from_address: None,
        }
    }

    /// Set the ABI function signature (e.g., "balanceOf(address)")
    ///
    /// Mutually exclusive with call_data.
    /// Use this with abi_parameters for high-level function calls.
    pub fn abi_function_signature(mut self, signature: String) -> Self {
        self.abi_function_signature = Some(signature);
        self.call_data = None; // Clear call_data if set
        self
    }

    /// Set the ABI parameters
    ///
    /// The parameters for the ABI function signature.
    /// Should be used exclusively with abi_function_signature.
    pub fn abi_parameters(mut self, parameters: Vec<Value>) -> Self {
        self.abi_parameters = Some(parameters);
        self
    }

    /// Set the contract's ABI in JSON stringified format
    pub fn abi_json(mut self, abi_json: String) -> Self {
        self.abi_json = Some(abi_json);
        self
    }

    /// Set the call data
    ///
    /// CallData is input data that encodes method and parameters.
    /// Mutually exclusive with abi_function_signature and abi_parameters.
    pub fn call_data(mut self, call_data: String) -> Self {
        self.call_data = Some(call_data);
        self.abi_function_signature = None; // Clear abi_function_signature if set
        self.abi_parameters = None; // Clear abi_parameters if set
        self
    }

    /// Set the from address
    ///
    /// The address that will populate msg.sender in the contract call.
    pub fn from_address(mut self, from_address: String) -> Self {
        self.from_address = Some(from_address);
        self
    }

    /// Build the request body as JSON
    pub fn build(self) -> Value {
        let mut body = serde_json::json!({
            "blockchain": self.blockchain,
            "address": self.address,
        });

        if let Some(abi_sig) = self.abi_function_signature {
            body["abiFunctionSignature"] = Value::String(abi_sig);
        }

        if let Some(abi_params) = self.abi_parameters {
            body["abiParameters"] = Value::Array(abi_params);
        }

        if let Some(abi_json) = self.abi_json {
            body["abiJson"] = Value::String(abi_json);
        }

        if let Some(call_data) = self.call_data {
            body["callData"] = Value::String(call_data);
        }

        if let Some(from_addr) = self.from_address {
            body["fromAddress"] = Value::String(from_addr);
        }

        body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_with_abi_signature() {
        let builder =
            QueryContractViewBodyBuilder::new(Blockchain::EthSepolia, "0x123...".to_string())
                .abi_function_signature("balanceOf(address)".to_string())
                .abi_parameters(vec![serde_json::json!("0xAddress")])
                .build();

        assert_eq!(builder["blockchain"], "ETH-SEPOLIA");
        assert_eq!(builder["address"], "0x123...");
        assert_eq!(builder["abiFunctionSignature"], "balanceOf(address)");
        assert!(builder.get("callData").is_none());
    }

    #[test]
    fn test_builder_with_call_data() {
        let builder =
            QueryContractViewBodyBuilder::new(Blockchain::EthSepolia, "0x123...".to_string())
                .call_data("0x70a08231000000000000000000000000...".to_string())
                .build();

        assert_eq!(builder["callData"], "0x70a08231000000000000000000000000...");
        assert!(builder.get("abiFunctionSignature").is_none());
        assert!(builder.get("abiParameters").is_none());
    }

    #[test]
    fn test_builder_with_from_address() {
        let builder =
            QueryContractViewBodyBuilder::new(Blockchain::EthSepolia, "0x123...".to_string())
                .abi_function_signature("owner()".to_string())
                .from_address("0xSender".to_string())
                .build();

        assert_eq!(builder["fromAddress"], "0xSender");
    }
}
