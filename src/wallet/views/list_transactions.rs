use chrono::{DateTime, Utc};

use crate::{helper::PaginationParams, wallet::dto::ListTransactionsParams};

pub struct ListTransactionsParamsBuilder {
    params: ListTransactionsParams,
}

impl ListTransactionsParamsBuilder {
    pub fn new() -> Self {
        Self {
            params: ListTransactionsParams::default(),
        }
    }

    pub fn wallet_ids(mut self, wallet_ids: String) -> Self {
        self.params.wallet_ids = Some(wallet_ids);
        self
    }

    pub fn blockchain(mut self, blockchain: String) -> Self {
        self.params.blockchain = Some(blockchain);
        self
    }

    pub fn custody_type(mut self, custody_type: String) -> Self {
        self.params.custody_type = Some(custody_type);
        self
    }

    pub fn operation(mut self, operation: String) -> Self {
        self.params.operation = Some(operation);
        self
    }

    pub fn state(mut self, state: String) -> Self {
        self.params.state = Some(state);
        self
    }

    pub fn tx_hash(mut self, tx_hash: String) -> Self {
        self.params.tx_hash = Some(tx_hash);
        self
    }

    pub fn tx_type(mut self, tx_type: String) -> Self {
        self.params.tx_type = Some(tx_type);
        self
    }

    pub fn destination_address(mut self, destination_address: String) -> Self {
        self.params.destination_address = Some(destination_address);
        self
    }

    pub fn date_range(mut self, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
        self.params.from = Some(from);
        self.params.to = Some(to);
        self
    }

    pub fn pagination(mut self, pagination: PaginationParams) -> Self {
        self.params.pagination = pagination;
        self
    }

    pub fn page_size(mut self, size: u32) -> Self {
        self.params.pagination.page_size = Some(size);
        self
    }

    pub fn page_after(mut self, cursor: String) -> Self {
        self.params.pagination.page_after = Some(cursor);
        self
    }

    pub fn page_before(mut self, cursor: String) -> Self {
        self.params.pagination.page_before = Some(cursor);
        self
    }

    pub fn order(mut self, order: String) -> Self {
        self.params.order = Some(order);
        self
    }

    pub fn build(self) -> ListTransactionsParams {
        self.params
    }
}
