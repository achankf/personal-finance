use std::path::PathBuf;

use transaction::{FinancialEntry, MyTransaction};

use crate::command::{
    print_distribution_check::print_distribution_check,
    print_transaction_check::print_transaction_check,
};

pub async fn upsert_transaction(
    transaction: &mut MyTransaction<'_>,
    csv_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    transaction.upsert_all::<FinancialEntry>(csv_path).await?;

    print_distribution_check(transaction).await?;
    print_transaction_check(transaction).await?;

    Ok(())
}
