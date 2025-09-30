//! Wallet read operations for CircleView

use crate::{
    circle_view::circle_view::CircleView,
    helper::CircleResult,
    wallet::dto::{
        ListTransactionsParams, ListWalletsWithBalancesParams, NftsResponse, QueryParams,
        TokenBalancesResponse, TransactionResponse, TransactionsResponse, WalletResponse,
        WalletsWithBalancesResponse,
    },
};

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

    /// List transactions
    ///
    /// Retrieves a list of all transactions that fit the specified parameters
    pub async fn list_transactions(
        &self,
        params: ListTransactionsParams,
    ) -> CircleResult<TransactionsResponse> {
        self.get_with_params("/v1/w3s/transactions", &params).await
    }

    /// Get a specific transaction
    ///
    /// Retrieves details of a specific transaction by ID
    pub async fn get_transaction(&self, tx_id: &str) -> CircleResult<TransactionResponse> {
        let path = format!("/v1/w3s/transactions/{}", tx_id);
        self.get(&path).await
    }
}
