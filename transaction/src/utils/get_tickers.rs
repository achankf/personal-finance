use std::collections::HashMap;

use db::SqlResult;

use crate::MyTransaction;

impl MyTransaction<'_> {
    pub async fn get_tickers(&mut self) -> SqlResult<HashMap<i64, String>> {
        struct RawData {
            security_id: i64,
            ticker: String,
        }

        Ok(sqlx::query_file_as!(RawData, "src/utils/get_tickers.sql")
            .fetch_all(&mut *self.0)
            .await?
            .into_iter()
            .map(|raw| (raw.security_id, raw.ticker))
            .collect())
    }
}
