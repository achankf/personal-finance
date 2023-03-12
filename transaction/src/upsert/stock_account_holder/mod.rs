use serde::Deserialize;
use serde_trim::string_trim;

mod stock_account_holder;

#[derive(Deserialize, Debug)]
pub struct StockAccountHolder {
    #[serde(deserialize_with = "string_trim")]
    pub person_key: String,
    #[serde(deserialize_with = "string_trim")]
    pub account_type: String,
    #[serde(deserialize_with = "string_trim")]
    pub ticker: String,
}
