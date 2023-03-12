use db::SqlResult;
use serde::Deserialize;

use crate::Transaction;

#[derive(Deserialize, Debug)]
pub struct EmergencyRebalance {
    pub first_name: String,
    pub last_name: String,
    pub account_name: String,
    pub currency: String,
    pub currency_symbol: String,
    pub unallocated_fund: f64,
    pub injection_needed: f64,
}

impl Transaction<'_> {
    pub async fn get_emergency_rebalance(&mut self) -> SqlResult<Vec<EmergencyRebalance>> {
        let result = sqlx::query_file_as!(
            EmergencyRebalance,
            "src/one_shot/get_emergency_rebalance.sql"
        )
        .fetch_all(&mut *self.0)
        .await?;

        Ok(result)
    }
}
