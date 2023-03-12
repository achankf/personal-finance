use std::collections::HashMap;

use db::SqlResult;

use crate::Transaction;

impl Transaction<'_> {
    pub async fn get_full_names(&mut self) -> SqlResult<HashMap<i64, String>> {
        struct RawData {
            person_id: i64,
            full_name: String,
        }

        Ok(sqlx::query_as!(
            RawData,
            r#"
SELECT
    person_id,
    first_name || ' ' || last_name AS full_name
FROM
    Person
"#
        )
        .fetch_all(&mut *self.0)
        .await?
        .into_iter()
        .map(|raw| (raw.person_id, raw.full_name))
        .collect())
    }
}
