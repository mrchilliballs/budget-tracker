#![feature(vec_try_remove)]
// TODO: export as CSV file
// TODO: `clap_complete` completions

use std::{
    fmt::{self, Display, Formatter},
    fs, io,
    num::NonZero,
};

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use rust_decimal::{Decimal, dec};
use serde::{Deserialize, Serialize};

pub const APP_NAME: &str = "Budget Tracker";
pub const APP_ABOUT: &str = "Keeps track of your income and expenses";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Summary {
        // TODO: from-to
    },
    Income {
        #[command(subcommand)]
        command: Option<TransactionCmds>,
    },
    Expense {
        #[command(subcommand)]
        command: Option<TransactionCmds>,
    },
}

#[derive(Subcommand)]
enum TransactionCmds {
    Add { amount: Decimal },
    Remove { index: NonZero<usize> },
    List,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct Transaction {
    // TODO: time
    kind: TransactionKind,
}

impl Transaction {
    fn new(kind: TransactionKind) -> Self {
        Self { kind }
    }

    fn kind(&self) -> TransactionKind {
        self.kind
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransactionKind {
    Income { amount: Decimal },
    Expense { amount: Decimal },
}

impl Display for TransactionKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Income { amount } => write!(f, "+{amount}"),
            Self::Expense { amount } => write!(f, "-{amount}"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct App {
    transactions: Vec<Transaction>,
}

impl App {
    fn build() -> Result<Self> {
        match fs::read_to_string(Self::SAVE_PATH) {
            io::Result::Ok(string) => Ok(toml::from_str(&string)?),
            io::Result::Err(err) if err.kind() == io::ErrorKind::NotFound => {
                let default_app = App {
                    transactions: vec![
                        Transaction::new(TransactionKind::Income { amount: dec!(69) }),
                        Transaction::new(TransactionKind::Expense { amount: dec!(67) }),
                    ],
                };
                default_app.save()?;
                Ok(default_app)
            }
            result @ io::Result::Err(_) => {
                result?;
                unreachable!()
            }
        }
    }
    fn save(&self) -> Result<()> {
        fs::write(Self::SAVE_PATH, toml::to_string_pretty(self)?)?;
        Ok(())
    }
    const SAVE_PATH: &str = "Budget_Tracker.toml";
    fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }
    fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }
    fn remove_transaction(&mut self, index: NonZero<usize>) -> Option<()> {
        self.transactions.try_remove(index.get() - 1).map(|_| ())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        println!("running drop..");
        let _ = self.save();
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Args::parse();
    let mut app = App::build()?;
    let Some(command) = cli.command else {
        return Ok(());
    };
    match command {
        Commands::Summary {} => {
            println!(
                "Total: {}",
                app.transactions()
                    .iter()
                    .fold(dec!(0), |total, transaction| match transaction.kind() {
                        TransactionKind::Income { amount } => total + amount,
                        TransactionKind::Expense { amount } => total - amount,
                    })
            )
        }
        Commands::Income { command: None }
        | Commands::Income {
            command: Some(TransactionCmds::List),
        } => {
            let transactions_iter = app.transactions().iter().filter_map(|transaction| {
                if let TransactionKind::Income { amount } = transaction.kind() {
                    Some(amount)
                } else {
                    None
                }
            });
            let max_figures = transactions_iter
                .clone()
                .max()
                .map(|amount| amount.to_string().len())
                .unwrap_or(1)
                // + positive sign
                + 1;
            for (i, transaction) in transactions_iter.enumerate() {
                println!("{}: {transaction:max_figures$}", i + 1);
            }
        }
        Commands::Expense { command: None }
        | Commands::Expense {
            command: Some(TransactionCmds::List),
        } => {
            let transactions_iter = app.transactions().iter().filter_map(|transaction| {
                if let TransactionKind::Expense { amount } = transaction.kind() {
                    Some(amount)
                } else {
                    None
                }
            });
            let max_figures = transactions_iter
                .clone()
                .max()
                .map(|amount| amount.to_string().len())
                .unwrap_or(1)
                // + negative sign
                + 1;
            for (i, transaction) in transactions_iter.enumerate() {
                println!("{}: {transaction:max_figures$}", i + 1);
            }
        }
        Commands::Income {
            command: Some(TransactionCmds::Add { amount }),
        } => {
            app.add_transaction(Transaction::new(TransactionKind::Income { amount }));
        }
        Commands::Income {
            command: Some(TransactionCmds::Remove { index }),
        } => {
            app.remove_transaction(index);
        }
        Commands::Expense {
            command: Some(TransactionCmds::Add { amount }),
        } => {
            app.add_transaction(Transaction::new(TransactionKind::Expense { amount }));
        }
        Commands::Expense {
            command: Some(TransactionCmds::Remove { index }),
        } => {
            app.remove_transaction(index);
        }
    };
    app.save()?;
    Ok(())
}
