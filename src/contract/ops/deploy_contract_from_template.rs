use crate::CircleResult;

/// Builder for DeployContractFromTemplateRequest
#[derive(Clone)]
pub struct DeployContractFromTemplateRequestBuilder {
    pub(crate) template_id: String,
    pub(crate) name: String,
    pub(crate) wallet_id: String,
    pub(crate) blockchain: String,
    pub(crate) description: Option<String>,
    pub(crate) template_parameters: Option<serde_json::Value>,
    pub(crate) fee_level: Option<String>,
    pub(crate) gas_limit: Option<String>,
    pub(crate) gas_price: Option<String>,
    pub(crate) max_fee: Option<String>,
    pub(crate) priority_fee: Option<String>,
    pub(crate) ref_id: Option<String>,
    pub(crate) idempotency_key: Option<String>,
}

impl DeployContractFromTemplateRequestBuilder {
    /// Create a new builder
    ///
    /// Entity secret encryption and UUID generation happen at request time for uniqueness
    pub fn new(
        template_id: String,
        name: String,
        wallet_id: String,
        blockchain: String,
    ) -> CircleResult<Self> {
        Ok(Self {
            template_id,
            name,
            wallet_id,
            blockchain,
            description: None,
            template_parameters: None,
            fee_level: None,
            gas_limit: None,
            gas_price: None,
            max_fee: None,
            priority_fee: None,
            ref_id: None,
            idempotency_key: None,
        })
    }

    /// Set contract description
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set template parameters (JSON object for contract initialization)
    pub fn template_parameters(mut self, params: serde_json::Value) -> Self {
        self.template_parameters = Some(params);
        self
    }

    /// Set fee level (LOW, MEDIUM, or HIGH)
    pub fn fee_level(mut self, level: String) -> Self {
        self.fee_level = Some(level);
        self
    }

    /// Set gas limit
    pub fn gas_limit(mut self, limit: String) -> Self {
        self.gas_limit = Some(limit);
        self
    }

    /// Set gas price (for non-EIP-1559 chains)
    pub fn gas_price(mut self, price: String) -> Self {
        self.gas_price = Some(price);
        self
    }

    /// Set max fee (for EIP-1559 chains)
    pub fn max_fee(mut self, fee: String) -> Self {
        self.max_fee = Some(fee);
        self
    }

    /// Set priority fee (for EIP-1559 chains)
    pub fn priority_fee(mut self, fee: String) -> Self {
        self.priority_fee = Some(fee);
        self
    }

    /// Set reference ID
    pub fn ref_id(mut self, ref_id: String) -> Self {
        self.ref_id = Some(ref_id);
        self
    }

    /// Set custom idempotency key
    pub fn idempotency_key(mut self, key: String) -> Self {
        self.idempotency_key = Some(key);
        self
    }

    /// Build the request parameters
    ///
    /// Returns the builder data for use by the deploy_contract_from_template method
    pub fn build(self) -> DeployContractFromTemplateRequestBuilder {
        self
    }
}
