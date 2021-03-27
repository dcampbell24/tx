use std::env;
use std::process;

mod account;
mod app;
mod transaction;

use app::App;

fn main() {
    let mut app = App::new();

    let tx_filepath = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: `tx <transactions.csv>`");
        process::exit(1);
    });

    if let Err(err) = app.process_transactions(&tx_filepath) {
        eprintln!("error reading transactions: {}", err);
        process::exit(1);
    }

    if let Err(err) = app.print_accounts_as_csv() {
        eprintln!("error serializing accounts: {}", err);
        process::exit(1);
    }
}
