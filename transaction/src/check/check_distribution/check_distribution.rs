use db::SqlResult;

use crate::MyTransaction;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Local;

pub struct DistributionCheckDistributionError {
    pub transaction_id: i64,
    pub ticker: String,
}

pub struct DistributionCheckFinancialEntryError {
    pub account_key: String,
    pub ticker: String,
    pub ex_dividend_date: DateTime<Local>,
    pub record_date: DateTime<Local>,
    pub payment_date: DateTime<Local>,
    pub transaction_id: Option<i64>,
    pub held_unit: Option<f64>,
}

pub struct DistributionCheckResult {
    pub distribution_errors: Vec<DistributionCheckDistributionError>,
    pub financial_entry_errors: Vec<DistributionCheckFinancialEntryError>,
}

impl MyTransaction<'_> {
    /// Ensures Financial distribution entries matches the reference data (and vice-versa).
    pub async fn check_distribution(&mut self) -> SqlResult<DistributionCheckResult> {
        let distribution_errors = sqlx::query_file_as!(
            DistributionCheckDistributionError,
            "src/check/check_distribution/check_distribution.sql"
        )
        .fetch_all(&mut *self.0)
        .await?;

        let financial_entry_errors = sqlx::query_file_as!(
            DistributionCheckFinancialEntryError,
            "src/check/check_distribution/check_financial_entry.sql"
        )
        .fetch_all(&mut *self.0)
        .await?;

        Ok(DistributionCheckResult {
            distribution_errors,
            financial_entry_errors,
        })
    }
}
