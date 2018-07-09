/// Transaction parameter option
pub struct TransactionOption<'a> {
    code: &'a str,
    address: &'a str,
    current_height: Option<u64>,
    quota: Option<u64>,
    value: Option<&'a str>,
}

impl<'a> TransactionOption<'a> {
    /// Default option
    pub fn new() -> Self {
        TransactionOption {
            code: "0x",
            address: "0x",
            current_height: None,
            quota: None,
            value: None,
        }
    }

    /// Set code. Transaction content, default is "0x"
    pub fn set_code(mut self, code: &'a str) -> Self {
        self.code = code;
        self
    }

    /// Get code
    pub fn code(&self) -> &str {
        self.code
    }

    /// Set address. Contract address, default is "0x", which creates the contract
    pub fn set_address(mut self, address: &'a str) -> Self {
        self.address = address;
        self
    }

    /// Get address
    pub fn address(&self) -> &str {
        self.address
    }

    /// Set current height.
    /// Set the current chain height, the default is None,
    /// automatically query before the transaction to get the current chain height
    pub fn set_current_height(mut self, height: Option<u64>) -> Self {
        self.current_height = height;
        self
    }

    /// Get current height
    pub fn current_height(&self) -> Option<u64> {
        self.current_height
    }

    /// Set quota. Transaction consumption quota limit
    pub fn set_quota(mut self, quota: Option<u64>) -> Self {
        self.quota = quota;
        self
    }

    /// Get quota
    pub fn quota(&self) -> Option<u64> {
        self.quota
    }

    /// Set value. Transaction transfer amount
    pub fn set_value(mut self, value: Option<&'a str>) -> Self {
        self.value = value;
        self
    }

    /// Get value
    pub fn value(&self) -> Option<&str> {
        self.value
    }

    /// Restore initialization status
    pub fn clear(&mut self) {
        self.value = None;
        self.quota = None;
        self.current_height = None;
        self.address = "0x";
        self.code = "0x";
    }
}
