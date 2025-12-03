use crate::dev_wallet::dto::ValidateAddressBody;

/// Builder for creating address validation requests
///
/// This builder helps construct requests to validate blockchain addresses.
///
/// # Example
///
/// ```rust,no_run
/// use inf_circle_sdk::dev_wallet::views::validate_address::ValidateAddressBodyBuilder;
///
/// let builder = ValidateAddressBodyBuilder::new()
///     .address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string())
///     .build();
/// ```
pub struct ValidateAddressBodyBuilder {
    address: String,
}

impl ValidateAddressBodyBuilder {
    /// Create a new builder instance
    pub fn new() -> Self {
        Self {
            address: String::new(),
        }
    }

    /// Set the address to validate
    pub fn address(mut self, address: String) -> Self {
        self.address = address;
        self
    }

    /// Build the address validation request
    pub fn build(self) -> ValidateAddressBody {
        ValidateAddressBody {
            address: self.address,
        }
    }
}
