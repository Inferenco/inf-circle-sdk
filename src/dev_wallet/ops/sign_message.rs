use crate::helper::CircleResult;

/// Builder for creating message signing requests
///
/// This builder helps construct requests to sign messages with a wallet's private key.
/// The message can be signed as-is or encoded as hex.
///
/// # Example
///
/// ```rust,no_run
/// use inf_circle_sdk::dev_wallet::ops::sign_message::SignMessageRequestBuilder;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let builder = SignMessageRequestBuilder::new(
///     "wallet-id".to_string(),
///     "Hello, World!".to_string()
/// )?
/// .encoded_by_hex(false)
/// .memo("Test message".to_string())
/// .build();
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct SignMessageRequestBuilder {
    pub(crate) wallet_id: String,
    pub(crate) message: String,
    pub(crate) encoded_by_hex: Option<bool>,
    pub(crate) memo: Option<String>,
}

impl SignMessageRequestBuilder {
    /// Create a new builder instance
    ///
    /// # Arguments
    /// * `wallet_id` - The wallet ID to sign with
    /// * `message` - The message to sign
    pub fn new(wallet_id: String, message: String) -> CircleResult<Self> {
        Ok(Self {
            wallet_id,
            message,
            encoded_by_hex: None,
            memo: None,
        })
    }

    /// Set the wallet ID
    pub fn wallet_id(mut self, wallet_id: String) -> Self {
        self.wallet_id = wallet_id;
        self
    }

    /// Set whether the message is hex-encoded
    pub fn encoded_by_hex(mut self, encoded_by_hex: bool) -> Self {
        self.encoded_by_hex = Some(encoded_by_hex);
        self
    }

    /// Set an optional memo for the signing request
    pub fn memo(mut self, memo: String) -> Self {
        self.memo = Some(memo);
        self
    }

    /// Build the sign message request
    pub fn build(self) -> SignMessageRequestBuilder {
        self
    }
}
