use chrono::{DateTime, Local};
use db::SqlResult;

use crate::Transaction;

pub struct CheckTransactionStore {
    pub account_key: String,
    pub transaction_id: i64,
    pub date: DateTime<Local>,
    pub balance: f64,
    pub description: String,
}

impl Transaction<'_> {
    pub async fn check_transaction_store(&mut self) -> SqlResult<Vec<CheckTransactionStore>> {
        let records = sqlx::query_file_as!(
            CheckTransactionStore,
            "src/check/check_transaction_store.sql"
        )
        .fetch_all(&mut *self.0)
        .await?;

        Ok(records)
    }
}
