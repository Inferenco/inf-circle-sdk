/// Builder for creating cancel transaction requests
///
/// This builder helps construct requests to cancel pending transactions.
///
/// # Example
///
/// ```rust,no_run
/// use inf_circle_sdk::dev_wallet::ops::cancel_transaction::CancelTransactionRequestBuilder;
/// use uuid::Uuid;
///
/// let builder = CancelTransactionRequestBuilder::new(
///     "transaction-id".to_string(),
///     Uuid::new_v4().to_string()
/// ).build();
/// ```
#[derive(Clone, Debug)]
pub struct CancelTransactionRequestBuilder {
    pub transaction_id: String,
    pub idempotency_key: String,
}

impl CancelTransactionRequestBuilder {
    /// Create a new builder with required fields
    ///
    /// # Arguments
    /// * `transaction_id` - The ID of the transaction to cancel
    /// * `idempotency_key` - Unique identifier for the request
    pub fn new(transaction_id: String, idempotency_key: String) -> Self {
        Self {
            transaction_id,
            idempotency_key,
        }
    }

    /// Build the CancelTransactionRequestBuilder
    pub fn build(self) -> CancelTransactionRequestBuilder {
        self
    }
}
