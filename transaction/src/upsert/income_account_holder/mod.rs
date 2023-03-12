use serde::Deserialize;
use serde_trim::string_trim;

mod income_account_holder;

#[derive(Debug, Deserialize)]
pub struct IncomeAccountHolder {
    #[serde(deserialize_with = "string_trim")]
    pub person_key: String,
    #[serde(deserialize_with = "string_trim")]
    pub account_type: String,
}
