use std::collections::HashMap;

use db::SqlResult;

use crate::Transaction;

impl Transaction<'_> {
    pub async fn get_account_names(&mut self) -> SqlResult<HashMap<i64, String>> {
        struct RawData {
            account_id: i64,
            account_name: String,
        }

        Ok(
            sqlx::query_file_as!(RawData, "src/utils/get_account_names.sql")
                .fetch_all(&mut *self.0)
                .await?
                .into_iter()
                .map(|record| (record.account_id, record.account_name))
                .collect(),
        )
    }
}
