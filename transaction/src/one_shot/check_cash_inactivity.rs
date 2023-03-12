use chrono::{DateTime, Local};
use db::SqlResult;

use crate::MyTransaction;

pub struct InactivityCheckResult {
    pub first_name: String,
    pub last_name: String,
    pub inactive_fee_months: i64,
    pub latest_transaction: DateTime<Local>,
    pub account_name: String,
    pub should_make_a_move: bool,
}

struct RawData {
    first_name: String,
    last_name: String,
    inactive_fee_months: i64,
    latest_transaction: DateTime<Local>,
    account_name: String,
}

impl From<RawData> for InactivityCheckResult {
    fn from(
        RawData {
            first_name,
            last_name,
            inactive_fee_months,
            latest_transaction,
            account_name,
        }: RawData,
    ) -> Self {
        let last_activity_days = (Local::now() - latest_transaction).num_days();

        const AVG_NUM_DAYS_IN_MONTH: f64 = 30.437; // according to Google/Wiki
        let account_inactivity_fee_period =
            (inactive_fee_months as f64 * AVG_NUM_DAYS_IN_MONTH) as i64;

        // make a transaction 2 weeks before you're charged with inactivity fees
        let should_make_a_move = account_inactivity_fee_period - last_activity_days < 10;

        InactivityCheckResult {
            first_name,
            last_name,
            inactive_fee_months,
            latest_transaction,
            account_name,
            should_make_a_move,
        }
    }
}

impl MyTransaction<'_> {
    pub async fn check_cash_inactivity(&mut self) -> SqlResult<Vec<InactivityCheckResult>> {
        let result = sqlx::query_file_as!(RawData, "src/one_shot/check_cash_inactivity.sql")
            .map(Into::into)
            .fetch_all(&mut *self.0)
            .await?;

        Ok(result)
    }
}
