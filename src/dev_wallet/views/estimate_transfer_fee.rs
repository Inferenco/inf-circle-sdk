use crate::dev_wallet::dto::EstimateTransferFeeRequest;

/// Builder for EstimateTransferFeeRequest
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

    /// Set NFT token IDs (batch transfers supported for ERC-1155 only)
    /// Length must match amounts length
    pub fn nft_token_ids(mut self, ids: Option<Vec<String>>) -> Self {
        self.nft_token_ids = ids;
        self
    }

    /// Set the source address
    /// Required with blockchain if walletId is not provided
    pub fn source_address(mut self, address: Option<String>) -> Self {
        self.source_address = address;
        self
    }

    /// Set the token ID
    /// Excluded with tokenAddress and blockchain
    pub fn token_id(mut self, id: Option<String>) -> Self {
        self.token_id = id;
        self
    }

    /// Set the token address (empty for native tokens)
    /// Excluded with tokenId
    pub fn token_address(mut self, address: Option<String>) -> Self {
        self.token_address = address;
        self
    }

    /// Set the blockchain
    /// Required if tokenId is not provided
    pub fn blockchain(mut self, blockchain: Option<String>) -> Self {
        self.blockchain = blockchain;
        self
    }

    /// Set the wallet ID
    /// Mutually exclusive with sourceAddress and blockchain
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
