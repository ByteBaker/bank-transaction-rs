#![allow(dead_code, unused_variables)]
use serde::Serialize;

use crate::objects::Account;

/// `AccountSummary` is only for testing purpose
///
/// Allows creation of an `Account` like object
/// without the fields `transactions, disputes`, or `resolves`
#[derive(Serialize, Debug, PartialEq)]
pub struct AccountSummary {
    pub client: u16,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool,
}

impl PartialEq<Account> for AccountSummary {
    fn eq(&self, other: &Account) -> bool {
        self.client == other.client
            && self.available == other.available
            && self.held == other.held
            && self.total == other.total
            && self.locked == other.locked
    }
}

#[cfg(test)]
mod test {

    use std::collections::HashMap;

    use crate::{
        objects::{Account, Bank, Transaction},
        tests::AccountSummary,
    };

    fn get_accounts_summary<'a>(filename: &str, bank: &'a mut Bank) -> HashMap<u16, &'a Account> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(filename)
            .unwrap();

        for result in reader.deserialize() {
            let txn: Transaction = result.unwrap();
            bank.add_txn(txn);
        }

        bank.get_accounts_summary()
    }

    #[test]
    pub fn basic_transaction() {
        const FILE_PATH: &str = "./tests/basic_transaction.csv";
        let expected_result = AccountSummary {
            client: 1,
            available: 7.0,
            held: 0.0,
            total: 7.0,
            locked: false,
        };

        let mut bank = Bank::new();
        let accounts = get_accounts_summary(FILE_PATH, &mut bank);
        assert_eq!(&expected_result, accounts.get(&1).unwrap().to_owned())
    }

    #[test]
    pub fn insufficient_balance() {
        const FILE_PATH: &str = "./tests/insufficient_bal.csv";

        let expected_result = AccountSummary {
            client: 1,
            available: 1.0,
            held: 0.0,
            total: 1.0,
            locked: false,
        };

        let mut bank = Bank::new();
        let accounts = get_accounts_summary(FILE_PATH, &mut bank);
        assert_eq!(&expected_result, accounts.get(&1).unwrap().to_owned())
    }

    #[test]
    pub fn dispute_without_resolve() {
        const FILE_PATH: &str = "./tests/dispute_without_resolve.csv";
        let expected_result = AccountSummary {
            client: 1,
            available: 3.0,
            held: 1.0,
            total: 4.0,
            locked: false,
        };

        let mut bank = Bank::new();
        let accounts = get_accounts_summary(FILE_PATH, &mut bank);
        assert_eq!(&expected_result, accounts.get(&1).unwrap().to_owned())
    }

    #[test]
    pub fn resolve_without_dispute() {
        const FILE_PATH: &str = "./tests/resolve_without_dispute.csv";
        let expected_result = AccountSummary {
            client: 1,
            available: 4.0,
            held: 0.0,
            total: 4.0,
            locked: false,
        };

        let mut bank = Bank::new();
        let accounts = get_accounts_summary(FILE_PATH, &mut bank);
        assert_eq!(&expected_result, accounts.get(&1).unwrap().to_owned())
    }

    #[test]
    pub fn dispute_then_resolve() {
        const FILE_PATH: &str = "./tests/dispute_then_resolve.csv";
        let expected_result = AccountSummary {
            client: 1,
            available: 4.0,
            held: 0.0,
            total: 4.0,
            locked: false,
        };

        let mut bank = Bank::new();
        let accounts = get_accounts_summary(FILE_PATH, &mut bank);
        assert_eq!(&expected_result, accounts.get(&1).unwrap().to_owned())
    }

    #[test]
    pub fn chargeback_after_dispute() {
        const FILE_PATH: &str = "./tests/chargeback_after_dispute.csv";
        let expected_result = AccountSummary {
            client: 1,
            available: 2.0,
            held: 0.0,
            total: 4.0,
            locked: true,
        };

        let mut bank = Bank::new();
        let accounts = get_accounts_summary(FILE_PATH, &mut bank);
        assert_eq!(&expected_result, accounts.get(&1).unwrap().to_owned())
    }

    #[test]
    pub fn chargeback_without_dispute() {
        const FILE_PATH: &str = "./tests/chargeback_without_dispute.csv";
        let expected_result = AccountSummary {
            client: 1,
            available: 4.0,
            held: 0.0,
            total: 4.0,
            locked: false,
        };

        let mut bank = Bank::new();
        let accounts = get_accounts_summary(FILE_PATH, &mut bank);
        assert_eq!(&expected_result, accounts.get(&1).unwrap().to_owned())
    }

    #[test]
    pub fn chargeback_after_resolve() {
        const FILE_PATH: &str = "./tests/chargeback_after_resolve.csv";
        let expected_result = AccountSummary {
            client: 1,
            available: 4.0,
            held: 0.0,
            total: 4.0,
            locked: false,
        };

        let mut bank = Bank::new();
        let accounts = get_accounts_summary(FILE_PATH, &mut bank);
        assert_eq!(&expected_result, accounts.get(&1).unwrap().to_owned())
    }

    #[test]
    pub fn chargeback_then_transactions() {
        const FILE_PATH: &str = "./tests/chargeback_then_transactions.csv";
        let expected_result = AccountSummary {
            client: 1,
            available: 2.0,
            held: 0.0,
            total: 4.0,
            locked: true,
        };

        let mut bank = Bank::new();
        let accounts = get_accounts_summary(FILE_PATH, &mut bank);
        assert_eq!(&expected_result, accounts.get(&1).unwrap().to_owned())
    }
}
