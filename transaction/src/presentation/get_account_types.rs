use std::collections::HashMap;

use db::SqlResult;

use crate::Transaction;

impl Transaction<'_> {
    pub async fn get_account_types(&mut self) -> SqlResult<HashMap<i64, String>> {
        struct RawData {
            account_type_id: i64,
            account_type: String,
        }

        Ok(sqlx::query_as!(
            RawData,
            r#"
SELECT
    account_type_id,
    account_type
FROM
    AccountType
"#,
        )
        .fetch_all(&mut *self.0)
        .await?
        .into_iter()
        .map(|record| (record.account_type_id, record.account_type))
        .collect())
    }
}
