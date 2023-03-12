use common::{excel_date_optional_time_format, is_numeric, is_optional_numeric, Id};
use serde::Deserialize;
use serde_trim::string_trim;

use db::Query;

#[derive(Debug, Deserialize)]
pub struct FinancialEntry {
    pub transaction_id: i64,
    pub item_id: i64,
    #[serde(deserialize_with = "excel_date_optional_time_format")]
    pub date: i64,
    #[serde(deserialize_with = "string_trim")]
    pub account_key: String,
    #[serde(deserialize_with = "is_optional_numeric")]
    pub forex_rate: Option<String>,
    #[serde(deserialize_with = "is_numeric")]
    pub unit: String,
    #[serde(deserialize_with = "is_optional_numeric")]
    pub debit: Option<String>,
    #[serde(deserialize_with = "is_optional_numeric")]
    pub credit: Option<String>,
    #[serde(deserialize_with = "string_trim")]
    pub description: String,
}

impl Id for FinancialEntry {
    type IdType = (i64, i64);

    fn id(&self) -> (i64, i64) {
        (self.transaction_id, self.item_id)
    }
}

impl Query for FinancialEntry {
    fn query(&self) -> db::SqlQuery {
        sqlx::query_file!(
            "src/upsert/financial_entry.sql",
            self.transaction_id,
            self.item_id,
            self.date,
            self.account_key,
            self.forex_rate,
            self.unit,
            self.debit,
            self.credit,
            self.description
        )
    }
}
