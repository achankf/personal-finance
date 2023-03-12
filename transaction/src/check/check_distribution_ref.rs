use chrono::{DateTime, Local};
use db::SqlResult;

use crate::Transaction;

pub struct DistributionRefSumZeroCheckResult {
    pub ticker: String,
    pub record_date: DateTime<Local>,
    pub unused_flag: bool,
    pub total_distribution: f64,
    pub taxable_gain: f64,
    pub foreign_tax: f64,
}

impl Transaction<'_> {
    pub async fn check_distribution_ref(
        &mut self,
    ) -> SqlResult<Vec<DistributionRefSumZeroCheckResult>> {
        let result = sqlx::query_file_as!(
            DistributionRefSumZeroCheckResult,
            "src/check/check_distribution_ref.sql"
        )
        .fetch_all(&mut *self.0)
        .await?;

        Ok(result)
    }
}
