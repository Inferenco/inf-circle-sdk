use crate::helper::CircleResult;

#[derive(Clone, Debug)]
pub struct SignDelegateRequestBuilder {
    pub(crate) wallet_id: String,
    pub(crate) unsigned_delegate_action: String,
}

impl SignDelegateRequestBuilder {
    pub fn new(wallet_id: String, unsigned_delegate_action: String) -> CircleResult<Self> {
        Ok(Self {
            wallet_id,
            unsigned_delegate_action,
        })
    }

    pub fn wallet_id(mut self, wallet_id: String) -> Self {
        self.wallet_id = wallet_id;
        self
    }

    pub fn unsigned_delegate_action(mut self, unsigned_delegate_action: String) -> Self {
        self.unsigned_delegate_action = unsigned_delegate_action;
        self
    }

    pub fn build(self) -> SignDelegateRequestBuilder {
        self
    }
}
