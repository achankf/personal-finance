mod get_default_folder;
mod print_distribution_check;
mod print_justify_amex;
mod print_transaction;
mod print_transaction_check;
mod rebalance;
mod report;
mod upsert;

use std::path::PathBuf;

use clap::Subcommand;

use db::Db;
use owo_colors::OwoColorize;
use print_transaction::print_transaction;
use rebalance::rebalance;

use transaction::MyTransaction;

use self::{
    get_default_folder::get_default_folder, print_distribution_check::print_distribution_check,
    print_justify_amex::print_justify_amex, print_transaction_check::print_transaction_check,
    report::ReportCommand, upsert::UpsertCommand,
};

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Verify the coherence of the database.
    Check,
    /// Calculates and evaluates whether using the Amex SimplyCash Preferred card is justified, based on the specified number of days.
    JustifyAmex,
    /// Retrieves the next available transaction ID.
    Next,
    /// Displays the allocation required to rebalance the asset.
    Rebalance,
    /// Provides a report of important data from the database.
    Report {
        #[command(subcommand)]
        command: ReportCommand,
    },
    /// Resets the database
    Reset,
    /// Add new entries to the database with the provided CSV file. This doesn't remove any entry. To correct data, reset the database instead.
    Upsert {
        #[command(subcommand)]
        command: UpsertCommand,
    },
    /// Synchronize the list of pending transactions
    SyncPendingTransaction {
        /// The directory containing all the CSV files.
        #[arg(default_value=get_default_folder().into_os_string())]
        csv_folder: PathBuf,
    },
    /// Get the transaction from the database
    Get {
        #[arg(default_value_t = 1)]
        transaction_id: i64,
    },
}

impl Command {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Reset => Ok(Db::reset().await?),
            Self::SyncPendingTransaction { csv_folder } => {
                let path = csv_folder.join("pending_transaction.csv");

                let mut db = Db::new().await?;
                let mut transaction = MyTransaction::try_new(&mut db).await?;

                transaction.sync_pending_transactions(&path).await?;

                Ok(transaction.commit().await?)
            }
            Self::JustifyAmex => {
                let mut db = Db::new().await?;
                let mut transaction = MyTransaction::try_new(&mut db).await?;

                print_justify_amex(&mut transaction).await?;

                Ok(transaction.commit().await?)
            }
            Self::Check => {
                let mut db = Db::new().await?;
                let mut transaction = MyTransaction::try_new(&mut db).await?;

                print_distribution_check(&mut transaction).await?;
                print_transaction_check(&mut transaction).await?;

                Ok(transaction.commit().await?)
            }
            Self::Next => {
                let mut db = Db::new().await?;
                let mut transaction = MyTransaction::try_new(&mut db).await?;

                print_distribution_check(&mut transaction).await?;
                print_transaction_check(&mut transaction).await?;

                let next_transaction_id = transaction.get_next_transaction_id().await?;

                println!(
                    "The succeeding transaction ID is: {}",
                    next_transaction_id.to_string().bold().yellow()
                );

                transaction.get_trade_data().await?;

                Ok(transaction.commit().await?)
            }
            Self::Upsert { command } => command.run().await,
            Self::Rebalance => rebalance().await,
            Self::Report { command } => command.run().await,
            Self::Get { transaction_id } => {
                let mut db = Db::new().await?;
                let mut transaction = MyTransaction::try_new(&mut db).await?;

                print_transaction(&mut transaction, transaction_id).await?;

                Ok(transaction.commit().await?)
            }
        }
    }
}
