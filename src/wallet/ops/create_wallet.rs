use crate::helper::CircleResult;
use crate::types::Blockchain;
use crate::wallet::dto::{AccountType, WalletMetadata};

/// Builder for CreateWalletsRequest
#[derive(Clone, Debug)]
pub struct CreateWalletRequestBuilder {
    pub(crate) wallet_set_id: String,
    pub(crate) blockchains: Vec<Blockchain>,
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

        Ok(Self {
            wallet_set_id,
            blockchains,
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
