use rust_decimal::Decimal;
use serde::Deserialize;

use crate::account::ClientId;

pub type TxId = u32;

/// A financial transaction meant to be read from a CSV file.
///
/// Transactions are chronologically ordered in a transactions file, but tx IDs may
/// be out of order.
#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub type_: Type,
    // An ID unique to each client.
    #[serde(rename = "client")]
    pub client_id: ClientId,
    // An ID unique to each tade.
    #[serde(rename = "tx")]
    pub tx_id: TxId,
    // A decimal with a precision to four places.
    pub amount: Option<Decimal>,
    pub disputed: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub enum Type {
    #[serde(rename = "chargeback")]
    Chargeback,
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "dispute")]
    Dispute,
    #[serde(rename = "resolve")]
    Resolve,
    #[serde(rename = "withdraw")]
    Withdraw,
    #[serde(rename = "withdrawal")]
    Withdrawal,
}
