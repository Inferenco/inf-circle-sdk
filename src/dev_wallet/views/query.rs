use crate::dev_wallet::dto::QueryParams;
use crate::helper::PaginationParams;

/// Builder for creating query parameters for wallet token balances and NFTs
///
/// This builder helps construct parameters for querying token balances and NFTs
/// for a specific wallet, with filtering and pagination support.
///
/// # Example
///
/// ```rust,no_run
/// use inf_circle_sdk::dev_wallet::views::query::QueryParamsBuilder;
///
/// let params = QueryParamsBuilder::new()
///     .include_all(true)
///     .token_address("0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238".to_string())
///     .standard("ERC20".to_string())
///     .page_size(20)
///     .build();
/// ```
pub struct QueryParamsBuilder {
    params: QueryParams,
}

impl QueryParamsBuilder {
    /// Create a new builder instance
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::dev_wallet::views::query::QueryParamsBuilder;
    ///
    /// let builder = QueryParamsBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            params: QueryParams::default(),
        }
    }

    /// Set whether to include all tokens (including zero balances)
    ///
    /// # Arguments
    ///
    /// * `include_all` - If true, includes tokens with zero balance
    pub fn include_all(mut self, include_all: bool) -> Self {
        self.params.include_all = Some(include_all);
        self
    }

    /// Filter by token name
    ///
    /// # Arguments
    ///
    /// * `name` - Token name to filter by
    pub fn name(mut self, name: String) -> Self {
        self.params.name = Some(name);
        self
    }

    /// Filter by token contract address
    ///
    /// # Arguments
    ///
    /// * `token_address` - Token contract address to filter by
    pub fn token_address(mut self, token_address: String) -> Self {
        self.params.token_address = Some(token_address);
        self
    }

    /// Filter by token standard (e.g., "ERC20", "ERC721", "ERC1155")
    ///
    /// # Arguments
    ///
    /// * `standard` - Token standard to filter by
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
