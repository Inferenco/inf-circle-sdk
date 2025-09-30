use crate::helper::CircleResult;

#[derive(Clone, Debug)]
pub struct SignTransactionRequestBuilder {
    pub(crate) wallet_id: String,
    pub(crate) raw_transaction: Option<String>,
    pub(crate) transaction: Option<String>,
    pub(crate) memo: Option<String>,
}

impl SignTransactionRequestBuilder {
    pub fn new(
        wallet_id: String,
        raw_transaction: Option<String>,
        transaction: Option<String>,
    ) -> CircleResult<Self> {
        Ok(Self {
            wallet_id,
            raw_transaction,
            transaction,
            memo: None,
        })
    }

    pub fn wallet_id(mut self, wallet_id: String) -> Self {
        self.wallet_id = wallet_id;
        self
    }

    pub fn raw_transaction(mut self, raw_transaction: String) -> Self {
        self.raw_transaction = Some(raw_transaction);
        self
    }

    pub fn transaction(mut self, transaction: String) -> Self {
        self.transaction = Some(transaction);
        self
    }

    pub fn memo(mut self, memo: String) -> Self {
        self.memo = Some(memo);
        self
    }

    pub fn build(self) -> SignTransactionRequestBuilder {
        self
    }
}
