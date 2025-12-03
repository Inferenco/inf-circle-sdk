use crate::dev_wallet::dto::TransactionParams;

/// Builder for creating transaction query parameters
///
/// This builder helps construct parameters for querying transactions by type.
///
/// # Example
///
/// ```rust,no_run
/// use inf_circle_sdk::dev_wallet::views::transaction::TransactionParamsBuilder;
///
/// let params = TransactionParamsBuilder::new()
///     .tx_type("TRANSFER".to_string())
///     .build();
/// ```
pub struct TransactionParamsBuilder {
    params: TransactionParams,
}

impl TransactionParamsBuilder {
    /// Create a new builder instance
    pub fn new() -> Self {
        Self {
            params: TransactionParams::default(),
        }
    }

    /// Set the transaction type filter
    pub fn tx_type(mut self, tx_type: String) -> Self {
        self.params.tx_type = tx_type;
        self
    }

    /// Build the transaction parameters
    pub fn build(self) -> TransactionParams {
        self.params
    }
}
