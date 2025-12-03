use crate::dev_wallet::dto::FeeLevel;
use crate::types::Blockchain;

/// Builder for creating transfer transaction requests
///
/// This builder helps construct transfer transaction requests for sending tokens
/// or NFTs from a wallet to a destination address.
///
/// # Example
///
/// ```rust,no_run
/// use inf_circle_sdk::dev_wallet::ops::create_transfer_transaction::CreateTransferTransactionRequestBuilder;
/// use inf_circle_sdk::types::Blockchain;
///
/// let builder = CreateTransferTransactionRequestBuilder::new()
///     .wallet_id("wallet-id".to_string())
///     .destination_address("0x1234...".to_string())
///     .amounts(vec!["1000000000000000000".to_string()]) // 1 ETH in wei
///     .blockchain(Blockchain::EthSepolia)
///     .build();
/// ```
#[derive(Clone, Debug)]
pub struct CreateTransferTransactionRequestBuilder {
    pub wallet_id: Option<String>,
    pub wallet_address: Option<String>,
    pub destination_address: String,
    pub amounts: Vec<String>,
    pub nft_token_ids: Option<Vec<String>>,
    pub token_id: Option<String>,
    pub token_address: Option<String>,
    pub idempotency_key: String,
    pub ref_id: Option<String>,
    pub blockchain: Option<Blockchain>,
    pub priority_fee: Option<String>,
    pub fee_level: Option<FeeLevel>,
    pub gas_limit: Option<String>,
    pub gas_price: Option<String>,
    pub max_fee: Option<String>,
}

impl CreateTransferTransactionRequestBuilder {
    /// Create a new builder instance
    pub fn new() -> Self {
        Self {
            wallet_id: None,
            wallet_address: None,
            destination_address: String::new(),
            amounts: Vec::new(),
            nft_token_ids: None,
            token_id: None,
            token_address: None,
            idempotency_key: String::new(),
            ref_id: None,
            blockchain: None,
            priority_fee: None,
            fee_level: None,
            gas_limit: None,
            gas_price: None,
            max_fee: None,
        }
    }

    /// Set the wallet ID (either wallet_id or wallet_address must be provided)
    pub fn wallet_id(mut self, wallet_id: String) -> Self {
        self.wallet_id = Some(wallet_id);
        self
    }

    /// Set the wallet address (either wallet_id or wallet_address must be provided)
    pub fn wallet_address(mut self, wallet_address: String) -> Self {
        self.wallet_address = Some(wallet_address);
        self
    }

    /// Set the destination address for the transfer
    pub fn destination_address(mut self, destination_address: String) -> Self {
        self.destination_address = destination_address;
        self
    }

    /// Set the amounts to transfer (in the token's smallest unit, e.g., wei for ETH)
    pub fn amounts(mut self, amounts: Vec<String>) -> Self {
        self.amounts = amounts;
        self
    }

    /// Set NFT token IDs to transfer (for NFT transfers)
    pub fn nft_token_ids(mut self, nft_token_ids: Vec<String>) -> Self {
        self.nft_token_ids = Some(nft_token_ids);
        self
    }

    /// Set the token ID for the transfer
    pub fn token_id(mut self, token_id: String) -> Self {
        self.token_id = Some(token_id);
        self
    }

    /// Set the token contract address (for ERC-20 token transfers)
    pub fn token_address(mut self, token_address: String) -> Self {
        self.token_address = Some(token_address);
        self
    }

    /// Set a custom idempotency key for the request
    pub fn idempotency_key(mut self, idempotency_key: String) -> Self {
        self.idempotency_key = idempotency_key;
        self
    }

    /// Set a reference ID for tracking purposes
    pub fn ref_id(mut self, ref_id: String) -> Self {
        self.ref_id = Some(ref_id);
        self
    }

    /// Set the blockchain for the transfer
    pub fn blockchain(mut self, blockchain: Blockchain) -> Self {
        self.blockchain = Some(blockchain);
        self
    }

    /// Set the priority fee (EIP-1559 maxPriorityFeePerGas)
    pub fn priority_fee(mut self, priority_fee: String) -> Self {
        self.priority_fee = Some(priority_fee);
        self
    }

    /// Set the fee level (low, medium, high)
    pub fn fee_level(mut self, fee_level: FeeLevel) -> Self {
        self.fee_level = Some(fee_level);
        self
    }

    /// Set the gas limit for the transaction
    pub fn gas_limit(mut self, gas_limit: String) -> Self {
        self.gas_limit = Some(gas_limit);
        self
    }

    /// Set the gas price for the transaction
    pub fn gas_price(mut self, gas_price: String) -> Self {
        self.gas_price = Some(gas_price);
        self
    }

    /// Set the maximum fee (EIP-1559 maxFeePerGas)
    pub fn max_fee(mut self, max_fee: String) -> Self {
        self.max_fee = Some(max_fee);
        self
    }

    /// Build the transfer transaction request
    pub fn build(self) -> CreateTransferTransactionRequestBuilder {
        self
    }
}
