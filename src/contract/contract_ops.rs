//! Contract write operations for CircleOps

use crate::circle_ops::circler_ops::CircleOps;
use crate::contract::dto::{
    ContractDeploymentResponse, ContractResponse, DeployContractFromTemplateRequest,
    DeployContractRequest, ImportContractRequest, TemplateContractDeploymentResponse,
};
use crate::contract::ops::deploy_contract::DeployContractRequestBuilder;
use crate::contract::ops::deploy_contract_from_template::DeployContractFromTemplateRequestBuilder;
use crate::contract::ops::import_contract::ImportContractRequestBuilder;
use crate::helper::CircleResult;
use uuid::Uuid;

impl CircleOps {
    /// Deploy a contract from template
    ///
    /// Deploys a smart contract using a template
    /// Automatically encrypts the entity secret and generates a unique UUID for each request
    pub async fn deploy_contract_from_template(
        &self,
        builder: DeployContractFromTemplateRequestBuilder,
    ) -> CircleResult<TemplateContractDeploymentResponse> {
        // Encrypt the entity secret (fresh encryption for each request)
        let entity_secret_ciphertext = self.entity_secret()?;

        // Generate a new UUID for each request (or use custom one if provided)
        let idempotency_key = builder
            .idempotency_key
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        let template_id = builder.template_id;

        let request = DeployContractFromTemplateRequest {
            entity_secret_ciphertext,
            name: builder.name,
            wallet_id: builder.wallet_id,
            blockchain: builder.blockchain,
            idempotency_key,
            description: builder.description,
            template_parameters: builder.template_parameters,
            fee_level: builder.fee_level,
            gas_limit: builder.gas_limit,
            gas_price: builder.gas_price,
            max_fee: builder.max_fee,
            priority_fee: builder.priority_fee,
            ref_id: builder.ref_id,
        };

        self.post(
            format!("/v1/w3s/templates/{}/deploy", template_id).as_str(),
            &request,
        )
        .await
    }

    /// Deploy a contract from bytecode
    ///
    /// Deploy a smart contract on a specified blockchain using the contract's ABI and bytecode.
    /// The deployment will originate from one of your Circle Wallets.
    /// Automatically encrypts the entity secret and generates a unique UUID for each request.
    pub async fn deploy_contract(
        &self,
        builder: DeployContractRequestBuilder,
    ) -> CircleResult<ContractDeploymentResponse> {
        // Encrypt the entity secret (fresh encryption for each request)
        let entity_secret_ciphertext = self.entity_secret()?;

        // Generate a new UUID for each request (or use custom one if provided)
        let built = builder.build();
        let idempotency_key = built
            .idempotency_key
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        let request = DeployContractRequest {
            entity_secret_ciphertext,
            bytecode: built.bytecode,
            abi_json: built.abi_json,
            wallet_id: built.wallet_id,
            name: built.name,
            blockchain: built.blockchain,
            idempotency_key,
            description: built.description,
            constructor_parameters: built.constructor_parameters,
            fee_level: built.fee_level,
            gas_limit: built.gas_limit,
            gas_price: built.gas_price,
            max_fee: built.max_fee,
            priority_fee: built.priority_fee,
            ref_id: built.ref_id,
        };

        self.post("/v1/w3s/contracts/deploy", &request).await
    }

    /// Import an existing contract
    ///
    /// Add an existing smart contract to your library of contracts
    /// Automatically generates a unique UUID for each request
    pub async fn import_contract(
        &self,
        builder: ImportContractRequestBuilder,
    ) -> CircleResult<ContractResponse> {
        // Generate a new UUID for idempotency
        let idempotency_key = Uuid::new_v4().to_string();

        let request = ImportContractRequest {
            blockchain: builder.blockchain,
            address: builder.address,
            name: builder.name,
            idempotency_key,
            description: builder.description,
        };

        self.post("/v1/w3s/contracts/import", &request).await
    }
}
