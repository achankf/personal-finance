use db::SqlResult;

use crate::Transaction;

impl Transaction<'_> {
    pub async fn get_next_transaction_id(&mut self) -> SqlResult<i64> {
        struct Record {
            transaction_id: i64,
        }

        let record = sqlx::query_file_as!(Record, "src/one_shot/get_next_transaction_id.sql")
            .fetch_one(&mut *self.0)
            .await?;

        Ok(record.transaction_id)
    }
}
