use chrono::NaiveDate;
use common::is_numeric;
use serde::Deserialize;
use serde_trim::string_trim;

mod gic_account_holder;

#[derive(Debug, Deserialize)]
pub struct GicAccountHolder {
    #[serde(deserialize_with = "string_trim")]
    pub person_key: String,
    #[serde(deserialize_with = "string_trim")]
    pub account_type: String,
    pub issue_date: NaiveDate,
    pub maturity_date: NaiveDate,
    #[serde(deserialize_with = "is_numeric")]
    pub apr: String,
}
