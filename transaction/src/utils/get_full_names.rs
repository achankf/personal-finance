use std::collections::HashMap;

use db::SqlResult;

use crate::MyTransaction;

impl MyTransaction<'_> {
    pub async fn get_full_names(&mut self) -> SqlResult<HashMap<i64, String>> {
        struct RawData {
            person_id: i64,
            full_name: String,
        }

        Ok(
            sqlx::query_file_as!(RawData, "src/utils/get_full_names.sql")
                .fetch_all(&mut *self.0)
                .await?
                .into_iter()
                .map(|raw| (raw.person_id, raw.full_name))
                .collect(),
        )
    }
}
