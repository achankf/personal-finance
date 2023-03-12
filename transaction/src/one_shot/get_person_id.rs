use db::SqlResult;

use crate::Transaction;

impl Transaction<'_> {
    pub async fn get_person_id(&mut self, person_key: &str) -> SqlResult<Option<i64>> {
        struct QueryResult {
            person_id: i64,
        }

        let rows = sqlx::query_file_as!(QueryResult, "src/one_shot/get_person_id.sql", person_key)
            .fetch_all(&mut *self.0)
            .await?;

        if rows.is_empty() {
            Ok(None)
        } else if rows.len() != 1 {
            unreachable!()
        } else {
            Ok(Some(rows[0].person_id))
        }
    }
}
