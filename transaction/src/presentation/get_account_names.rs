use std::collections::HashMap;

use db::SqlResult;

use crate::Transaction;

impl Transaction<'_> {
    pub async fn get_account_names(&mut self) -> SqlResult<HashMap<i64, String>> {
        struct RawData {
            account_id: i64,
            account_name: String,
        }

        Ok(sqlx::query_as!(
            RawData,
            r#"
SELECT
    account_id,
    account_name
FROM
    Account
"#,
        )
        .fetch_all(&mut *self.0)
        .await?
        .into_iter()
        .map(|record| (record.account_id, record.account_name))
        .collect())
    }
}
