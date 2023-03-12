use db::SqlResult;

use crate::MyTransaction;

impl MyTransaction<'_> {
    pub async fn get_security_id(&mut self, ticker: &str) -> SqlResult<Option<i64>> {
        struct QueryResult {
            security_id: i64,
        }

        let rows = sqlx::query_file_as!(QueryResult, "src/one_shot/get_security_id.sql", ticker)
            .fetch_all(&mut *self.0)
            .await?;

        if rows.is_empty() {
            Ok(None)
        } else if rows.len() != 1 {
            unreachable!()
        } else {
            Ok(Some(rows[0].security_id))
        }
    }
}
