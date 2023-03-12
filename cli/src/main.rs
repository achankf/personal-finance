mod command;
mod tax_shelter_type;
mod utils;

use clap::Parser;
use command::Command;

///  _____  ______
/// |  __ \|  ____|
/// | |__) | |__
/// |  ___/|  __|    (Alfred Chan's) Personal Finance Tracker
/// | |    | |
/// |_|    |_|
/// This is a personal finance tracker designed to be used through the command line. It works alongside CSV files to input data, which is then converted into a Sqlite database and stored at a specified location. Its features include:
///     - Rebalancing of assets such as credit card debts, emergency funds, and stock/ETF allocations.
///     - Calculations for adjusted cost base and capital gain/loss.
///     - Double-entry accounting for every manually entered transaction (which can be quite involved).
/// As this utility is intended solely for personal use, I have not provided documentation on how to use it. Essentially, this serves as "open core" code for others to examine my programming skills.
#[derive(Parser, Debug)]
#[command(author, version, about, verbatim_doc_comment)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Args::parse().command.run().await?;

    Ok(())
}
