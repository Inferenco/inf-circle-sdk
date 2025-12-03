/// Builder for creating accelerate transaction requests
///
/// This builder helps construct requests to accelerate pending transactions by replacing
/// them with higher gas fees.
///
/// # Example
///
/// ```rust,no_run
/// use inf_circle_sdk::dev_wallet::ops::accelerate_transaction::AccelerateTransactionRequestBuilder;
/// use uuid::Uuid;
///
/// let builder = AccelerateTransactionRequestBuilder::new(
///     "transaction-id".to_string(),
///     Uuid::new_v4().to_string()
/// ).build();
/// ```
#[derive(Clone, Debug)]
pub struct AccelerateTransactionRequestBuilder {
    pub transaction_id: String,
    pub idempotency_key: String,
}

impl AccelerateTransactionRequestBuilder {
    /// Create a new builder with required fields
    ///
    /// # Arguments
    /// * `transaction_id` - The ID of the transaction to accelerate
    /// * `idempotency_key` - Unique identifier for the request
    pub fn new(transaction_id: String, idempotency_key: String) -> Self {
        Self {
            transaction_id,
            idempotency_key,
        }
    }

    /// Build the AccelerateTransactionRequestBuilder
    pub fn build(self) -> AccelerateTransactionRequestBuilder {
        self
    }
}
