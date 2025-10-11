use crate::helper::CircleResult;

#[derive(Clone, Debug)]
pub struct SignMessageRequestBuilder {
    pub(crate) wallet_id: String,
    pub(crate) message: String,
    pub(crate) encoded_by_hex: Option<bool>,
    pub(crate) memo: Option<String>,
}

impl SignMessageRequestBuilder {
    pub fn new(wallet_id: String, message: String) -> CircleResult<Self> {
        Ok(Self {
            wallet_id,
            message,
            encoded_by_hex: None,
            memo: None,
        })
    }

    pub fn wallet_id(mut self, wallet_id: String) -> Self {
        self.wallet_id = wallet_id;
        self
    }

    pub fn encoded_by_hex(mut self, encoded_by_hex: bool) -> Self {
        self.encoded_by_hex = Some(encoded_by_hex);
        self
    }

    pub fn memo(mut self, memo: String) -> Self {
        self.memo = Some(memo);
        self
    }

    pub fn build(self) -> SignMessageRequestBuilder {
        self
    }
}
