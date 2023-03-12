use serde::Deserialize;

use crate::MyBigDecimal;

mod get_balance;

#[derive(Clone, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BalanceRecord {
    pub name: String,
    pub account_name: String,
    pub balance: MyBigDecimal,
}
