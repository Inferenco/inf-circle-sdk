use crate::helper::PaginationParams;
use crate::wallet::dto::QueryParams;

/// Builder for QueryParams
pub struct QueryParamsBuilder {
    params: QueryParams,
}

impl QueryParamsBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            params: QueryParams::default(),
        }
    }

    /// Set include all
    pub fn include_all(mut self, include_all: bool) -> Self {
        self.params.include_all = Some(include_all);
        self
    }

    /// Set name
    pub fn name(mut self, name: String) -> Self {
        self.params.name = Some(name);
        self
    }

    /// Set token address
    pub fn token_address(mut self, token_address: String) -> Self {
        self.params.token_address = Some(token_address);
        self
    }

    /// Set standard
    pub fn standard(mut self, standard: String) -> Self {
        self.params.standard = Some(standard);
        self
    }

    /// Set pagination
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

    /// Build the parameters
    pub fn build(self) -> QueryParams {
        self.params
    }
}
