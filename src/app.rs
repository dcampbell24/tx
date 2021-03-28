use csv::Trim;

use std::collections::HashMap;
use std::default::Default;
use std::error::Error;
use std::io;

use crate::account::{Account, ClientId};
use crate::transaction::{Transaction, TxId, Type};

#[derive(Debug, Default)]
pub struct App {
    pub client_accounts: HashMap<ClientId, Account>,
    pub transactions: HashMap<TxId, Transaction>,
}

impl App {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn process_transactions(&mut self, transactions_path: &str) -> Result<(), Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .trim(Trim::All)
            .from_path(transactions_path)?;

        for result in rdr.deserialize::<Transaction>() {
            match result {
                Ok(tx) => self.process_transaction(tx),
                Err(e) => eprintln!("{}", e),
            }
        }
        Ok(())
    }

    fn process_transaction(&mut self, tx: Transaction) {
        let account = self
            .client_accounts
            .entry(tx.client_id)
            .or_insert_with(|| Account::new(tx.client_id));

        match tx.type_ {
            Type::Chargeback => {}
            Type::Deposit => {
                let amount = if tx.amount.is_some() {
                    tx.amount.unwrap()
                } else {
                    eprintln!("tx {}: invalid deposit, no amount", tx.tx_id);
                    return;
                };

                if self.transactions.contains_key(&tx.tx_id) {
                    eprintln!(
                        "tx {}: {} already deposited to account {}",
                        tx.tx_id, amount, tx.client_id
                    );
                    return;
                }

                account.available += amount;
                account.total += amount;

                eprintln!(
                    "tx {}: deposited {} into account {}",
                    tx.tx_id, amount, tx.client_id
                );
            }
            Type::Dispute => {
                if let Some(tx) = self.transactions.get(&tx.tx_id) {
                    if let Some(amount) = tx.amount {
                        account.available -= amount;
                        account.held += amount;
                    }
                    eprintln!("tx {} disputed by client {} over {:?}", tx.tx_id, tx.client_id, tx.amount);
                } else {
                    eprintln!("tx {} disputed, but we have no record of this transaction", tx.tx_id);
                }
            }
            Type::Resolve => {}
            Type::Withdrawal | Type::Withdraw => {
                let amount = if tx.amount.is_some() {
                    tx.amount.unwrap()
                } else {
                    eprintln!("tx {}: invalid withdrawal, no amount", tx.tx_id);
                    return;
                };

                if amount > account.available {
                    eprintln!(
                        "tx {}: account {} has insufficient funds to withdraw {}",
                        tx.tx_id, tx.client_id, amount
                    );
                } else {
                    account.available -= amount;
                    account.total -= amount;
                    eprintln!(
                        "tx {}: withdrew {} from account {}",
                        tx.tx_id, amount, tx.client_id
                    );
                }
            }
        }

        self.transactions.insert(tx.tx_id, tx);
    }

    pub fn print_accounts_as_csv(&self) -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::Writer::from_writer(io::stdout());

        for account in self.client_accounts.values() {
            wtr.serialize(account)?;
        }

        wtr.flush()?;
        Ok(())
    }
}
