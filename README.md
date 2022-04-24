# Bank-Transactions.rs

This crate takes input from a CSV file containing transaction data of several clients and summarises the final status of all the accounts

It can be run using
```sh
cargo run -- filename.csv
```
which generates **accounts.csv**

- The repo contains a sample file called ***transactions.csv*** that can be used to run the program.

## Running tests
```
cargo test
```

## Assumptions made
- Two clients can have the same transaction ID. Transaction IDs are unique per client
- No transaction can take place once the account is frozen
- A chargeback can't take place if the dispute has already been resolved or there was no dispute


## Source file (_transactions.csv_)
- It is manually populated to simulate transactions for 2 clients

## Test cases
- All test cases are written inside **src/tests.rs**
- Source files for all test cases can be found inside **tests/** directory
