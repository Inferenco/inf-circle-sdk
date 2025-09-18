//! Wallet write operations for CircleOps

use crate::{
    circle_ops::circler_ops::CircleOps,
    helper::CircleResult,
    wallet::dto::{
        AccountType, Blockchain, CreateWalletRequest, SignDataRequest, SignDelegateRequest,
        SignDelegateResponse, SignMessageRequest, SignTransactionRequest, SignTransactionResponse,
        SignatureResponse, UpdateWalletRequest, WalletMetadata, WalletResponse, WalletsResponse,
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
}

/// Builder for CreateWalletsRequest
#[derive(Clone, Debug)]
pub struct CreateWalletRequestBuilder {
    pub(crate) wallet_set_id: String,
    pub(crate) blockchains: Vec<String>,
    pub(crate) account_type: Option<String>,
    pub(crate) count: Option<u32>,
    pub(crate) metadata: Option<Vec<WalletMetadata>>,
    pub(crate) name: Option<String>,
    pub(crate) ref_id: Option<String>,
    pub(crate) idempotency_key: Option<String>,
}

impl CreateWalletRequestBuilder {
    /// Create a new builder
    ///
    /// Entity secret encryption and UUID generation happen at request time for uniqueness
    pub fn new(wallet_set_id: String, blockchains: Vec<Blockchain>) -> CircleResult<Self> {
        dotenv::dotenv().ok();
        let blockchain_strings: Vec<String> =
            blockchains.iter().map(|b| b.as_str().to_string()).collect();

        Ok(Self {
            wallet_set_id,
            blockchains: blockchain_strings,
            account_type: None,
            count: None,
            metadata: None,
            name: None,
            ref_id: None,
            idempotency_key: None,
        })
    }

    /// Set account type
    pub fn account_type(mut self, account_type: AccountType) -> Self {
        self.account_type = Some(account_type.as_str().to_string());
        self
    }

    /// Set wallet count per blockchain
    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }

    /// Set wallet metadata
    pub fn metadata(mut self, metadata: Vec<WalletMetadata>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Set wallet name
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
    /// Returns the builder data for use by the create_wallet method
    pub fn build(self) -> CreateWalletRequestBuilder {
        self
    }
}

#[derive(Clone, Debug)]
pub struct SignMessageRequestBuilder {
    pub(crate) wallet_id: String,
    pub(crate) message: String,
    pub(crate) encoded_by_hex: Option<bool>,
    pub(crate) memo: Option<String>,
}

impl SignMessageRequestBuilder {
    pub fn new(wallet_id: String, message: String) -> CircleResult<Self> {
        Ok(Self {
            wallet_id,
            message,
            encoded_by_hex: None,
            memo: None,
        })
    }

    pub fn wallet_id(mut self, wallet_id: String) -> Self {
        self.wallet_id = wallet_id;
        self
    }

    pub fn encoded_by_hex(mut self, encoded_by_hex: bool) -> Self {
        self.encoded_by_hex = Some(encoded_by_hex);
        self
    }

    pub fn memo(mut self, memo: String) -> Self {
        self.memo = Some(memo);
        self
    }

    pub fn build(self) -> SignMessageRequestBuilder {
        self
    }
}

#[derive(Clone, Debug)]
pub struct SignDataRequestBuilder {
    pub(crate) wallet_id: String,
    pub(crate) data: String,
    pub(crate) memo: Option<String>,
}

impl SignDataRequestBuilder {
    pub fn new(wallet_id: String, data: String) -> CircleResult<Self> {
        Ok(Self {
            wallet_id,
            data,
            memo: None,
        })
    }
}

impl SignDataRequestBuilder {
    pub fn wallet_id(mut self, wallet_id: String) -> Self {
        self.wallet_id = wallet_id;
        self
    }

    pub fn data(mut self, data: String) -> Self {
        self.data = data;
        self
    }

    pub fn memo(mut self, memo: String) -> Self {
        self.memo = Some(memo);
        self
    }

    pub fn build(self) -> SignDataRequestBuilder {
        self
    }
}

#[derive(Clone, Debug)]
pub struct SignDelegateRequestBuilder {
    pub(crate) wallet_id: String,
    pub(crate) unsigned_delegate_action: String,
}

impl SignDelegateRequestBuilder {
    pub fn new(wallet_id: String, unsigned_delegate_action: String) -> CircleResult<Self> {
        Ok(Self {
            wallet_id,
            unsigned_delegate_action,
        })
    }

    pub fn wallet_id(mut self, wallet_id: String) -> Self {
        self.wallet_id = wallet_id;
        self
    }

    pub fn unsigned_delegate_action(mut self, unsigned_delegate_action: String) -> Self {
        self.unsigned_delegate_action = unsigned_delegate_action;
        self
    }

    pub fn build(self) -> SignDelegateRequestBuilder {
        self
    }
}

#[derive(Clone, Debug)]
pub struct SignTransactionRequestBuilder {
    pub(crate) wallet_id: String,
    pub(crate) raw_transaction: Option<String>,
    pub(crate) transaction: Option<String>,
    pub(crate) memo: Option<String>,
}

impl SignTransactionRequestBuilder {
    pub fn new(
        wallet_id: String,
        raw_transaction: Option<String>,
        transaction: Option<String>,
    ) -> CircleResult<Self> {
        Ok(Self {
            wallet_id,
            raw_transaction,
            transaction,
            memo: None,
        })
    }

    pub fn wallet_id(mut self, wallet_id: String) -> Self {
        self.wallet_id = wallet_id;
        self
    }

    pub fn raw_transaction(mut self, raw_transaction: String) -> Self {
        self.raw_transaction = Some(raw_transaction);
        self
    }

    pub fn transaction(mut self, transaction: String) -> Self {
        self.transaction = Some(transaction);
        self
    }

    pub fn memo(mut self, memo: String) -> Self {
        self.memo = Some(memo);
        self
    }

    pub fn build(self) -> SignTransactionRequestBuilder {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
