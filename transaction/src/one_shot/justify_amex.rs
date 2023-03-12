use crate::MyTransaction;

pub struct JustifyAmex {
    pub year: i64,
    pub month: i64,
    pub balance: f64,
    pub extra_cashback_rate: f64,
    pub incl_amex_cashback: f64,
    pub excl_amex_cashback: f64,
    pub extra_cashback: f64,
    pub extra_cashback_after_fee: f64,
    pub missed_opportunities: f64,
}

impl MyTransaction<'_> {
    pub async fn justify_amex(&mut self) -> Result<Vec<JustifyAmex>, Box<dyn std::error::Error>> {
        let records = sqlx::query_file_as!(JustifyAmex, "src/one_shot/justify_amex.sql")
            .fetch_all(&mut *self.0)
            .await?;

        Ok(records)
    }
}
