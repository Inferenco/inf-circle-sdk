use crate::dev_wallet::dto::ListWalletsWithBalancesParams;
use crate::helper::PaginationParams;
use chrono::{DateTime, Utc};

/// Builder for creating list wallets with balances query parameters
///
/// This builder helps construct parameters for querying wallets that have token balances,
/// with support for filtering by token, amount, blockchain, and pagination.
///
/// # Example
///
/// ```rust,no_run
/// use inf_circle_sdk::dev_wallet::views::list_wallets_with_balances::ListWalletsWithBalancesParamsBuilder;
///
/// let params = ListWalletsWithBalancesParamsBuilder::new()
///     .blockchain("ETH-SEPOLIA".to_string())
///     .token_address("0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238".to_string()) // USDC
///     .amount_gte("1000000".to_string()) // At least 1 USDC (6 decimals)
///     .page_size(10)
///     .build();
/// ```
pub struct ListWalletsWithBalancesParamsBuilder {
    params: ListWalletsWithBalancesParams,
}

impl ListWalletsWithBalancesParamsBuilder {
    /// Create a new builder instance
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::dev_wallet::views::list_wallets_with_balances::ListWalletsWithBalancesParamsBuilder;
    ///
    /// let builder = ListWalletsWithBalancesParamsBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            params: ListWalletsWithBalancesParams::default(),
        }
    }

    /// Filter by wallet blockchain address
    ///
    /// # Arguments
    ///
    /// * `address` - The blockchain address to filter by
    pub fn address(mut self, address: String) -> Self {
        self.params.address = Some(address);
        self
    }

    /// Filter by minimum token amount (greater than or equal to)
    ///
    /// # Arguments
    ///
    /// * `amount` - Minimum token amount in the token's smallest unit
    pub fn amount_gte(mut self, amount: String) -> Self {
        self.params.amount_gte = Some(amount);
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

    /// Filter by wallet set ID
    pub fn wallet_set_id(mut self, wallet_set_id: String) -> Self {
        self.params.wallet_set_id = Some(wallet_set_id);
        self
    }

    /// Filter by blockchain identifier (required)
    ///
    /// # Arguments
    ///
    /// * `blockchain` - Blockchain identifier (e.g., "ETH-SEPOLIA", "AVAX-FUJI")
    pub fn blockchain(mut self, blockchain: String) -> Self {
        self.params.blockchain = blockchain;
        self
    }

    /// Filter by SCA version
    pub fn sca_core(mut self, sca_core: String) -> Self {
        self.params.sca_core = Some(sca_core);
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

    /// Build the parameters
    pub fn build(self) -> ListWalletsWithBalancesParams {
        self.params
    }
}
