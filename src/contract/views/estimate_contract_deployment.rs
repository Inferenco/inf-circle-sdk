use serde_json::Value;

/// Builder for estimating contract deployment fees
///
/// According to Circle API: Estimate the network fee for deploying a smart contract
/// on a specified blockchain, given the contract bytecode.
pub struct EstimateContractDeploymentBodyBuilder {
    /// Bytecode of the contract being deployed (required)
    bytecode: String,

    /// The contract's ABI in a JSON stringified format (optional)
    abi_json: Option<String>,

    /// Blockchain network (required with sourceAddress, mutually exclusive with walletId)
    blockchain: Option<String>,

    /// Signature of the constructor if the contract has one (default: "constructor()")
    constructor_signature: Option<String>,

    /// Arguments to pass to the contract's constructor function
    constructor_parameters: Option<Vec<Value>>,

    /// Source address of the transaction (required with blockchain, mutually exclusive with walletId)
    source_address: Option<String>,

    /// Unique system generated identifier of the wallet (mutually exclusive with sourceAddress/blockchain)
    wallet_id: Option<String>,
}

impl EstimateContractDeploymentBodyBuilder {
    /// Create a new builder with required bytecode
    ///
    /// # Arguments
    /// * `bytecode` - Bytecode of the contract being deployed
    pub fn new(bytecode: String) -> Self {
        Self {
            bytecode,
            abi_json: None,
            blockchain: None,
            constructor_signature: None,
            constructor_parameters: None,
            source_address: None,
            wallet_id: None,
        }
    }

    /// Set the contract's ABI in JSON stringified format
    pub fn abi_json(mut self, abi_json: String) -> Self {
        self.abi_json = Some(abi_json);
        self
    }

    /// Set the blockchain network and source address (mutually exclusive with wallet_id)
    pub fn blockchain_and_source(mut self, blockchain: String, source_address: String) -> Self {
        self.blockchain = Some(blockchain);
        self.source_address = Some(source_address);
        self.wallet_id = None; // Clear wallet_id if set
        self
    }

    /// Set the wallet ID (mutually exclusive with blockchain/sourceAddress)
    pub fn wallet_id(mut self, wallet_id: String) -> Self {
        self.wallet_id = Some(wallet_id);
        self.blockchain = None; // Clear blockchain if set
        self.source_address = None; // Clear source_address if set
        self
    }

    /// Set the constructor signature (default: "constructor()")
    pub fn constructor_signature(mut self, signature: String) -> Self {
        self.constructor_signature = Some(signature);
        self
    }

    /// Set the constructor parameters
    pub fn constructor_parameters(mut self, parameters: Vec<Value>) -> Self {
        self.constructor_parameters = Some(parameters);
        self
    }

    /// Build the request body as JSON
    pub fn build(self) -> Value {
        let mut body = serde_json::json!({
            "bytecode": self.bytecode,
        });

        if let Some(abi_json) = self.abi_json {
            body["abiJson"] = Value::String(abi_json);
        }

        if let Some(blockchain) = self.blockchain {
            body["blockchain"] = Value::String(blockchain);
        }

        if let Some(constructor_sig) = self.constructor_signature {
            body["constructorSignature"] = Value::String(constructor_sig);
        }

        if let Some(constructor_params) = self.constructor_parameters {
            body["constructorParameters"] = Value::Array(constructor_params);
        }

        if let Some(source_address) = self.source_address {
            body["sourceAddress"] = Value::String(source_address);
        }

        if let Some(wallet_id) = self.wallet_id {
            body["walletId"] = Value::String(wallet_id);
        }

        body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_with_wallet_id() {
        let builder = EstimateContractDeploymentBodyBuilder::new("0x6080604052...".to_string())
            .wallet_id("wallet-123".to_string())
            .build();

        assert_eq!(builder["bytecode"], "0x6080604052...");
        assert_eq!(builder["walletId"], "wallet-123");
        assert!(builder.get("blockchain").is_none());
        assert!(builder.get("sourceAddress").is_none());
    }

    #[test]
    fn test_builder_with_blockchain_and_source() {
        let builder = EstimateContractDeploymentBodyBuilder::new("0x6080604052...".to_string())
            .blockchain_and_source("ETH-SEPOLIA".to_string(), "0x123...".to_string())
            .build();

        assert_eq!(builder["bytecode"], "0x6080604052...");
        assert_eq!(builder["blockchain"], "ETH-SEPOLIA");
        assert_eq!(builder["sourceAddress"], "0x123...");
        assert!(builder.get("walletId").is_none());
    }

    #[test]
    fn test_builder_with_constructor() {
        let params = vec![serde_json::json!("0xAddress"), serde_json::json!(100)];

        let builder = EstimateContractDeploymentBodyBuilder::new("0x6080604052...".to_string())
            .wallet_id("wallet-123".to_string())
            .constructor_signature("constructor(address,uint256)".to_string())
            .constructor_parameters(params.clone())
            .build();

        assert_eq!(builder["bytecode"], "0x6080604052...");
        assert_eq!(
            builder["constructorSignature"],
            "constructor(address,uint256)"
        );
        assert_eq!(builder["constructorParameters"], serde_json::json!(params));
    }
}
