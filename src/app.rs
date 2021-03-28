use csv::Trim;
use log::info;

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
                Err(e) => info!("{}", e),
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
            Type::Chargeback => {
                if let Some(tx_chargeback) = self.transactions.get_mut(&tx.tx_id) {
                    if tx_chargeback.client_id != tx.client_id {
                        info!("client id in chargeback and chargeback transaction do not match");
                        return;
                    }

                    if tx_chargeback.disputed != Some(true) {
                        info!("tx {} was not disputed and can't be charged back", tx.tx_id);
                        return;
                    }

                    if let Some(amount) = tx_chargeback.amount {
                        account.held -= amount;
                        account.total -= amount;
                    }
                    tx_chargeback.disputed = Some(false);
                    account.locked = true;

                    info!(
                        "tx {} dispute charged back {:?} by client {} and account locked",
                        tx.tx_id, tx_chargeback.amount, tx_chargeback.client_id
                    );
                }
            }
            Type::Deposit => {
                let amount = if tx.amount.is_some() {
                    tx.amount.unwrap()
                } else {
                    info!("tx {}: invalid deposit, no amount", tx.tx_id);
                    return;
                };

                if self.transactions.contains_key(&tx.tx_id) {
                    info!(
                        "tx {}: {} already deposited to account {}",
                        tx.tx_id, amount, tx.client_id
                    );
                    return;
                }

                account.available += amount;
                account.total += amount;

                info!(
                    "tx {}: depositing {} into account {}",
                    tx.tx_id, amount, tx.client_id
                );
                self.transactions.insert(tx.tx_id, tx);
            }
            Type::Dispute => {
                if let Some(tx_disputed) = self.transactions.get_mut(&tx.tx_id) {
                    if tx_disputed.client_id != tx.client_id {
                        info!("client id in dispute and disputed transaction do not match");
                        return;
                    }

                    if let Some(amount) = &tx_disputed.amount {
                        account.available -= amount;
                        account.held += amount;
                    }
                    tx_disputed.disputed = Some(true);

                    info!(
                        "tx {} disputed by client {} over {:?}",
                        tx.tx_id, tx_disputed.client_id, &tx_disputed.amount
                    );
                } else {
                    info!(
                        "tx {} disputed, but we have no record of this transaction",
                        tx.tx_id
                    );
                }
            }
            Type::Resolve => {
                if let Some(tx_resolved) = self.transactions.get_mut(&tx.tx_id) {
                    if tx_resolved.client_id != tx.client_id {
                        info!("client id in resolve and resolved transaction do not match");
                        return;
                    }

                    if tx_resolved.disputed != Some(true) {
                        info!("tx {} was not disputed and can't be resolved", tx.tx_id);
                        return;
                    }

                    if let Some(amount) = tx_resolved.amount {
                        account.available += amount;
                        account.held -= amount;
                    }
                    tx_resolved.disputed = Some(false);

                    info!(
                        "tx {} dispute resolved by client {} over {:?}",
                        tx.tx_id, tx_resolved.client_id, tx_resolved.amount
                    );
                } else {
                    info!(
                        "tx {} resolved, but we have no record of this transaction",
                        tx.tx_id
                    );
                }
            }
            Type::Withdrawal | Type::Withdraw => {
                let amount = if tx.amount.is_some() {
                    tx.amount.unwrap()
                } else {
                    info!("tx {}: invalid withdrawal, no amount", tx.tx_id);
                    return;
                };

                if amount > account.available {
                    info!(
                        "tx {}: account {} has insufficient funds to withdraw {}",
                        tx.tx_id, tx.client_id, amount
                    );
                } else {
                    account.available -= amount;
                    account.total -= amount;
                    info!(
                        "tx {}: withdrawing {} from account {}",
                        tx.tx_id, amount, tx.client_id
                    );
                    self.transactions.insert(tx.tx_id, tx);
                }
            }
        }
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
