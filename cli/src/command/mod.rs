mod print_distribution_check;
mod print_justify_amex;
mod print_transaction_check;
mod rebalance;
mod report;
mod upsert;

use clap::Subcommand;
use db::Db;
use owo_colors::OwoColorize;
use rebalance::rebalance;
use transaction::Transaction;

use self::{
    print_distribution_check::print_distribution_check, print_justify_amex::print_justify_amex,
    print_transaction_check::print_transaction_check, report::ReportCommand, upsert::UpsertCommand,
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
    /// Synchronizes the database with the provided CSV file.
    Upsert {
        #[command(subcommand)]
        command: UpsertCommand,
    },
}

impl Command {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Reset => {
                Db::reset().await?;
                Ok(())
            }
            Self::JustifyAmex => {
                let mut db = Db::new().await?;
                let mut transaction = Transaction::new(&mut db).await?;

                print_justify_amex(&mut transaction).await?;

                transaction.commit().await?;

                Ok(())
            }
            Self::Check => {
                let mut db = Db::new().await?;
                let mut transaction = Transaction::new(&mut db).await?;

                print_distribution_check(&mut transaction).await?;
                print_transaction_check(&mut transaction).await?;

                transaction.commit().await?;

                Ok(())
            }
            Self::Next => {
                let mut db = Db::new().await?;
                let mut transaction = Transaction::new(&mut db).await?;

                print_distribution_check(&mut transaction).await?;
                print_transaction_check(&mut transaction).await?;

                let next_transaction_id = transaction.get_next_transaction_id().await?;

                println!(
                    "The succeeding transaction ID is: {}",
                    next_transaction_id.to_string().bold().yellow()
                );

                transaction.get_trade_data().await?;

                transaction.commit().await?;

                Ok(())
            }
            Self::Upsert { command } => command.run().await,
            Self::Rebalance => {
                rebalance().await?;
                Ok(())
            }
            Self::Report { command } => command.run().await,
        }
    }
}
