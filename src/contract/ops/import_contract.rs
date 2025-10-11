use crate::types::Blockchain;

pub struct ImportContractRequestBuilder {
    pub blockchain: Blockchain,
    pub address: String,
    pub name: String,
    pub description: Option<String>,
}

impl ImportContractRequestBuilder {
    /// Create a new builder with required fields
    ///
    /// # Arguments
    /// * `blockchain` - The blockchain network
    /// * `address` - Address of the contract to import
    /// * `name` - Name for the contract (must be alphanumeric [a-zA-Z0-9])
    pub fn new(blockchain: Blockchain, address: String, name: String) -> Self {
        Self {
            blockchain,
            address,
            name,
            description: None,
        }
    }

    pub fn description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    pub fn build(self) -> Self {
        self
    }
}
