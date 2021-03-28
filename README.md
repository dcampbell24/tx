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
given as its first argument and log the processing of each
transaction to stderr. Once processing is complete tx writes the
final state of each account referenced to stdout in CSV format.

## Example

```
cargo run -- examples/transactions.csv
tx 1: deposited 1 into account 1
tx 2: deposited 2 into account 2
tx 3: deposited 2 into account 1
tx 4: withdrew 1.5 from account 1
tx 5: account 2 has insufficient funds to withdraw 3
client_id,available,held,total,locked
2,2,0.0000,2,false
1,1.5,0.0000,1.5,false
```
