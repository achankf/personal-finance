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
        let records = sqlx::query_as!(
            CheckTransactionStore,
            r#"
SELECT
    transaction_id,
    account_key,
    date AS "date!:DateTime<Local>",
    ROUND(
        unit * (COALESCE(debit, 0.0) - COALESCE(credit, 0.0)),
        2
    ) AS "balance!:f64",
    description
FROM
    CashbackTransaction
    INNER JOIN Account USING (account_id)
WHERE
    transaction_id NOT IN (
        SELECT
            transaction_id
        FROM
            TransactionStore
    )
ORDER BY
    account_key,
    date
"#
        )
        .fetch_all(&mut *self.0)
        .await?;

        Ok(records)
    }
}
