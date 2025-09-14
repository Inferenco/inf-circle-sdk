//! Contract read operations for CircleView
use crate::circle_view::circle_view::CircleView;
use crate::helper::CircleResult;
// Re-use the Contract struct from CircleOps since it's the same
pub use crate::contract::dto::{Contract, ContractsResponse, ListContractsParams};

impl CircleView {
    /// List contracts
    ///
    /// Retrieves a list of all contracts that fit the specified parameters
    pub async fn list_contracts(
        &self,
        params: Option<ListContractsParams>,
    ) -> CircleResult<ContractsResponse> {
        match params {
            Some(params) => self.get_with_params("/v1/w3s/contracts", &params).await,
            None => self.get("/v1/w3s/contracts").await,
        }
    }

    /// Get a specific contract
    ///
    /// Retrieves details of a specific contract by ID
    pub async fn get_contract(&self, contract_id: &str) -> CircleResult<Contract> {
        let path = format!("/v1/w3s/contracts/{}", contract_id);
        self.get(&path).await
    }
}
