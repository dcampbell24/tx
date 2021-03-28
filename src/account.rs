use rust_decimal::Decimal;
use serde::Serialize;

pub type ClientId = u16;

#[derive(Debug, Serialize)]
pub struct Account {
    pub client_id: ClientId,
    /// ```
    /// available = total - held
    /// ```
    /// The total funds that are available for
    /// trading, staking, withdrawal, etc.
    pub available: Decimal,
    /// ```
    /// held = total - available
    /// ```
    /// The total funds that are held for dispute.
    pub held: Decimal,
    /// ```
    /// total = available + held
    /// ```
    pub total: Decimal,
    /// Whether the account is locked. An
    /// account is locked if a charge back occurs.
    pub locked: bool,
}

impl Account {
    pub fn new(client_id: ClientId) -> Self {
        Account {
            client_id,
            available: Decimal::new(0, 4),
            held: Decimal::new(0, 4),
            total: Decimal::new(0, 4),
            locked: false,
        }
    }
}
