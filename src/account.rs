use rust_decimal::Decimal;
use serde::Serialize;

use crate::transaction::{Transaction, Type};

pub type ClientId = u16;

#[derive(Debug, Serialize)]
pub struct Account {
    client_id: ClientId,
    /// ```
    /// available = total - held
    /// ```
    /// The total funds that are available for
    /// trading, staking, withdrawal, etc.
    available: Decimal,
    /// ```
    /// held = total - available
    /// ```
    /// The total funds that are held for dispute.
    held: Decimal,
    /// ```
    /// total = available + held
    /// ```
    total: Decimal,
    /// Whether the account is locked. An
    /// account is locked if a charge back occurs.
    locked: bool,
}

impl Account {
    pub fn new(client_id: ClientId) -> Self {
        Account {
            client_id,
            available: Decimal::new(0, 0),
            held: Decimal::new(0, 0),
            total: Decimal::new(0, 0),
            locked: false,
        }
    }

    pub fn process_transaction(&mut self, tx: &Transaction) {
        match tx.type_ {
            Type::Deposit => {
                self.available += tx.amount;
                self.total += tx.amount;
                eprintln!("{}: {}", tx.client_id, tx.amount);
            }
            Type::Withdrawal => {
                if tx.amount > self.available {
                    eprintln!("insufficent funds for withdrawal");
                } else {
                    self.available -= tx.amount;
                    self.total -= tx.amount;
                    eprintln!("{}: ({})", tx.client_id, tx.amount);
                }
            }
        }
    }
}
