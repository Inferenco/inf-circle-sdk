//! Wallet write operations for CircleOps

use crate::{
    circle_ops::circler_ops::CircleOps,
    helper::{encrypt_entity_secret, get_env_var, CircleResult},
    wallet::dto::{
        AccountType, Blockchain, CreateWalletRequest, UpdateWalletRequest, WalletMetadata,
        WalletResponse, WalletsResponse,
    },
    CircleError,
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
        let entity_secret = builder.entity_secret.clone();
        let public_key = builder.public_key.clone();

        // Encrypt the entity secret (fresh encryption for each request)
        let entity_secret_ciphertext = encrypt_entity_secret(&entity_secret, &public_key)
            .map_err(|e| CircleError::Config(format!("Failed to encrypt entity secret: {}", e)))?;

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
    pub(crate) entity_secret: String,
    pub(crate) public_key: String,
}

impl CreateWalletRequestBuilder {
    /// Create a new builder
    ///
    /// Entity secret encryption and UUID generation happen at request time for uniqueness
    pub fn new(wallet_set_id: String, blockchains: Vec<Blockchain>) -> CircleResult<Self> {
        dotenv::dotenv().ok();
        let blockchain_strings: Vec<String> =
            blockchains.iter().map(|b| b.as_str().to_string()).collect();

        let entity_secret = get_env_var("CIRCLE_ENTITY_SECRET")?;
        let public_key = get_env_var("CIRCLE_PUBLIC_KEY")?;

        Ok(Self {
            wallet_set_id,
            blockchains: blockchain_strings,
            account_type: None,
            count: None,
            metadata: None,
            name: None,
            ref_id: None,
            idempotency_key: None,
            entity_secret,
            public_key,
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
