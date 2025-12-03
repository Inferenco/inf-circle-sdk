use crate::helper::CircleResult;

/// Builder for creating data signing requests
///
/// This builder helps construct requests to sign arbitrary data (EIP-712 typed data)
/// with a wallet's private key.
///
/// # Example
///
/// ```rust,no_run
/// use inf_circle_sdk::dev_wallet::ops::sign_data::SignDataRequestBuilder;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let builder = SignDataRequestBuilder::new(
///     "wallet-id".to_string(),
///     r#"{"types": {...}, "domain": {...}, "message": {...}}"#.to_string()
/// )?
/// .memo("Signing typed data".to_string())
/// .build();
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct SignDataRequestBuilder {
    pub(crate) wallet_id: String,
    pub(crate) data: String,
    pub(crate) memo: Option<String>,
}

impl SignDataRequestBuilder {
    /// Create a new builder instance
    ///
    /// # Arguments
    /// * `wallet_id` - The wallet ID to sign with
    /// * `data` - The data to sign (typically EIP-712 typed data JSON)
    pub fn new(wallet_id: String, data: String) -> CircleResult<Self> {
        Ok(Self {
            wallet_id,
            data,
            memo: None,
        })
    }

    /// Set the wallet ID
    pub fn wallet_id(mut self, wallet_id: String) -> Self {
        self.wallet_id = wallet_id;
        self
    }

    /// Set the data to sign
    pub fn data(mut self, data: String) -> Self {
        self.data = data;
        self
    }

    /// Set an optional memo for the signing request
    pub fn memo(mut self, memo: String) -> Self {
        self.memo = Some(memo);
        self
    }

    /// Build the sign data request
    pub fn build(self) -> SignDataRequestBuilder {
        self
    }
}
