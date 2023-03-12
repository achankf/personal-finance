use db::SqlResult;
use serde::Serialize;
use sqlx::types::chrono::{DateTime, Local};

use crate::MyTransaction;

#[derive(Serialize, Debug)]
pub struct CurrentCredit {
    pub first_name: String,
    pub last_name: String,
    pub account_name: String,
    pub last_payment_date: Option<DateTime<Local>>,
    pub balance: f64,
    pub has_pad: bool,
}

impl MyTransaction<'_> {
    pub async fn get_current_credit_card_balance(&mut self) -> SqlResult<Vec<CurrentCredit>> {
        let result = sqlx::query_file_as!(
            CurrentCredit,
            "src/one_shot/get_current_credit_card_balance.sql"
        )
        .fetch_all(&mut *self.0)
        .await?;

        Ok(result)
    }
}
