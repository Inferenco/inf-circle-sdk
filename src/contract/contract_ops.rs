//! Contract write operations for CircleOps

use crate::circle_ops::circler_ops::CircleOps;
use crate::contract::dto::{
    Contract, ContractDeploymentResponse, DeployContractFromTemplateRequest,
    EstimateTemplateDeploymentFeeRequest, FeeEstimation, UpdateContractRequest,
};
use crate::helper::{encrypt_entity_secret, get_env_var, CircleResult};
use crate::CircleError;
use uuid::Uuid;

impl CircleOps {
    /// Estimate fee for contract template deployment
    ///
    /// Estimates the gas fee required to deploy a contract from a template
    pub async fn estimate_template_deployment_fee(
        &self,
        request: EstimateTemplateDeploymentFeeRequest,
    ) -> CircleResult<FeeEstimation> {
        self.post("/v1/w3s/contracts/templates/estimate-fee", &request)
            .await
    }

    /// Deploy a contract from template
    ///
    /// Deploys a new contract instance from a predefined template
    /// Automatically encrypts the entity secret and generates a unique UUID for each request
    pub async fn deploy_contract_from_template(
        &self,
        builder: DeployContractFromTemplateRequestBuilder,
    ) -> CircleResult<ContractDeploymentResponse> {
        let entity_secret = builder.entity_secret;
        let public_key = builder.public_key;

        // Encrypt the entity secret (fresh encryption for each request)
        let entity_secret_ciphertext = encrypt_entity_secret(&entity_secret, &public_key)
            .map_err(|e| CircleError::Config(format!("Failed to encrypt entity secret: {}", e)))?;

        // Generate a new UUID for each request (or use custom one if provided)
        let idempotency_key = Uuid::new_v4().to_string();

        let request = DeployContractFromTemplateRequest {
            template_id: builder.template_id,
            blockchain: builder.blockchain,
            wallet_id: builder.wallet_id,
            entity_secret_ciphertext,
            idempotency_key,
            constructor_params: builder.constructor_params,
            name: builder.name,
            ref_id: builder.ref_id,
        };

        self.post("/v1/w3s/contracts/templates/deploy", &request)
            .await
    }

    /// Update a contract
    ///
    /// Updates contract metadata such as name and reference ID
    pub async fn update_contract(
        &self,
        contract_id: &str,
        request: UpdateContractRequest,
    ) -> CircleResult<Contract> {
        let path = format!("/v1/w3s/contracts/{}", contract_id);
        self.patch(&path, &request).await
    }
}

/// Builder for EstimateTemplateDeploymentFeeRequest
pub struct EstimateTemplateDeploymentFeeRequestBuilder {
    request: EstimateTemplateDeploymentFeeRequest,
}

impl EstimateTemplateDeploymentFeeRequestBuilder {
    /// Create a new builder
    pub fn new(template_id: String, blockchain: String, wallet_id: String) -> Self {
        Self {
            request: EstimateTemplateDeploymentFeeRequest {
                template_id,
                blockchain,
                wallet_id,
                constructor_params: None,
            },
        }
    }

    /// Set constructor parameters
    pub fn constructor_params(mut self, params: Vec<serde_json::Value>) -> Self {
        self.request.constructor_params = Some(params);
        self
    }

    /// Build the request
    pub fn build(self) -> EstimateTemplateDeploymentFeeRequest {
        self.request
    }
}

/// Builder for DeployContractFromTemplateRequest
#[derive(Clone)]
pub struct DeployContractFromTemplateRequestBuilder {
    pub(crate) template_id: String,
    pub(crate) blockchain: String,
    pub(crate) wallet_id: String,
    pub(crate) constructor_params: Option<Vec<serde_json::Value>>,
    pub(crate) name: Option<String>,
    pub(crate) ref_id: Option<String>,
    pub(crate) idempotency_key: Option<String>,
    pub(crate) entity_secret: String,
    pub(crate) public_key: String,
}

impl DeployContractFromTemplateRequestBuilder {
    /// Create a new builder
    ///
    /// Entity secret encryption and UUID generation happen at request time for uniqueness
    pub fn new(template_id: String, blockchain: String, wallet_id: String) -> CircleResult<Self> {
        dotenv::dotenv().ok();
        let entity_secret = get_env_var("CIRCLE_ENTITY_SECRET")?;
        let public_key = get_env_var("CIRCLE_PUBLIC_KEY")?;

        Ok(Self {
            template_id,
            blockchain,
            wallet_id,
            constructor_params: None,
            name: None,
            ref_id: None,
            idempotency_key: None,
            entity_secret,
            public_key,
        })
    }

    /// Set constructor parameters
    pub fn constructor_params(mut self, params: Vec<serde_json::Value>) -> Self {
        self.constructor_params = Some(params);
        self
    }

    /// Set contract name
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
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
