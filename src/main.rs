use log::info;
use stderrlog::{self, Timestamp};

use std::env;
use std::process;

mod account;
mod app;
mod transaction;

use app::App;

const LOG_INFO: usize = 2;

fn main() {
    if let Ok(value) = env::var("TX_ENABLE_LOGGING") {
        if &value == "true" || &value == "1" {
            stderrlog::new()
                .verbosity(LOG_INFO)
                .timestamp(Timestamp::Microsecond)
                .init()
                .unwrap();
        }
    }

    let mut app = App::new();

    let tx_filepath = env::args().nth(1).unwrap_or_else(|| {
        info!("usage: `tx <transactions.csv>`");
        process::exit(1);
    });

    if let Err(err) = app.process_transactions(&tx_filepath) {
        info!("error reading transactions: {}", err);
        process::exit(1);
    }

    if let Err(err) = app.print_accounts_as_csv() {
        info!("error serializing accounts: {}", err);
        process::exit(1);
    }
}
