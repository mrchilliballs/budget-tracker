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
    Add { amount: Decimal, category: String },
    Remove { index: NonZero<usize> },
    List,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transaction {
    // TODO: time
    kind: TransactionKind,
    amount: Decimal,
    category: String,
}

impl Transaction {
    fn new(kind: TransactionKind, amount: Decimal, category: String) -> Self {
        Self {
            kind,
            amount,
            category,
        }
    }

    fn kind(&self) -> TransactionKind {
        self.kind
    }

    fn amount(&self) -> Decimal {
        self.amount
    }

    fn category(&self) -> &str {
        &self.category
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.kind {
            TransactionKind::Income => write!(f, "+{}", self.amount),
            TransactionKind::Expense => write!(f, "-{}", self.amount),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransactionKind {
    Income,
    Expense,
}

#[derive(Debug, Serialize, Deserialize)]
struct App {
    transactions: Vec<Transaction>,
}

impl App {
    const SAVE_PATH: &'static str = "Budget_Tracker.toml";
    fn build() -> Result<Self> {
        match fs::read_to_string(Self::SAVE_PATH) {
            io::Result::Ok(string) => Ok(toml::from_str(&string)?),
            io::Result::Err(err) if err.kind() == io::ErrorKind::NotFound => {
                let default_app = App {
                    transactions: vec![
                        Transaction::new(
                            TransactionKind::Income,
                            dec!(69),
                            "groceries".to_string(),
                        ),
                        Transaction::new(TransactionKind::Expense, dec!(67), "rent".to_string()),
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
            let total = app
                .transactions()
                .iter()
                .fold(dec!(0), |total, transaction| match transaction.kind() {
                    TransactionKind::Income => total + transaction.amount(),
                    TransactionKind::Expense => total - transaction.amount(),
                });

            println!("Total: {}{total}", if total > dec!(0) { "+" } else { "" })
        }
        Commands::Income { command: None }
        | Commands::Income {
            command: Some(TransactionCmds::List),
        } => {
            let transactions_iter = app.transactions().iter().filter_map(|transaction| {
                if let TransactionKind::Income = transaction.kind() {
                    Some(transaction.amount())
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
                if let TransactionKind::Expense = transaction.kind() {
                    Some(transaction.amount())
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
            command: Some(TransactionCmds::Add { amount, category }),
        } => {
            app.add_transaction(Transaction::new(TransactionKind::Income, amount, category));
        }
        Commands::Income {
            command: Some(TransactionCmds::Remove { index }),
        } => {
            app.remove_transaction(index);
        }
        Commands::Expense {
            command: Some(TransactionCmds::Add { amount, category }),
        } => {
            app.add_transaction(Transaction::new(TransactionKind::Expense, amount, category));
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
