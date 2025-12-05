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
    /// Deploys a smart contract using a pre-configured template. Templates simplify deployment
    /// by providing pre-written, audited contract code that can be customized with parameters.
    /// Automatically encrypts the entity secret and generates a unique UUID for each request.
    ///
    /// # Arguments
    ///
    /// * `builder` - A `DeployContractFromTemplateRequestBuilder` with deployment parameters
    ///
    /// # Returns
    ///
    /// Returns contract IDs and transaction ID for the deployment.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_ops::circler_ops::CircleOps;
    /// use inf_circle_sdk::contract::ops::deploy_contract_from_template::DeployContractFromTemplateRequestBuilder;
    /// use inf_circle_sdk::types::Blockchain;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let ops = CircleOps::new(None)?;
    ///
    /// // Deploy an NFT contract from template
    /// let template_params = serde_json::json!({
    ///     "defaultAdmin": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
    /// });
    ///
    /// let builder = DeployContractFromTemplateRequestBuilder::new(
    ///     "template-id".to_string(),
    ///     "My NFT Collection".to_string(),
    ///     "wallet-id".to_string(),
    ///     Blockchain::EthSepolia.as_str().to_string(),
    /// )?
    /// .template_parameters(template_params)
    /// .description("My awesome NFT collection".to_string())
    /// .build();
    ///
    /// let response = ops.deploy_contract_from_template(builder).await?;
    /// println!("Contract IDs: {:?}", response.contract_ids);
    /// println!("Transaction ID: {}", response.transaction_id);
    /// # Ok(())
    /// # }
    /// ```
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
    /// Deploys a custom smart contract using its compiled bytecode and ABI.
    /// This gives you full control over the contract code. The deployment will originate
    /// from one of your Circle Wallets. Automatically encrypts the entity secret and
    /// generates a unique UUID for each request.
    ///
    /// # Arguments
    ///
    /// * `builder` - A `DeployContractRequestBuilder` with bytecode, ABI, and deployment params
    ///
    /// # Returns
    ///
    /// Returns the contract ID and deployment transaction ID.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_ops::circler_ops::CircleOps;
    /// use inf_circle_sdk::contract::ops::deploy_contract::DeployContractRequestBuilder;
    /// use inf_circle_sdk::types::Blockchain;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let ops = CircleOps::new(None)?;
    ///
    /// // Bytecode from compiled Solidity contract
    /// let bytecode = "0x608060405234801561001057600080fd5b50...";
    /// let abi_json = r#"[{"inputs":[],"name":"getValue","outputs":[...]...}]"#;
    ///
    /// let builder = DeployContractRequestBuilder::new(
    ///     bytecode.to_string(),
    ///     abi_json.to_string(),
    ///     "wallet-id".to_string(),
    ///     "MyContract".to_string(),
    ///     Blockchain::EthSepolia,
    /// )
    /// .description("My custom contract".to_string())
    /// .constructor_parameters(vec![serde_json::json!("param1")]);
    ///
    /// let response = ops.deploy_contract(builder).await?;
    /// println!("Contract ID: {}", response.contract_id);
    /// println!("Transaction ID: {}", response.transaction_id);
    /// # Ok(())
    /// # }
    /// ```
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
    /// Imports an existing deployed smart contract into your Circle account.
    /// This allows you to interact with contracts deployed outside of Circle
    /// or contracts deployed by others. Automatically generates a unique UUID for each request.
    ///
    /// # Arguments
    ///
    /// * `builder` - An `ImportContractRequestBuilder` with the contract address and details
    ///
    /// # Returns
    ///
    /// Returns the imported contract details.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_ops::circler_ops::CircleOps;
    /// use inf_circle_sdk::contract::ops::import_contract::ImportContractRequestBuilder;
    /// use inf_circle_sdk::types::Blockchain;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let ops = CircleOps::new(None)?;
    ///
    /// // Import USDC contract on Sepolia
    /// let builder = ImportContractRequestBuilder::new(
    ///     Blockchain::EthSepolia,
    ///     "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238".to_string(),
    ///     "USDC".to_string(),
    /// )
    /// .description(Some("USD Coin on Sepolia".to_string()))
    /// .build();
    ///
    /// let response = ops.import_contract(builder).await?;
    /// println!("Imported contract ID: {:?}", response.contract.id);
    /// # Ok(())
    /// # }
    /// ```
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
