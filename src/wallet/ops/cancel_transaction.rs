#[derive(Clone, Debug)]
pub struct CancelTransactionRequestBuilder {
    pub transaction_id: String,
    pub idempotency_key: String,
}

impl CancelTransactionRequestBuilder {
    /// Create a new builder with required fields
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
