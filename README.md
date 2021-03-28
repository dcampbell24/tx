# TX

tx is a simple engine for processing simple transactions of some imaginary money
going into imaginary accounts. tx loves processing transactions. tx will not
let one invalid transaction stop any valid transaction.

## Usage

```
tx <transactions.csv>
```

```
cargo run -- <transactions.csv>
```

When you run tx it will process all of the transactions in the file
given as its first argument and log the processing of each transaction with the
info! macro. Setting the environment variable "TX_ENABLE_LOGGING"
will turn on info level logging to stderr. Once processing is complete tx writes
the final state of each account referenced to stdout in CSV format.

## Example

```
$ export TX_ENABLE_LOGGING=1; cargo run -- examples/transactions.csv
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/tx examples/transactions.csv`
2021-03-27T22:29:58.612600-04:00 - INFO - tx 1: deposited 1 into account 1
2021-03-27T22:29:58.612819-04:00 - INFO - tx 2: deposited 2 into account 2
2021-03-27T22:29:58.612959-04:00 - INFO - tx 3: deposited 2 into account 1
2021-03-27T22:29:58.613093-04:00 - INFO - tx 4: withdrew 1.5 from account 1
2021-03-27T22:29:58.613226-04:00 - INFO - tx 5: account 2 has insufficient funds to withdraw 3
client_id,available,held,total,locked
1,1.5,0.0000,1.5,false
2,2,0.0000,2,false
```

## Correctness
My belief in the correctness of this application is based mainly off of an
attempt to use the type system to enforce correctness and running integration
tests against sample data sets. In order to understand what is going on there
are lots of log messages when logging is turned on. The project would benefit
from some tests that can be run by `cargo test` and the ability to run
benchmarks on the nightly version of Rust to test the throughput of this engine.
