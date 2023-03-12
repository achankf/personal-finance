use db::SqlResult;

use crate::Transaction;

use super::NetBalanceRecord;

impl Transaction<'_> {
    pub async fn get_net_asset_balance(&mut self) -> SqlResult<Vec<NetBalanceRecord>> {
        /*
        Splits into 2 types of calculations:
        - one for cash equivalent assets based on summing records together
        - one for stocks based on multiplying number of units to market price
         */
        let rows = sqlx::query_file_as!(NetBalanceRecord, "src/one_shot/get_net_asset_balance.sql")
            .fetch_all(&mut *self.0)
            .await?;

        Ok(rows)
    }
}
