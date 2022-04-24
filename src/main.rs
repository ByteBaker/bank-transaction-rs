mod objects;
mod tests;

use std::{path::PathBuf, process::exit};

use csv;

use crate::objects::{Bank, Transaction};


/// Takes the input filename from STDIN
/// and converts to PathBuf
fn get_args() -> PathBuf {
    match std::env::args().nth(1) {
        Some(filename) => filename.into(),
        None => {
            eprintln!("No filename given");
            exit(1)
        }
    }
}

/// Reads a CSV file and loads the rows into `Bank`
/// by parsing them as `Transaction`
fn load_transactions_from_file(bank: &mut Bank, file_path: PathBuf) {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)
        .expect("File not found");

    for result in reader.deserialize() {
        let txn: Transaction = result.unwrap();
        bank.add_txn(txn);
    }
}

/// Serializes `Transaction`s from a `Bank` into CSV rows
/// and saves them to a file
fn save_transactions_to_file(bank: &Bank, file_path: &str) {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(file_path)
        .unwrap();

    for summary in bank.get_accounts_iter() {
        writer.serialize(summary).ok();
    }
}

fn main() {
    let input_path = get_args();
    const OUTPUT_PATH: &str = "./accounts.csv";

    let mut bank = Bank::new();
    load_transactions_from_file(&mut bank, input_path);
    save_transactions_to_file(&bank, OUTPUT_PATH);
}
