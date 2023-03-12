use std::collections::HashMap;

use crate::MyTransaction;

impl MyTransaction<'_> {
    pub async fn get_account_types(
        &mut self,
    ) -> Result<HashMap<i64, String>, Box<dyn std::error::Error>> {
        struct RawData {
            account_type_id: i64,
            account_type: String,
        }

        Ok(
            sqlx::query_file_as!(RawData, "src/utils/get_account_types.sql")
                .fetch_all(&mut *self.0)
                .await?
                .into_iter()
                .map(|record| (record.account_type_id, record.account_type))
                .collect(),
        )
    }
}
