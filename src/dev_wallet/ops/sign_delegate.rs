use crate::helper::CircleResult;

/// Builder for creating delegate action signing requests
///
/// This builder helps construct requests to sign NEAR delegate actions (NEP-461)
/// with a wallet's private key.
///
/// # Example
///
/// ```rust,no_run
/// use inf_circle_sdk::dev_wallet::ops::sign_delegate::SignDelegateRequestBuilder;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let builder = SignDelegateRequestBuilder::new(
///     "wallet-id".to_string(),
///     "base64-encoded-delegate-action".to_string()
/// )?
/// .build();
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct SignDelegateRequestBuilder {
    pub(crate) wallet_id: String,
    pub(crate) unsigned_delegate_action: String,
}

impl SignDelegateRequestBuilder {
    /// Create a new builder instance
    ///
    /// # Arguments
    /// * `wallet_id` - The wallet ID to sign with
    /// * `unsigned_delegate_action` - Base64-encoded unsigned delegate action
    pub fn new(wallet_id: String, unsigned_delegate_action: String) -> CircleResult<Self> {
        Ok(Self {
            wallet_id,
            unsigned_delegate_action,
        })
    }

    /// Set the wallet ID
    pub fn wallet_id(mut self, wallet_id: String) -> Self {
        self.wallet_id = wallet_id;
        self
    }

    /// Set the unsigned delegate action (base64-encoded)
    pub fn unsigned_delegate_action(mut self, unsigned_delegate_action: String) -> Self {
        self.unsigned_delegate_action = unsigned_delegate_action;
        self
    }

    /// Build the sign delegate request
    pub fn build(self) -> SignDelegateRequestBuilder {
        self
    }
}
