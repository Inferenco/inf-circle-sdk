#[derive(Clone, Debug)]
pub struct AccelerateTransactionRequestBuilder {
    pub transaction_id: String,
    pub idempotency_key: String,
}

impl AccelerateTransactionRequestBuilder {
    /// Create a new builder with required fields
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
