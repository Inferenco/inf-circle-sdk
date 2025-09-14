//! Wallet read operations for CircleView

use crate::{
    circle_view::circle_view::CircleView,
    helper::{CircleResult, PaginationParams},
    wallet::dto::{
        ListWalletsWithBalancesParams, NftsResponse, QueryParams, TokenBalancesResponse,
        WalletResponse, WalletsWithBalancesResponse,
    },
};
use chrono::{DateTime, Utc};

// Re-use the Wallet struct from CircleOps since it's the same
pub use crate::wallet::dto::{ListWalletsParams, Wallet, WalletsResponse};

impl CircleView {
    /// List wallets
    ///
    /// Retrieves a list of all wallets that fit the specified parameters
    pub async fn list_wallets(&self, params: ListWalletsParams) -> CircleResult<WalletsResponse> {
        self.get_with_params("/v1/w3s/wallets", &params).await
    }

    /// List wallets with token balances
    ///
    /// Retrieves a list of all wallets with token balances that fit the specified parameters
    pub async fn list_wallets_with_token_balances(
        &self,
        params: ListWalletsWithBalancesParams,
    ) -> CircleResult<WalletsWithBalancesResponse> {
        self.get_with_params("/v1/w3s/wallets/balances", &params)
            .await
    }

    /// Get a specific wallet
    ///
    /// Retrieves details of a specific wallet by ID
    pub async fn get_wallet(&self, wallet_id: &str) -> CircleResult<WalletResponse> {
        let path = format!("/v1/w3s/wallets/{}", wallet_id);
        self.get(&path).await
    }

    /// Get token balances for a specific wallet
    ///
    /// Retrieves the token balances for a specific wallet by ID that fit the specified parameters
    pub async fn get_token_balances(
        &self,
        wallet_id: &str,
        params: QueryParams,
    ) -> CircleResult<TokenBalancesResponse> {
        let path = format!("/v1/w3s/wallets/{}/balances", wallet_id);
        self.get_with_params(&path, &params).await
    }

    /// Get NFTs for a specific wallet
    ///
    /// Retrieves the NFTs for a specific wallet by ID
    pub async fn get_nfts(
        &self,
        wallet_id: &str,
        params: QueryParams,
    ) -> CircleResult<NftsResponse> {
        let path = format!("/v1/w3s/wallets/{}/nfts", wallet_id);
        self.get_with_params(&path, &params).await
    }
}

/// Builder for ListWalletsParams
pub struct ListWalletsParamsBuilder {
    params: ListWalletsParams,
}

impl ListWalletsParamsBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            params: ListWalletsParams::default(),
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
    pub fn build(self) -> ListWalletsParams {
        self.params
    }
}

impl Default for ListWalletsParamsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for ListWalletsWithBalancesParams
pub struct ListWalletsWithBalancesParamsBuilder {
    params: ListWalletsWithBalancesParams,
}

impl ListWalletsWithBalancesParamsBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            params: ListWalletsWithBalancesParams::default(),
        }
    }

    /// Filter by address
    pub fn address(mut self, address: String) -> Self {
        self.params.address = Some(address);
        self
    }

    /// Filter by amount greater than or equal to
    pub fn amount_gte(mut self, amount: String) -> Self {
        self.params.amount_gte = Some(amount);
        self
    }

    /// Filter by token address
    pub fn token_address(mut self, token_address: String) -> Self {
        self.params.token_address = Some(token_address);
        self
    }

    /// Filter by wallet set ID
    pub fn wallet_set_id(mut self, wallet_set_id: String) -> Self {
        self.params.wallet_set_id = Some(wallet_set_id);
        self
    }

    /// Filter by blockchain
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

impl Default for QueryParamsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
