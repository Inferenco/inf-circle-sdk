use crate::wallet::dto::{FeeLevel, ScaCore};

#[derive(Clone, Debug)]
pub struct CreateWalletUpgradeTransactionRequestBuilder {
    pub wallet_id: String,
    pub new_sca_core: ScaCore,
    pub idempotency_key: String,
    pub fee_level: Option<FeeLevel>,
    pub gas_limit: Option<String>,
    pub gas_price: Option<String>,
    pub max_fee: Option<String>,
    pub priority_fee: Option<String>,
    pub ref_id: Option<String>,
}

impl CreateWalletUpgradeTransactionRequestBuilder {
    /// Create a new builder with required fields
    pub fn new(wallet_id: String, new_sca_core: ScaCore, idempotency_key: String) -> Self {
        Self {
            wallet_id,
            new_sca_core,
            idempotency_key,
            fee_level: None,
            gas_limit: None,
            gas_price: None,
            max_fee: None,
            priority_fee: None,
            ref_id: None,
        }
    }

    /// Set the fee level (LOW, MEDIUM, or HIGH)
    /// Cannot be used with gasPrice, priorityFee, or maxFee
    pub fn fee_level(mut self, fee_level: FeeLevel) -> Self {
        self.fee_level = Some(fee_level);
        self
    }

    /// Set the maximum units of gas to use for the transaction
    pub fn gas_limit(mut self, gas_limit: String) -> Self {
        self.gas_limit = Some(gas_limit);
        self
    }

    /// Set the maximum price of gas, in gwei
    /// Requires gasLimit. Cannot be used with feeLevel, priorityFee, or maxFee
    pub fn gas_price(mut self, gas_price: String) -> Self {
        self.gas_price = Some(gas_price);
        self
    }

    /// Set the maximum price per unit of gas, in gwei (EIP-1559)
    /// Requires priorityFee and gasLimit. Cannot be used with feeLevel or gasPrice
    pub fn max_fee(mut self, max_fee: String) -> Self {
        self.max_fee = Some(max_fee);
        self
    }

    /// Set the priority fee, in gwei (EIP-1559)
    /// Requires maxFee and gasLimit. Cannot be used with feeLevel or gasPrice
    pub fn priority_fee(mut self, priority_fee: String) -> Self {
        self.priority_fee = Some(priority_fee);
        self
    }

    /// Set the optional reference or description
    pub fn ref_id(mut self, ref_id: String) -> Self {
        self.ref_id = Some(ref_id);
        self
    }

    /// Build the CreateWalletUpgradeTransactionRequestBuilder
    pub fn build(self) -> CreateWalletUpgradeTransactionRequestBuilder {
        self
    }
}
