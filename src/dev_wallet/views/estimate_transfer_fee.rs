use crate::dev_wallet::dto::EstimateTransferFeeRequest;

/// Builder for creating transfer fee estimation requests
///
/// This builder helps construct requests to estimate gas fees for transfer transactions,
/// supporting native tokens, ERC-20 tokens, and NFTs.
///
/// # Example
///
/// ```rust,no_run
/// use inf_circle_sdk::dev_wallet::views::estimate_transfer_fee::EstimateTransferFeeRequestBuilder;
///
/// // Estimate fee for native token transfer
/// let request = EstimateTransferFeeRequestBuilder::new(
///     "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string(),
///     vec!["1000000000000000000".to_string()] // 1 ETH in wei
/// )
/// .blockchain(Some("ETH-SEPOLIA".to_string()))
/// .wallet_id(Some("wallet-id".to_string()))
/// .build();
/// ```
pub struct EstimateTransferFeeRequestBuilder {
    destination_address: String,
    amounts: Vec<String>,
    nft_token_ids: Option<Vec<String>>,
    source_address: Option<String>,
    token_id: Option<String>,
    token_address: Option<String>,
    blockchain: Option<String>,
    wallet_id: Option<String>,
}

impl EstimateTransferFeeRequestBuilder {
    /// Create a new builder with required fields
    ///
    /// # Arguments
    ///
    /// * `destination_address` - The destination address for the transfer
    /// * `amounts` - Vector of amounts to transfer (in the token's smallest unit)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::dev_wallet::views::estimate_transfer_fee::EstimateTransferFeeRequestBuilder;
    ///
    /// let builder = EstimateTransferFeeRequestBuilder::new(
    ///     "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string(),
    ///     vec!["1000000000000000000".to_string()] // 1 ETH
    /// );
    /// ```
    pub fn new(destination_address: impl Into<String>, amounts: Vec<String>) -> Self {
        Self {
            destination_address: destination_address.into(),
            amounts,
            nft_token_ids: None,
            source_address: None,
            token_id: None,
            token_address: None,
            blockchain: None,
            wallet_id: None,
        }
    }

    /// Set NFT token IDs for NFT transfers (batch transfers supported for ERC-1155 only)
    ///
    /// # Arguments
    ///
    /// * `ids` - Vector of NFT token IDs (length must match amounts length)
    pub fn nft_token_ids(mut self, ids: Option<Vec<String>>) -> Self {
        self.nft_token_ids = ids;
        self
    }

    /// Set the source address (required with blockchain if wallet_id is not provided)
    ///
    /// # Arguments
    ///
    /// * `address` - Source wallet address
    pub fn source_address(mut self, address: Option<String>) -> Self {
        self.source_address = address;
        self
    }

    /// Set the token ID (mutually exclusive with token_address and blockchain)
    ///
    /// # Arguments
    ///
    /// * `id` - Token ID for NFT transfers
    pub fn token_id(mut self, id: Option<String>) -> Self {
        self.token_id = id;
        self
    }

    /// Set the token contract address (empty for native tokens, mutually exclusive with token_id)
    ///
    /// # Arguments
    ///
    /// * `address` - ERC-20 token contract address, or None/empty for native tokens
    pub fn token_address(mut self, address: Option<String>) -> Self {
        self.token_address = address;
        self
    }

    /// Set the blockchain (required if token_id is not provided)
    ///
    /// # Arguments
    ///
    /// * `blockchain` - Blockchain identifier (e.g., "ETH-SEPOLIA")
    pub fn blockchain(mut self, blockchain: Option<String>) -> Self {
        self.blockchain = blockchain;
        self
    }

    /// Set the wallet ID (mutually exclusive with source_address and blockchain)
    ///
    /// # Arguments
    ///
    /// * `id` - Circle wallet ID (recommended approach)
    pub fn wallet_id(mut self, id: Option<String>) -> Self {
        self.wallet_id = id;
        self
    }

    /// Build the EstimateTransferFeeRequest
    pub fn build(self) -> EstimateTransferFeeRequest {
        EstimateTransferFeeRequest {
            destination_address: self.destination_address,
            amounts: self.amounts,
            nft_token_ids: self.nft_token_ids,
            source_address: self.source_address,
            token_id: self.token_id,
            token_address: self.token_address,
            blockchain: self.blockchain,
            wallet_id: self.wallet_id,
        }
    }
}
