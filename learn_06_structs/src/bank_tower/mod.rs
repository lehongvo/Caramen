pub struct Account {
    pub owner: String,
    balance: u64, // riêng tư: chỉ code trong mod bank thấy
}

impl Account {
    pub fn new(owner: &str, balance: u64) -> Account {
        Account {
            owner: String::from(owner),
            balance,
        }
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }
}
