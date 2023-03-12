use db::SqlResult;

use crate::Transaction;

impl Transaction<'_> {
    pub async fn has_cash_account(&mut self, account_type: &str) -> SqlResult<bool> {
        let result = sqlx::query_file!("src/utils/has_cash_account.sql", account_type)
            .fetch_one(&mut *self.0)
            .await?;

        Ok(result.is_account_exist)
    }
}
