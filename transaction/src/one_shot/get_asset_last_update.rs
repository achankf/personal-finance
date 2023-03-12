use db::SqlResult;
use serde::Serialize;
use sqlx::types::chrono::{DateTime, Local};

use crate::MyTransaction;

#[derive(Serialize, Debug)]
pub struct AccountLatestTransaction {
    pub first_name: String,
    pub last_name: String,
    pub account_name: String,
    pub last_update: DateTime<Local>,
}

impl MyTransaction<'_> {
    pub async fn get_account_latest_transaction(
        &mut self,
    ) -> SqlResult<Vec<AccountLatestTransaction>> {
        let result = sqlx::query_file_as!(
            AccountLatestTransaction,
            "src/one_shot/get_asset_last_update.sql"
        )
        .fetch_all(&mut *self.0)
        .await?;

        Ok(result)
    }
}
