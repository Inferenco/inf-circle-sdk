use crate::contract::dto::CreateEventMonitorRequest;
use crate::types::Blockchain;

/// Builder for creating an event monitor request
pub struct CreateEventMonitorBodyBuilder {
    idempotency_key: String,
    event_signature: String,
    contract_address: String,
    blockchain: Blockchain,
}

impl CreateEventMonitorBodyBuilder {
    /// Create a new builder with required parameters
    ///
    /// # Arguments
    /// * `idempotency_key` - UUID v4 for idempotency
    /// * `event_signature` - The specific event signature to monitor (no spaces)
    ///   Example: "Transfer(address indexed from, address indexed to, uint256 value)"
    /// * `contract_address` - The on-chain address of the contract
    /// * `blockchain` - The blockchain network where the contract is deployed
    pub fn new(
        idempotency_key: String,
        event_signature: String,
        contract_address: String,
        blockchain: Blockchain,
    ) -> Self {
        Self {
            idempotency_key,
            event_signature,
            contract_address,
            blockchain,
        }
    }

    /// Build the request
    pub fn build(self) -> CreateEventMonitorRequest {
        CreateEventMonitorRequest {
            idempotency_key: self.idempotency_key,
            event_signature: self.event_signature,
            contract_address: self.contract_address,
            blockchain: self.blockchain,
        }
    }
}
