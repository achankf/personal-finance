use common::Id;
use serde::Deserialize;
use serde_trim::string_trim;

use db::Query;

#[derive(Deserialize, Debug)]
pub struct CashbackCard {
    #[serde(deserialize_with = "string_trim")]
    pub account_type: String,
}

impl Id for CashbackCard {
    type IdType = String;

    fn id(&self) -> Self::IdType {
        self.account_type.clone()
    }
}

impl Query for CashbackCard {
    fn query(&self) -> db::SqlQuery {
        sqlx::query_file!("src/upsert/cashback_card.sql", self.account_type,)
    }
}
