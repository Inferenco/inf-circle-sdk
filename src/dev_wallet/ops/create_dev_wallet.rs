use crate::dev_wallet::dto::{AccountType, DevWalletMetadata};
use crate::helper::CircleResult;
use crate::types::Blockchain;

/// Builder for creating developer wallet requests
///
/// This builder helps construct requests to create new developer-controlled wallets
/// (EOA or SCA) on one or more blockchains.
///
/// # Example
///
/// ```rust,no_run
/// use inf_circle_sdk::dev_wallet::ops::create_dev_wallet::CreateDevWalletRequestBuilder;
/// use inf_circle_sdk::dev_wallet::dto::AccountType;
/// use inf_circle_sdk::types::Blockchain;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let builder = CreateDevWalletRequestBuilder::new(
///     "wallet-set-id".to_string(),
///     vec![Blockchain::EthSepolia, Blockchain::AvaxFuji]
/// )?
/// .account_type(AccountType::Sca)
/// .count(1)
/// .name("My Wallet".to_string())
/// .ref_id("wallet-ref-123".to_string())
/// .build();
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct CreateDevWalletRequestBuilder {
    pub(crate) wallet_set_id: String,
    pub(crate) blockchains: Vec<Blockchain>,
    pub(crate) account_type: Option<String>,
    pub(crate) count: Option<u32>,
    pub(crate) metadata: Option<Vec<DevWalletMetadata>>,
    pub(crate) name: Option<String>,
    pub(crate) ref_id: Option<String>,
    pub(crate) idempotency_key: Option<String>,
}

impl CreateDevWalletRequestBuilder {
    /// Create a new builder
    ///
    /// Entity secret encryption and UUID generation happen at request time for uniqueness.
    ///
    /// # Arguments
    ///
    /// * `wallet_set_id` - The wallet set ID where the wallet will be created
    /// * `blockchains` - List of blockchains to create the wallet on
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::dev_wallet::ops::create_dev_wallet::CreateDevWalletRequestBuilder;
    /// use inf_circle_sdk::types::Blockchain;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let builder = CreateDevWalletRequestBuilder::new(
    ///     "wallet-set-id".to_string(),
    ///     vec![Blockchain::EthSepolia]
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Set account type (EOA or SCA)
    ///
    /// # Arguments
    ///
    /// * `account_type` - The account type (EOA for externally-owned account, SCA for smart contract account)
    pub fn account_type(mut self, account_type: AccountType) -> Self {
        self.account_type = Some(account_type.as_str().to_string());
        self
    }

    /// Set wallet count per blockchain
    ///
    /// # Arguments
    ///
    /// * `count` - Number of wallets to create per blockchain (default: 1)
    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }

    /// Set wallet metadata
    ///
    /// # Arguments
    ///
    /// * `metadata` - Vector of metadata objects to attach to the wallets
    pub fn metadata(mut self, metadata: Vec<DevWalletMetadata>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Set wallet name
    ///
    /// # Arguments
    ///
    /// * `name` - Human-readable name for the wallet
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set reference ID for tracking purposes
    ///
    /// # Arguments
    ///
    /// * `ref_id` - Custom reference ID to associate with the wallet
    pub fn ref_id(mut self, ref_id: String) -> Self {
        self.ref_id = Some(ref_id);
        self
    }

    /// Set custom idempotency key
    ///
    /// # Arguments
    ///
    /// * `key` - Custom idempotency key (if not provided, a UUID will be generated automatically)
    pub fn idempotency_key(mut self, key: String) -> Self {
        self.idempotency_key = Some(key);
        self
    }

    /// Build the request parameters
    ///
    /// Returns the builder data for use by the create_dev_wallet method
    pub fn build(self) -> CreateDevWalletRequestBuilder {
        self
    }
}
