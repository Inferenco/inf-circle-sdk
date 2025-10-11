use crate::types::Blockchain;

/// Builder for EstimateTemplateDeploymentFeeRequest
pub struct EstimateTemplateDeploymentFeeBodyBuilder {
    pub(crate) template_id: String,
    pub(crate) blockchain: Blockchain,
    pub(crate) wallet_id: String,
    pub(crate) constructor_params: Option<Vec<serde_json::Value>>,
    pub(crate) template_parameters: Option<serde_json::Value>,
}

impl EstimateTemplateDeploymentFeeBodyBuilder {
    /// Create a new builder with required fields
    ///
    /// # Arguments
    /// * `template_id` - ID of the template to deploy
    /// * `blockchain` - The blockchain network
    /// * `wallet_id` - Wallet ID to use for deployment
    pub fn new(template_id: String, blockchain: Blockchain, wallet_id: String) -> Self {
        Self {
            template_id,
            blockchain,
            wallet_id,
            constructor_params: None,
            template_parameters: None,
        }
    }

    /// Set constructor parameters
    pub fn constructor_params(mut self, params: Option<Vec<serde_json::Value>>) -> Self {
        self.constructor_params = params;
        self
    }

    /// Set template parameters (JSON object for contract initialization)
    pub fn template_parameters(mut self, params: serde_json::Value) -> Self {
        self.template_parameters = Some(params);
        self
    }

    /// Build the request
    pub fn build(self) -> EstimateTemplateDeploymentFeeBodyBuilder {
        self
    }
}
