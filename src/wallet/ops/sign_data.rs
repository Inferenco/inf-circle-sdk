use crate::helper::CircleResult;

#[derive(Clone, Debug)]
pub struct SignDataRequestBuilder {
    pub(crate) wallet_id: String,
    pub(crate) data: String,
    pub(crate) memo: Option<String>,
}

impl SignDataRequestBuilder {
    pub fn new(wallet_id: String, data: String) -> CircleResult<Self> {
        Ok(Self {
            wallet_id,
            data,
            memo: None,
        })
    }

    pub fn wallet_id(mut self, wallet_id: String) -> Self {
        self.wallet_id = wallet_id;
        self
    }

    pub fn data(mut self, data: String) -> Self {
        self.data = data;
        self
    }

    pub fn memo(mut self, memo: String) -> Self {
        self.memo = Some(memo);
        self
    }

    pub fn build(self) -> SignDataRequestBuilder {
        self
    }
}
