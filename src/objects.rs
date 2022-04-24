use std::collections::HashMap;

use serde::{self, Deserialize, Serialize};

use crate::tests::AccountSummary;

/// Types of transactions represented as an enum
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TxType {
    ChargeBack,
    Deposit,
    Dispute,
    Resolve,
    Withdrawal,
}

/// Deserializes a row from CSV
/// and is directly saved in an `Account`
#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    r#type: TxType,
    pub client: u16,
    tx: u32,
    amount: Option<f32>,
}

/// Account corresponding to each client
///
/// More like a bank account
#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub client: u16,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool,
    #[serde(skip)]
    transactions: HashMap<u32, Transaction>,
    #[serde(skip)]
    disputes: HashMap<u32, Transaction>,
    #[serde(skip)]
    resolves: HashMap<u32, Transaction>,
}

impl Account {
    pub fn new(client_id: u16) -> Self {
        Self {
            client: client_id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
            transactions: HashMap::new(),
            disputes: HashMap::new(),
            resolves: HashMap::new(),
        }
    }

    pub fn apply_txn(&mut self, txn: Transaction) {
        if self.locked {
            return;
        }
        match txn.r#type {
            TxType::ChargeBack => {
                // This ensures a chargeback does not happen if
                // A resolve has already happened against a dispute
                if self.disputes.contains_key(&txn.tx) && !self.resolves.contains_key(&txn.tx) {
                    if let Some(tx) = self.transactions.get(&txn.tx) {
                        self.locked = true;
                        let amt = tx.amount.unwrap_or_default();
                        self.held -= amt;
                        self.available -= amt;
                    }
                }
            }
            TxType::Deposit => {
                let amt = txn.amount.unwrap_or_default();
                self.available += amt;
                self.total += amt;

                self.transactions.insert(txn.tx, txn);
            }
            TxType::Dispute => {
                if let Some(tx) = self.transactions.get(&txn.tx) {
                    let amt = tx.amount.unwrap_or_default();
                    self.held += amt;
                    self.available -= amt;

                    self.disputes.insert(txn.tx, txn);
                }
            }
            TxType::Resolve => {
                if self.disputes.contains_key(&txn.tx) {
                    if let Some(tx) = self.transactions.get(&txn.tx) {
                        let amt = tx.amount.unwrap_or_default();
                        self.held -= amt;
                        self.available += amt;

                        self.resolves.insert(txn.tx, txn);
                    }
                }
            }
            TxType::Withdrawal => {
                let amt = txn.amount.unwrap_or_default();
                if amt <= self.available {
                    self.available -= amt;
                    self.total -= amt;

                    self.transactions.insert(txn.tx, txn);
                }
            }
        }
    }
}

/// A bank that can store accounts
/// and process transactions
#[derive(Default)]
pub struct Bank {
    accounts: HashMap<u16, Account>,
}

impl Bank {
    pub fn add_txn(&mut self, t: Transaction) {
        match self.accounts.get_mut(&t.client) {
            Some(account) => account.apply_txn(t),
            None => {
                let client = t.client;
                let mut new_client = Account::new(client);
                new_client.apply_txn(t);
                self.accounts.insert(client, new_client);
            }
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_accounts_iter(&self) -> impl Iterator<Item = &Account> + '_ {
        self.accounts.iter().map(|(_, account)| account)
    }

    /// Shouldn't be all that inefficient because it stores
    /// only the references
    ///
    /// Only meant to be used for unit tests
    #[allow(dead_code)]
    pub fn get_accounts_summary(&self) -> HashMap<u16, &Account> {
        self.accounts
            .iter()
            .map(|(client_id, account)| (client_id.clone(), account))
            .collect::<HashMap<u16, &Account>>()
    }
}

impl PartialEq<AccountSummary> for Account {
    fn eq(&self, other: &AccountSummary) -> bool {
        self.client == other.client
            && self.available == other.available
            && self.held == other.held
            && self.total == other.total
            && self.locked == other.locked
    }
}
