use std::ops::Range;

use chrono::{DateTime, Local};
use db::SqlResult;

use crate::MyTransaction;

use super::NetBalanceRecord;

impl MyTransaction<'_> {
    pub async fn get_net_revenue_balance(
        &mut self,
        range: Range<DateTime<Local>>,
    ) -> SqlResult<Vec<NetBalanceRecord>> {
        let start_ts = range.start.timestamp();
        let end_ts = range.end.timestamp();

        let rows = sqlx::query_file_as!(
            NetBalanceRecord,
            "src/one_shot/get_net_revenue_balance.sql",
            start_ts,
            end_ts
        )
        .fetch_all(&mut *self.0)
        .await?;

        Ok(rows)
    }
}
