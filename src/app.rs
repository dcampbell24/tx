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
                if self.transactions.contains_key(&tx.tx_id) {
                    eprintln!(
                        "tx {}: {} already deposited to account {}",
                        tx.tx_id, tx.amount, tx.client_id
                    );
                    return;
                }

                account.available += tx.amount;
                account.total += tx.amount;

                eprintln!(
                    "tx {}: deposited {} into account {}",
                    tx.tx_id, tx.amount, tx.client_id
                );
            }
            Type::Dispute => {
                if let Some(tx) = self.transactions.get(&tx.tx_id) {
                    account.available -= tx.amount;
                    account.held += tx.amount;
                    eprintln!("tx {} disputed by client {} over {}", tx.tx_id, tx.client_id, tx.amount);
                } else {
                    eprintln!("tx {} disputed, but we have no record of this transaction", tx.tx_id);
                }
            }
            Type::Resolve => {}
            Type::Withdrawal | Type::Withdraw => {
                if tx.amount > account.available {
                    eprintln!(
                        "tx {}: account {} has insufficient funds to withdraw {}",
                        tx.tx_id, tx.client_id, tx.amount
                    );
                } else {
                    account.available -= tx.amount;
                    account.total -= tx.amount;
                    eprintln!(
                        "tx {}: withdrew {} from account {}",
                        tx.tx_id, tx.amount, tx.client_id
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
