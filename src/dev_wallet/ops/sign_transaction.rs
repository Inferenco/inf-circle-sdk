use crate::helper::CircleResult;

/// Builder for creating transaction signing requests
///
/// This builder helps construct requests to sign raw transactions or transaction objects
/// with a wallet's private key.
///
/// # Example
///
/// ```rust,no_run
/// use inf_circle_sdk::dev_wallet::ops::sign_transaction::SignTransactionRequestBuilder;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let builder = SignTransactionRequestBuilder::new(
///     "wallet-id".to_string(),
///     Some("0x...".to_string()), // raw_transaction
///     None
/// )?
/// .memo("Signing transaction".to_string())
/// .build();
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct SignTransactionRequestBuilder {
    pub(crate) wallet_id: String,
    pub(crate) raw_transaction: Option<String>,
    pub(crate) transaction: Option<String>,
    pub(crate) memo: Option<String>,
}

impl SignTransactionRequestBuilder {
    /// Create a new builder instance
    ///
    /// # Arguments
    /// * `wallet_id` - The wallet ID to sign with
    /// * `raw_transaction` - Optional raw transaction hex string
    /// * `transaction` - Optional transaction object JSON string
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

    /// Set the wallet ID
    pub fn wallet_id(mut self, wallet_id: String) -> Self {
        self.wallet_id = wallet_id;
        self
    }

    /// Set the raw transaction hex string
    pub fn raw_transaction(mut self, raw_transaction: String) -> Self {
        self.raw_transaction = Some(raw_transaction);
        self
    }

    /// Set the transaction object JSON string
    pub fn transaction(mut self, transaction: String) -> Self {
        self.transaction = Some(transaction);
        self
    }

    /// Set an optional memo for the signing request
    pub fn memo(mut self, memo: String) -> Self {
        self.memo = Some(memo);
        self
    }

    /// Build the sign transaction request
    pub fn build(self) -> SignTransactionRequestBuilder {
        self
    }
}
