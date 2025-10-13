//! Wallet write operations for CircleOps

use crate::{
    circle_ops::circler_ops::CircleOps,
    dev_wallet::{
        dto::{
            AccelerateTransactionRequest, AccelerateTransactionResponse, CancelTransactionRequest,
            CancelTransactionResponse, CreateContractExecutionTransactionRequest,
            CreateContractExecutionTransactionResponse, CreateDevWalletRequest,
            CreateTransferTransactionRequest, CreateTransferTransactionResponse,
            CreateWalletUpgradeTransactionRequest, CreateWalletUpgradeTransactionResponse,
            DevWalletResponse, DevWalletsResponse, QueryContractRequest, QueryContractResponse,
            SignDataRequest, SignDelegateRequest, SignDelegateResponse, SignMessageRequest,
            SignTransactionRequest, SignTransactionResponse, SignatureResponse,
            UpdateDevWalletRequest,
        },
        ops::{
            accelerate_transaction::AccelerateTransactionRequestBuilder,
            cancel_transaction::CancelTransactionRequestBuilder,
            create_contract_transaction::CreateContractExecutionTransactionRequestBuilder,
            create_dev_wallet::CreateDevWalletRequestBuilder,
            create_transfer_transaction::CreateTransferTransactionRequestBuilder,
            create_wallet_upgrade_transaction::CreateWalletUpgradeTransactionRequestBuilder,
            sign_data::SignDataRequestBuilder, sign_delegate::SignDelegateRequestBuilder,
            sign_message::SignMessageRequestBuilder,
            sign_transaction::SignTransactionRequestBuilder,
        },
    },
    helper::CircleResult,
};
use uuid::Uuid;

impl CircleOps {
    /// Create new wallets
    ///
    /// Creates a new developer-controlled wallet or batch of wallets within a wallet set.
    /// Automatically encrypts the entity secret and generates a unique UUID for each request.
    ///
    /// # Arguments
    ///
    /// * `builder` - A `CreateDevWalletRequestBuilder` configured with wallet parameters
    ///
    /// # Returns
    ///
    /// Returns a `DevWalletsResponse` containing the created wallet(s) with their addresses and IDs.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_ops::circler_ops::CircleOps;
    /// use inf_circle_sdk::dev_wallet::ops::create_dev_wallet::CreateDevWalletRequestBuilder;
    /// use inf_circle_sdk::dev_wallet::dto::AccountType;
    /// use inf_circle_sdk::types::Blockchain;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let ops = CircleOps::new()?;
    /// let wallet_set_id = std::env::var("CIRCLE_WALLET_SET_ID")?;
    ///
    /// // Create a single SCA wallet on Ethereum Sepolia
    /// let builder = CreateDevWalletRequestBuilder::new(
    ///     wallet_set_id,
    ///     vec![Blockchain::EthSepolia]
    /// )?
    /// .account_type(AccountType::Sca)
    /// .count(1)
    /// .name("My Wallet".to_string())
    /// .build();
    ///
    /// let response = ops.create_dev_wallet(builder).await?;
    /// println!("Created wallet: {}", response.wallets[0].address);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_dev_wallet(
        &self,
        builder: CreateDevWalletRequestBuilder,
    ) -> CircleResult<DevWalletsResponse> {
        // Encrypt the entity secret (fresh encryption for each request)
        let entity_secret_ciphertext = self.entity_secret()?;

        // Generate a new UUID for each request (or use custom one if provided)
        let idempotency_key = Uuid::new_v4().to_string();

        let request = CreateDevWalletRequest {
            wallet_set_id: builder.wallet_set_id,
            entity_secret_ciphertext,
            blockchains: builder.blockchains,
            idempotency_key,
            account_type: builder.account_type,
            count: builder.count,
            metadata: builder.metadata,
            name: builder.name,
            ref_id: builder.ref_id,
        };

        self.post("/v1/w3s/developer/wallets", &request).await
    }

    /// Update a wallet
    ///
    /// Updates wallet metadata such as name and reference ID
    pub async fn update_dev_wallet(
        &self,
        wallet_id: &str,
        request: UpdateDevWalletRequest,
    ) -> CircleResult<DevWalletResponse> {
        let path = format!("/v1/w3s/wallets/{}", wallet_id);
        self.put(&path, &request).await
    }

    /// Sign a message
    ///
    /// Cryptographically signs a message using a wallet's private key.
    /// This is useful for proving ownership or authenticating actions.
    ///
    /// # Arguments
    ///
    /// * `builder` - A `SignMessageRequestBuilder` with the message and wallet ID
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_ops::circler_ops::CircleOps;
    /// use inf_circle_sdk::dev_wallet::ops::sign_message::SignMessageRequestBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let ops = CircleOps::new()?;
    ///
    /// let builder = SignMessageRequestBuilder::new(
    ///     "wallet-id".to_string(),
    ///     "Hello, World!".to_string()
    /// )?
    /// .encoded_by_hex(false)
    /// .build();
    ///
    /// let response = ops.dev_sign_message(builder).await?;
    /// println!("Signature: {}", response.signature);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn dev_sign_message(
        &self,
        builder: SignMessageRequestBuilder,
    ) -> CircleResult<SignatureResponse> {
        let entity_secret_ciphertext = self.entity_secret()?;

        let request = SignMessageRequest {
            entity_secret_ciphertext,
            message: builder.message,
            wallet_id: builder.wallet_id,
            encoded_by_hex: builder.encoded_by_hex,
            memo: builder.memo,
        };

        let path = format!("/v1/w3s/developer/sign/message");
        self.post(&path, &request).await
    }

    /// Sign typed data (EIP-712)
    ///
    /// Signs structured data according to the EIP-712 standard.
    /// This is commonly used for signing transactions in a human-readable format.
    ///
    /// # Arguments
    ///
    /// * `builder` - A `SignDataRequestBuilder` with the typed data and wallet ID
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_ops::circler_ops::CircleOps;
    /// use inf_circle_sdk::dev_wallet::ops::sign_data::SignDataRequestBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let ops = CircleOps::new()?;
    ///
    /// let typed_data = serde_json::json!({
    ///     "types": {
    ///         "EIP712Domain": [
    ///             {"name": "name", "type": "string"},
    ///             {"name": "version", "type": "string"}
    ///         ]
    ///     },
    ///     "domain": {"name": "MyApp", "version": "1"}
    /// });
    ///
    /// let typed_data_str = serde_json::to_string(&typed_data)?;
    /// let builder = SignDataRequestBuilder::new(
    ///     "wallet-id".to_string(),
    ///     typed_data_str
    /// )?
    /// .build();
    ///
    /// let response = ops.dev_sign_data(builder).await?;
    /// println!("Signature: {}", response.signature);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn dev_sign_data(
        &self,
        builder: SignDataRequestBuilder,
    ) -> CircleResult<SignatureResponse> {
        let entity_secret_ciphertext = self.entity_secret()?;

        let request = SignDataRequest {
            entity_secret_ciphertext,
            data: builder.data,
            wallet_id: builder.wallet_id,
            memo: builder.memo,
        };

        let path = format!("/v1/w3s/developer/sign/typedData");
        self.post(&path, &request).await
    }

    /// Sign a transaction
    ///
    /// Signs a raw blockchain transaction using a wallet's private key.
    /// Use this for custom transactions that aren't covered by the standard transaction types.
    ///
    /// # Arguments
    ///
    /// * `builder` - A `SignTransactionRequestBuilder` with the transaction data
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_ops::circler_ops::CircleOps;
    /// use inf_circle_sdk::dev_wallet::ops::sign_transaction::SignTransactionRequestBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let ops = CircleOps::new()?;
    ///
    /// let raw_tx = serde_json::json!({
    ///     "to": "0x...",
    ///     "value": "1000000000000000",
    ///     "data": "0x"
    /// });
    ///
    /// let builder = SignTransactionRequestBuilder::new(
    ///     "wallet-id".to_string(),
    ///     None,  // raw_transaction
    ///     Some(serde_json::to_string(&raw_tx)?)  // transaction
    /// )?
    /// .build();
    ///
    /// let response = ops.dev_sign_transaction(builder).await?;
    /// println!("Signed transaction: {}", response.signature);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn dev_sign_transaction(
        &self,
        builder: SignTransactionRequestBuilder,
    ) -> CircleResult<SignTransactionResponse> {
        let entity_secret_ciphertext = self.entity_secret()?;

        let request = SignTransactionRequest {
            entity_secret_ciphertext,
            raw_transaction: builder.raw_transaction,
            transaction: builder.transaction,
            wallet_id: builder.wallet_id,
            memo: builder.memo,
        };

        let path = format!("/v1/w3s/developer/sign/transaction");
        self.post(&path, &request).await
    }

    /// Sign a delegate action (NEAR Protocol)
    ///
    /// Signs a delegate action for NEAR Protocol, allowing meta-transactions
    /// where one account can execute actions on behalf of another.
    ///
    /// # Arguments
    ///
    /// * `builder` - A `SignDelegateRequestBuilder` with the delegate action data
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_ops::circler_ops::CircleOps;
    /// use inf_circle_sdk::dev_wallet::ops::sign_delegate::SignDelegateRequestBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let ops = CircleOps::new()?;
    ///
    /// let delegate_action = serde_json::json!({
    ///     "delegateAction": {
    ///         "senderId": "alice.near",
    ///         "receiverId": "contract.near"
    ///     }
    /// });
    ///
    /// let delegate_str = serde_json::to_string(&delegate_action)?;
    /// let builder = SignDelegateRequestBuilder::new(
    ///     "wallet-id".to_string(),
    ///     delegate_str
    /// )?
    /// .build();
    ///
    /// let response = ops.dev_sign_delegate(builder).await?;
    /// println!("Signed delegate action");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn dev_sign_delegate(
        &self,
        builder: SignDelegateRequestBuilder,
    ) -> CircleResult<SignDelegateResponse> {
        let entity_secret_ciphertext = self.entity_secret()?;

        let request = SignDelegateRequest {
            entity_secret_ciphertext,
            unsigned_delegate_action: builder.unsigned_delegate_action,
            wallet_id: builder.wallet_id,
        };

        let path = format!("/v1/w3s/developer/sign/delegateAction");
        self.post(&path, &request).await
    }

    /// Create a transfer transaction
    ///
    /// Creates a transaction to transfer native tokens, ERC-20 tokens, or NFTs
    /// from a wallet to another address.
    ///
    /// # Arguments
    ///
    /// * `builder` - A `CreateTransferTransactionRequestBuilder` with transfer details
    ///
    /// # Returns
    ///
    /// Returns transaction details including the transaction ID and state.
    ///
    /// # Example - Native Token Transfer
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_ops::circler_ops::CircleOps;
    /// use inf_circle_sdk::dev_wallet::ops::create_transfer_transaction::CreateTransferTransactionRequestBuilder;
    /// use inf_circle_sdk::dev_wallet::dto::FeeLevel;
    /// use inf_circle_sdk::types::Blockchain;
    /// use uuid::Uuid;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let ops = CircleOps::new()?;
    ///
    /// let builder = CreateTransferTransactionRequestBuilder::new("wallet-id".to_string())
    ///     .destination_address("0x1234...".to_string())
    ///     .amounts(vec!["0.1".to_string()])  // 0.1 ETH
    ///     .blockchain(Blockchain::EthSepolia)
    ///     .fee_level(FeeLevel::Medium)
    ///     .idempotency_key(Uuid::new_v4().to_string())
    ///     .build();
    ///
    /// let response = ops.create_dev_transfer_transaction(builder).await?;
    /// println!("Transaction ID: {}", response.id);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Example - ERC-20 Token Transfer
    ///
    /// ```rust,no_run
    /// # use inf_circle_sdk::circle_ops::circler_ops::CircleOps;
    /// # use inf_circle_sdk::dev_wallet::ops::create_transfer_transaction::CreateTransferTransactionRequestBuilder;
    /// # use inf_circle_sdk::dev_wallet::dto::FeeLevel;
    /// # use inf_circle_sdk::types::Blockchain;
    /// # use uuid::Uuid;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let ops = CircleOps::new()?;
    /// let builder = CreateTransferTransactionRequestBuilder::new("wallet-id".to_string())
    ///     .destination_address("0x1234...".to_string())
    ///     .amounts(vec!["10.0".to_string()])  // 10 USDC
    ///     .token_address("0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238".to_string())  // USDC
    ///     .blockchain(Blockchain::EthSepolia)
    ///     .fee_level(FeeLevel::High)
    ///     .idempotency_key(Uuid::new_v4().to_string())
    ///     .build();
    ///
    /// let response = ops.create_dev_transfer_transaction(builder).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_dev_transfer_transaction(
        &self,
        builder: CreateTransferTransactionRequestBuilder,
    ) -> CircleResult<CreateTransferTransactionResponse> {
        let entity_secret_ciphertext = self.entity_secret()?;

        let request = CreateTransferTransactionRequest {
            entity_secret_ciphertext,
            wallet_id: builder.wallet_id,
            destination_address: builder.destination_address,
            amounts: builder.amounts,
            nft_token_ids: builder.nft_token_ids,
            token_id: builder.token_id,
            token_address: builder.token_address,
            idempotency_key: builder.idempotency_key,
            ref_id: builder.ref_id,
            blockchain: builder.blockchain,
            gas_limit: builder.gas_limit,
            gas_price: builder.gas_price,
            max_fee: builder.max_fee,
            priority_fee: builder.priority_fee,
            fee_level: builder.fee_level,
        };

        let path = format!("/v1/w3s/developer/transactions/transfer");
        self.post(&path, &request).await
    }

    /// Query a contract
    ///
    /// Execute a query function on a contract by providing the address and blockchain
    pub async fn dev_query_contract(
        &self,
        request: QueryContractRequest,
    ) -> CircleResult<QueryContractResponse> {
        self.post("/v1/w3s/contracts/query", &request).await
    }

    /// Create a contract execution transaction
    ///
    /// Creates a transaction that calls a smart contract function. Use this to interact
    /// with deployed smart contracts (e.g., minting NFTs, swapping tokens, etc.).
    ///
    /// # Arguments
    ///
    /// * `builder` - A `CreateContractExecutionTransactionRequestBuilder` with contract call details
    ///
    /// # Returns
    ///
    /// Returns transaction details including the transaction ID and state.
    ///
    /// # Example - Call ERC-20 Approve Function
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_ops::circler_ops::CircleOps;
    /// use inf_circle_sdk::dev_wallet::ops::create_contract_transaction::CreateContractExecutionTransactionRequestBuilder;
    /// use inf_circle_sdk::dev_wallet::dto::FeeLevel;
    /// use uuid::Uuid;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let ops = CircleOps::new()?;
    ///
    /// use inf_circle_sdk::dev_wallet::dto::AbiParameter;
    ///
    /// let builder = CreateContractExecutionTransactionRequestBuilder::new(
    ///     "wallet-id".to_string(),
    ///     "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238".to_string(),
    ///     Uuid::new_v4().to_string()
    /// )
    /// .abi_function_signature("approve(address,uint256)".to_string())
    /// .abi_parameters(vec![
    ///     AbiParameter::String("0xSpenderAddress...".to_string()),
    ///     AbiParameter::String("1000000".to_string())
    /// ])
    /// .fee_level(FeeLevel::Medium)
    /// .build();
    ///
    /// let response = ops.create_dev_contract_execution_transaction(builder).await?;
    /// println!("Transaction ID: {}", response.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_dev_contract_execution_transaction(
        &self,
        builder: CreateContractExecutionTransactionRequestBuilder,
    ) -> CircleResult<CreateContractExecutionTransactionResponse> {
        let entity_secret_ciphertext = self.entity_secret()?;

        let request = CreateContractExecutionTransactionRequest {
            wallet_id: builder.wallet_id,
            entity_secret_ciphertext,
            contract_address: builder.contract_address,
            idempotency_key: builder.idempotency_key,
            abi_function_signature: builder.abi_function_signature,
            abi_parameters: builder.abi_parameters,
            call_data: builder.call_data,
            amount: builder.amount,
            fee_level: builder.fee_level,
            gas_limit: builder.gas_limit,
            gas_price: builder.gas_price,
            max_fee: builder.max_fee,
            priority_fee: builder.priority_fee,
            ref_id: builder.ref_id,
        };

        self.post("/v1/w3s/developer/transactions/contractExecution", &request)
            .await
    }

    /// Create a wallet upgrade transaction
    ///
    /// Creates a transaction which upgrades a wallet to a new SCA core version
    pub async fn create_dev_wallet_upgrade_transaction(
        &self,
        builder: CreateWalletUpgradeTransactionRequestBuilder,
    ) -> CircleResult<CreateWalletUpgradeTransactionResponse> {
        let entity_secret_ciphertext = self.entity_secret()?;

        let request = CreateWalletUpgradeTransactionRequest {
            wallet_id: builder.wallet_id,
            entity_secret_ciphertext,
            new_sca_core: builder.new_sca_core.as_str().to_string(),
            idempotency_key: builder.idempotency_key,
            fee_level: builder.fee_level,
            gas_limit: builder.gas_limit,
            gas_price: builder.gas_price,
            max_fee: builder.max_fee,
            priority_fee: builder.priority_fee,
            ref_id: builder.ref_id,
        };

        self.post("/v1/w3s/developer/transactions/walletUpgrade", &request)
            .await
    }

    /// Cancel a transaction
    ///
    /// Cancels a pending transaction by submitting a replacement transaction with higher gas fees.
    /// This is a best-effort operation and won't be effective if the original transaction
    /// has already been confirmed by the blockchain. Gas fees may still be incurred.
    ///
    /// # Arguments
    ///
    /// * `builder` - A `CancelTransactionRequestBuilder` with the transaction ID to cancel
    ///
    /// # Returns
    ///
    /// Returns the cancellation transaction details.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_ops::circler_ops::CircleOps;
    /// use inf_circle_sdk::dev_wallet::ops::cancel_transaction::CancelTransactionRequestBuilder;
    /// use uuid::Uuid;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let ops = CircleOps::new()?;
    ///
    /// let builder = CancelTransactionRequestBuilder::new(
    ///     "transaction-id-to-cancel".to_string(),
    ///     Uuid::new_v4().to_string()
    /// )
    /// .build();
    ///
    /// let response = ops.cancel_dev_transaction(builder).await?;
    /// println!("Cancellation transaction ID: {}", response.id);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// Only pending transactions can be cancelled. Confirmed transactions cannot be reversed.
    pub async fn cancel_dev_transaction(
        &self,
        builder: CancelTransactionRequestBuilder,
    ) -> CircleResult<CancelTransactionResponse> {
        let entity_secret_ciphertext = self.entity_secret()?;

        let request = CancelTransactionRequest {
            entity_secret_ciphertext,
            idempotency_key: builder.idempotency_key,
        };

        let path = format!(
            "/v1/w3s/developer/transactions/{}/cancel",
            builder.transaction_id
        );
        self.post(&path, &request).await
    }

    /// Accelerate a transaction
    ///
    /// Speeds up a pending transaction by replacing it with a higher gas fee transaction.
    /// This is useful when a transaction is taking too long to confirm. Additional gas fees
    /// will be incurred.
    ///
    /// # Arguments
    ///
    /// * `builder` - An `AccelerateTransactionRequestBuilder` with the transaction ID to accelerate
    ///
    /// # Returns
    ///
    /// Returns the accelerated transaction details.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_ops::circler_ops::CircleOps;
    /// use inf_circle_sdk::dev_wallet::ops::accelerate_transaction::AccelerateTransactionRequestBuilder;
    /// use uuid::Uuid;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let ops = CircleOps::new()?;
    ///
    /// let builder = AccelerateTransactionRequestBuilder::new(
    ///     "slow-transaction-id".to_string(),
    ///     Uuid::new_v4().to_string()
    /// )
    /// .build();
    ///
    /// let response = ops.accelerate_dev_transaction(builder).await?;
    /// println!("Accelerated transaction ID: {}", response.id);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// Only pending transactions can be accelerated. Confirmed transactions cannot be modified.
    pub async fn accelerate_dev_transaction(
        &self,
        builder: AccelerateTransactionRequestBuilder,
    ) -> CircleResult<AccelerateTransactionResponse> {
        let entity_secret_ciphertext = self.entity_secret()?;

        let request = AccelerateTransactionRequest {
            entity_secret_ciphertext,
            idempotency_key: builder.idempotency_key,
        };

        let path = format!(
            "/v1/w3s/developer/transactions/{}/accelerate",
            builder.transaction_id
        );
        self.post(&path, &request).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dev_wallet::{dto::AccountType, ops::create_dev_wallet::CreateDevWalletRequestBuilder},
        types::Blockchain,
    };

    #[test]
    fn test_builder_pattern() {
        // Test that the builder pattern works correctly
        let builder = CreateDevWalletRequestBuilder::new(
            "test-wallet-set-id".to_string(),
            vec![Blockchain::EthSepolia],
        )
        .unwrap()
        .account_type(AccountType::Sca)
        .count(5)
        .name("Test Wallet".to_string())
        .build();

        assert_eq!(builder.wallet_set_id, "test-wallet-set-id");
        assert_eq!(builder.blockchains, vec![Blockchain::EthSepolia]);
        assert_eq!(builder.account_type, Some("SCA".to_string()));
        assert_eq!(builder.count, Some(5));
        assert_eq!(builder.name, Some("Test Wallet".to_string()));
    }

    #[test]
    fn test_builder_with_custom_idempotency_key() {
        // Test that custom idempotency keys are preserved in the builder
        let custom_key = "custom-test-key-123";
        let builder = CreateDevWalletRequestBuilder::new(
            "test-wallet-set-id".to_string(),
            vec![Blockchain::EthSepolia],
        )
        .unwrap()
        .idempotency_key(custom_key.to_string())
        .build();

        assert_eq!(builder.idempotency_key, Some(custom_key.to_string()));
    }
}
