use crate::dev_wallet::dto::TransactionParams;

pub struct TransactionParamsBuilder {
    params: TransactionParams,
}

impl TransactionParamsBuilder {
    pub fn new() -> Self {
        Self {
            params: TransactionParams::default(),
        }
    }

    pub fn tx_type(mut self, tx_type: String) -> Self {
        self.params.tx_type = tx_type;
        self
    }

    pub fn build(self) -> TransactionParams {
        self.params
    }
}
