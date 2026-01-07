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
use sqlx::{PgPool, postgres::PgPoolOptions};

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

async fn summarize_budget(pool: &PgPool) {
    // sqlx::query
    let rec = sqlx::query!(
        "
            SELECT *
            FROM table;
        "
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Args::parse();
    let Some(command) = cli.command else {
        return Ok(());
    };

    // let pool = PgPool::connect(
    //     "postgres://postgres:budget-tracker@localhost/postgres")
    //     .await?;
    match command {
        Commands::Summary {} => {}
        Commands::Income { command: None }
        | Commands::Income {
            command: Some(TransactionCmds::List),
        } => {}
        Commands::Expense { command: None }
        | Commands::Expense {
            command: Some(TransactionCmds::List),
        } => {}
        Commands::Income {
            command: Some(TransactionCmds::Add { amount, category }),
        } => {}
        Commands::Income {
            command: Some(TransactionCmds::Remove { index }),
        } => {}
        Commands::Expense {
            command: Some(TransactionCmds::Add { amount, category }),
        } => {}
        Commands::Expense {
            command: Some(TransactionCmds::Remove { index }),
        } => {}
    };
    Ok(())
}
