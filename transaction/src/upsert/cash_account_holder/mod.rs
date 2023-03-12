use ::serde::Deserialize;
use common::{bool_from_str, is_numeric};
use serde_trim::string_trim;

mod cash_account_holder;

#[derive(Debug, Deserialize)]
pub struct CashAccountHolder {
    #[serde(deserialize_with = "string_trim")]
    pub person_key: String,
    #[serde(deserialize_with = "string_trim")]
    pub account_type: String,
    #[serde(deserialize_with = "is_numeric")]
    pub emergency_target: String,
    #[serde(deserialize_with = "bool_from_str")]
    pub is_closed: bool,
}
