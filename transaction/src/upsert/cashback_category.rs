use common::{is_numeric, Id};
use serde::Deserialize;
use serde_trim::string_trim;

use db::Query;

#[derive(Deserialize, Debug)]
pub struct CashbackCategory {
    #[serde(deserialize_with = "string_trim")]
    pub account_type: String,
    #[serde(deserialize_with = "string_trim")]
    pub cashback_category_name: String,
    #[serde(deserialize_with = "is_numeric")]
    pub cashback_rate: String,
}

impl Id for CashbackCategory {
    type IdType = (String, String);

    fn id(&self) -> Self::IdType {
        (
            self.account_type.clone(),
            self.cashback_category_name.clone(),
        )
    }
}

impl Query for CashbackCategory {
    fn query(&self) -> db::SqlQuery {
        sqlx::query_file!(
            "src/upsert/cashback_category.sql",
            self.account_type,
            self.cashback_category_name,
            self.cashback_rate
        )
    }
}
