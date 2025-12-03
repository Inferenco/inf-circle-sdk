use chrono::{DateTime, Utc};

use crate::{dev_wallet::dto::ListTransactionsParams, helper::PaginationParams};

/// Builder for creating list transactions query parameters
///
/// This builder helps construct parameters for querying and filtering transactions.
///
/// # Example
///
/// ```rust,no_run
/// use inf_circle_sdk::dev_wallet::views::list_transactions::ListTransactionsParamsBuilder;
///
/// let params = ListTransactionsParamsBuilder::new()
///     .wallet_ids("wallet-id-1,wallet-id-2".to_string())
///     .blockchain("ETH-SEPOLIA".to_string())
///     .state("PENDING".to_string())
///     .page_size(10)
///     .build();
/// ```
pub struct ListTransactionsParamsBuilder {
    params: ListTransactionsParams,
}

impl ListTransactionsParamsBuilder {
    /// Create a new builder instance
    pub fn new() -> Self {
        Self {
            params: ListTransactionsParams::default(),
        }
    }

    /// Filter by wallet IDs (comma-separated)
    pub fn wallet_ids(mut self, wallet_ids: String) -> Self {
        self.params.wallet_ids = Some(wallet_ids);
        self
    }

    /// Filter by blockchain
    pub fn blockchain(mut self, blockchain: String) -> Self {
        self.params.blockchain = Some(blockchain);
        self
    }

    /// Filter by custody type
    pub fn custody_type(mut self, custody_type: String) -> Self {
        self.params.custody_type = Some(custody_type);
        self
    }

    /// Filter by operation type
    pub fn operation(mut self, operation: String) -> Self {
        self.params.operation = Some(operation);
        self
    }

    /// Filter by transaction state
    pub fn state(mut self, state: String) -> Self {
        self.params.state = Some(state);
        self
    }

    /// Filter by transaction hash
    pub fn tx_hash(mut self, tx_hash: String) -> Self {
        self.params.tx_hash = Some(tx_hash);
        self
    }

    /// Filter by transaction type
    pub fn tx_type(mut self, tx_type: String) -> Self {
        self.params.tx_type = Some(tx_type);
        self
    }

    /// Filter by destination address
    pub fn destination_address(mut self, destination_address: String) -> Self {
        self.params.destination_address = Some(destination_address);
        self
    }

    /// Filter by date range
    pub fn date_range(mut self, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
        self.params.from = Some(from);
        self.params.to = Some(to);
        self
    }

    /// Set pagination parameters
    pub fn pagination(mut self, pagination: PaginationParams) -> Self {
        self.params.pagination = pagination;
        self
    }

    /// Set page size
    pub fn page_size(mut self, size: u32) -> Self {
        self.params.pagination.page_size = Some(size);
        self
    }

    /// Set page after cursor
    pub fn page_after(mut self, cursor: String) -> Self {
        self.params.pagination.page_after = Some(cursor);
        self
    }

    /// Set page before cursor
    pub fn page_before(mut self, cursor: String) -> Self {
        self.params.pagination.page_before = Some(cursor);
        self
    }

    /// Set sort order
    pub fn order(mut self, order: String) -> Self {
        self.params.order = Some(order);
        self
    }

    /// Build the list transactions parameters
    pub fn build(self) -> ListTransactionsParams {
        self.params
    }
}
