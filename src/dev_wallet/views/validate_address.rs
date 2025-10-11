use crate::dev_wallet::dto::ValidateAddressBody;

pub struct ValidateAddressBodyBuilder {
    address: String,
}

impl ValidateAddressBodyBuilder {
    pub fn new() -> Self {
        Self {
            address: String::new(),
        }
    }
}

impl ValidateAddressBodyBuilder {
    pub fn address(mut self, address: String) -> Self {
        self.address = address;
        self
    }

    pub fn build(self) -> ValidateAddressBody {
        ValidateAddressBody {
            address: self.address,
        }
    }
}
