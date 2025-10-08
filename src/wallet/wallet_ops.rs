//! Wallet write operations for CircleOps

use crate::{
    circle_ops::circler_ops::CircleOps,
    helper::CircleResult,
    wallet::{
        dto::{
            CreateTransferTransactionRequest, CreateTransferTransactionResponse,
            CreateWalletRequest, SignDataRequest, SignDelegateRequest, SignDelegateResponse,
            SignMessageRequest, SignTransactionRequest, SignTransactionResponse, SignatureResponse,
            UpdateWalletRequest, WalletResponse, WalletsResponse,
        },
        ops::{
            create_transfer_transaction::CreateTransferTransactionRequestBuilder,
            create_wallet::CreateWalletRequestBuilder, sign_data::SignDataRequestBuilder,
            sign_delegate::SignDelegateRequestBuilder, sign_message::SignMessageRequestBuilder,
            sign_transaction::SignTransactionRequestBuilder,
        },
    },
};
use uuid::Uuid;

impl CircleOps {
    /// Create new wallets
    ///
    /// Creates a new developer-controlled wallet or batch of wallets within a wallet set
    /// Automatically encrypts the entity secret and generates a unique UUID for each request
    pub async fn create_wallet(
        &self,
        builder: CreateWalletRequestBuilder,
    ) -> CircleResult<WalletsResponse> {
        // Encrypt the entity secret (fresh encryption for each request)
        let entity_secret_ciphertext = self.entity_secret()?;

        // Generate a new UUID for each request (or use custom one if provided)
        let idempotency_key = Uuid::new_v4().to_string();

        let request = CreateWalletRequest {
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
    pub async fn update_wallet(
        &self,
        wallet_id: &str,
        request: UpdateWalletRequest,
    ) -> CircleResult<WalletResponse> {
        let path = format!("/v1/w3s/wallets/{}", wallet_id);
        self.put(&path, &request).await
    }

    /// sign a message
    pub async fn sign_message(
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

    /// sign a data
    pub async fn sign_data(
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

    /// sign a transaction
    pub async fn sign_transaction(
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

    /// sign a delegate action
    pub async fn sign_delegate(
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

    /// create a transfer transaction
    pub async fn create_transfer_transaction(
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
}

#[cfg(test)]
mod tests {
    use crate::wallet::{
        dto::{AccountType, Blockchain},
        ops::create_wallet::CreateWalletRequestBuilder,
    };

    #[test]
    fn test_builder_pattern() {
        // Test that the builder pattern works correctly
        let builder = CreateWalletRequestBuilder::new(
            "test-wallet-set-id".to_string(),
            vec![Blockchain::EthSepolia],
        )
        .unwrap()
        .account_type(AccountType::Sca)
        .count(5)
        .name("Test Wallet".to_string())
        .build();

        assert_eq!(builder.wallet_set_id, "test-wallet-set-id");
        assert_eq!(builder.blockchains, vec!["ETH-SEPOLIA"]);
        assert_eq!(builder.account_type, Some("SCA".to_string()));
        assert_eq!(builder.count, Some(5));
        assert_eq!(builder.name, Some("Test Wallet".to_string()));
    }

    #[test]
    fn test_builder_with_custom_idempotency_key() {
        // Test that custom idempotency keys are preserved in the builder
        let custom_key = "custom-test-key-123";
        let builder = CreateWalletRequestBuilder::new(
            "test-wallet-set-id".to_string(),
            vec![Blockchain::EthSepolia],
        )
        .unwrap()
        .idempotency_key(custom_key.to_string())
        .build();

        assert_eq!(builder.idempotency_key, Some(custom_key.to_string()));
    }
}
