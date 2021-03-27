use csv::Trim;

use std::collections::HashMap;
use std::default::Default;
use std::error::Error;
use std::io;

use crate::account::{Account, ClientId};
use crate::transaction::{Transaction, TxId};

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
        if self.transactions.contains_key(&tx.tx_id) {
            eprintln!("ignoring already processed transaction {}", tx.tx_id);
            return;
        }

        eprintln!("processing {:?} ...", tx);
        let account = self
            .client_accounts
            .entry(tx.client_id)
            .or_insert_with(|| Account::new(tx.client_id));

        account.process_transaction(&tx);
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
