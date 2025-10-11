//! Wallet read operations for CircleView

use crate::{
    circle_view::circle_view::CircleView,
    helper::CircleResult,
    wallet::{
        dto::{
            EstimateContractExecutionFeeBody, EstimateContractExecutionFeeResponse,
            EstimateTransferFeeRequest, EstimateTransferFeeResponse, ListTransactionsParams,
            ListWalletsWithBalancesParams, NftsResponse, QueryParams, RequestTestnetTokensRequest,
            TokenBalancesResponse, TransactionResponse, TransactionsResponse, ValidateAddressBody,
            ValidateAddressResponse, WalletResponse, WalletsWithBalancesResponse,
        },
        views::{
            estimate_contract_execution_fee::EstimateContractExecutionFeeBodyBuilder,
            validate_address::ValidateAddressBodyBuilder,
        },
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

    /// Validate an address
    ///
    /// Validates an address for a specific blockchain
    pub async fn validate_address(
        &self,
        body: ValidateAddressBody,
    ) -> CircleResult<ValidateAddressResponse> {
        let body = ValidateAddressBodyBuilder::new()
            .address(body.address)
            .build();

        self.post::<ValidateAddressBody, ValidateAddressResponse>(
            "/v1/w3s/transactions/validateAddress",
            &body,
        )
        .await
    }

    /// Estimate fee for contract execution transaction
    ///
    /// Estimates gas fees that will be incurred for a contract execution transaction,
    /// given its ABI parameters and blockchain.
    pub async fn estimate_contract_execution_fee(
        &self,
        request: EstimateContractExecutionFeeBody,
    ) -> CircleResult<EstimateContractExecutionFeeResponse> {
        let body = EstimateContractExecutionFeeBodyBuilder::new(request.contract_address)
            .abi_function_signature(request.abi_function_signature)
            .abi_parameters(request.abi_parameters)
            .call_data(request.call_data)
            .amount(request.amount)
            .blockchain(request.blockchain)
            .source_address(request.source_address)
            .wallet_id(request.wallet_id)
            .build();

        self.post::<EstimateContractExecutionFeeBody, EstimateContractExecutionFeeResponse>(
            "/v1/w3s/transactions/contractExecution/estimateFee",
            &body,
        )
        .await
    }

    /// Estimate fee for transfer transaction
    ///
    /// Estimates gas fees that will be incurred for a transfer transaction,
    /// given its amount, blockchain, and token.
    pub async fn estimate_transfer_fee(
        &self,
        request: EstimateTransferFeeRequest,
    ) -> CircleResult<EstimateTransferFeeResponse> {
        self.post::<EstimateTransferFeeRequest, EstimateTransferFeeResponse>(
            "/v1/w3s/transactions/transfer/estimateFee",
            &request,
        )
        .await
    }

    /// Request testnet tokens from faucet
    ///
    /// Request testnet tokens for your wallet. Returns Ok(()) on success.
    /// Note: The faucet endpoint returns an empty response body on success.
    pub async fn request_testnet_tokens(
        &self,
        request: RequestTestnetTokensRequest,
    ) -> CircleResult<()> {
        // Try to post and handle empty JSON responses
        // The faucet endpoint returns empty body on success
        match self
            .post::<RequestTestnetTokensRequest, serde_json::Value>("/v1/faucet/drips", &request)
            .await
        {
            Ok(_) => Ok(()),
            Err(crate::CircleError::Json(e)) if e.to_string().contains("EOF") => {
                // Empty response is actually success for faucet endpoint
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
