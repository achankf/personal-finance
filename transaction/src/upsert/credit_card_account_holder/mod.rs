use common::bool_from_str;

use serde::Deserialize;
use serde_trim::{option_string_trim, string_trim};

mod credit_card_account_holder;

#[derive(Debug, Deserialize)]
pub struct CreditCardHolder {
    #[serde(deserialize_with = "string_trim")]
    pub person_key: String,
    #[serde(deserialize_with = "string_trim")]
    pub account_type: String,
    #[serde(deserialize_with = "option_string_trim")]
    pub pad_source: Option<String>,
    #[serde(deserialize_with = "bool_from_str")]
    pub is_closed: bool,
}
