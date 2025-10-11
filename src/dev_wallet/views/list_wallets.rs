use crate::dev_wallet::dto::ListDevWalletsParams;
use crate::helper::PaginationParams;
use chrono::{DateTime, Utc};

/// Builder for ListWalletsParams
pub struct ListDevWalletsParamsBuilder {
    params: ListDevWalletsParams,
}

impl ListDevWalletsParamsBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            params: ListDevWalletsParams::default(),
        }
    }

    /// Filter by blockchain address
    pub fn address(mut self, address: String) -> Self {
        self.params.address = Some(address);
        self
    }

    /// Filter by blockchain
    pub fn blockchain(mut self, blockchain: String) -> Self {
        self.params.blockchain = Some(blockchain);
        self
    }

    /// Filter by SCA version
    pub fn sca_core(mut self, sca_core: String) -> Self {
        self.params.sca_core = Some(sca_core);
        self
    }

    /// Filter by wallet set ID
    pub fn wallet_set_id(mut self, wallet_set_id: String) -> Self {
        self.params.wallet_set_id = Some(wallet_set_id);
        self
    }

    /// Filter by reference ID
    pub fn ref_id(mut self, ref_id: String) -> Self {
        self.params.ref_id = Some(ref_id);
        self
    }

    /// Filter by creation date range
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

    /// Build the parameters
    pub fn build(self) -> ListDevWalletsParams {
        self.params
    }
}
