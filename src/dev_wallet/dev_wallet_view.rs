//! Wallet read operations for CircleView

use crate::{
    circle_view::circle_view::CircleView,
    dev_wallet::{
        dto::{
            DevWalletResponse, EstimateContractExecutionFeeBody,
            EstimateContractExecutionFeeResponse, EstimateTransferFeeRequest,
            EstimateTransferFeeResponse, ListTransactionsParams, ListWalletsWithBalancesParams,
            NftsResponse, QueryParams, RequestTestnetTokensRequest, TokenBalancesResponse,
            TransactionResponse, TransactionsResponse, ValidateAddressBody,
            ValidateAddressResponse, WalletsWithBalancesResponse,
        },
        views::{
            estimate_contract_execution_fee::EstimateContractExecutionFeeBodyBuilder,
            validate_address::ValidateAddressBodyBuilder,
        },
    },
    helper::CircleResult,
};

// Re-use the Wallet struct from CircleOps since it's the same
pub use crate::dev_wallet::dto::{DevWallet, DevWalletsResponse, ListDevWalletsParams};

impl CircleView {
    /// List wallets
    ///
    /// Retrieves a list of all wallets that match the specified filter parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - Filter parameters including wallet set ID, blockchain, pagination, etc.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    /// use inf_circle_sdk::dev_wallet::views::list_wallets::ListDevWalletsParamsBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let params = ListDevWalletsParamsBuilder::new()
    ///     .wallet_set_id("wallet-set-id".to_string())
    ///     .page_size(10)
    ///     .build();
    ///
    /// let response = view.list_wallets(params).await?;
    /// for wallet in response.wallets {
    ///     println!("Wallet: {} - {}", wallet.id, wallet.address);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_wallets(
        &self,
        params: ListDevWalletsParams,
    ) -> CircleResult<DevWalletsResponse> {
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
    pub async fn get_wallet(&self, wallet_id: &str) -> CircleResult<DevWalletResponse> {
        let path = format!("/v1/w3s/wallets/{}", wallet_id);
        self.get(&path).await
    }

    /// Get token balances for a specific wallet
    ///
    /// Retrieves all token balances (native and ERC-20 tokens) for a specific wallet.
    ///
    /// # Arguments
    ///
    /// * `wallet_id` - The unique identifier of the wallet
    /// * `params` - Query parameters for filtering and pagination
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    /// use inf_circle_sdk::dev_wallet::views::query::QueryParamsBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let params = QueryParamsBuilder::new().build();
    /// let balances = view.get_token_balances("wallet-id", params).await?;
    ///
    /// for balance in balances.token_balances {
    ///     let symbol = balance.token.symbol.as_deref().unwrap_or("UNKNOWN");
    ///     println!("{}: {} {}",
    ///         symbol,
    ///         balance.amount,
    ///         if balance.token.is_native { "(native)" } else { "" }
    ///     );
    /// }
    /// # Ok(())
    /// # }
    /// ```
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
    /// Retrieves all NFTs (ERC-721 and ERC-1155 tokens) owned by a specific wallet.
    ///
    /// # Arguments
    ///
    /// * `wallet_id` - The unique identifier of the wallet
    /// * `params` - Query parameters for filtering and pagination
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    /// use inf_circle_sdk::dev_wallet::views::query::QueryParamsBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let params = QueryParamsBuilder::new().build();
    /// let nfts = view.get_nfts("wallet-id", params).await?;
    ///
    /// for nft in nfts.nfts {
    ///     println!("NFT: {} #{}", nft.token.name.unwrap_or_default(), nft.nft_token_id.unwrap_or_default());
    /// }
    /// # Ok(())
    /// # }
    /// ```
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
    /// Validates whether an address is correctly formatted for a specific blockchain.
    /// This is useful before attempting transfers to ensure the destination address is valid.
    ///
    /// # Arguments
    ///
    /// * `body` - The address to validate
    ///
    /// # Returns
    ///
    /// Returns validation result indicating if the address is valid.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    /// use inf_circle_sdk::dev_wallet::dto::ValidateAddressBody;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let body = ValidateAddressBody {
    ///     address: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string(),
    /// };
    ///
    /// let result = view.validate_address(body).await?;
    /// if result.is_valid {
    ///     println!("✅ Address is valid!");
    /// } else {
    ///     println!("❌ Invalid address");
    /// }
    /// # Ok(())
    /// # }
    /// ```
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
    /// Requests testnet tokens (ETH, USDC, EURC) from Circle's faucet for testing purposes.
    /// Only works on testnet blockchains (Sepolia, Fuji, etc.).
    ///
    /// # Arguments
    ///
    /// * `request` - Faucet request specifying the blockchain, address, and token types
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    /// use inf_circle_sdk::dev_wallet::dto::RequestTestnetTokensRequest;
    /// use inf_circle_sdk::types::Blockchain;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let request = RequestTestnetTokensRequest {
    ///     blockchain: Blockchain::EthSepolia,
    ///     address: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string(),
    ///     native: Some(true),   // Request ETH
    ///     usdc: Some(true),     // Request USDC
    ///     eurc: Some(false),
    /// };
    ///
    /// view.request_testnet_tokens(request).await?;
    /// println!("✅ Testnet tokens requested! Check wallet in a few minutes.");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// The faucet has rate limits. If you exceed them, wait a few minutes before trying again.
    /// Returns `Ok(())` on success with an empty response body.
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
