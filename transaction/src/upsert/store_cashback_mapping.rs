use common::Id;
use serde::Deserialize;
use serde_trim::string_trim;

use db::Query;

#[derive(Deserialize, Debug)]
pub struct StoreCashbackMapping {
    #[serde(deserialize_with = "string_trim")]
    pub store_key: String,
    #[serde(deserialize_with = "string_trim")]
    pub account_type: String,
    #[serde(deserialize_with = "string_trim")]
    pub cashback_category_name: String,
}

impl Id for StoreCashbackMapping {
    type IdType = (String, String);

    fn id(&self) -> Self::IdType {
        (self.store_key.clone(), self.account_type.clone())
    }
}

impl Query for StoreCashbackMapping {
    fn query(&self) -> db::SqlQuery {
        sqlx::query_file!(
            "src/upsert/store_cashback_mapping.sql",
            self.store_key,
            self.account_type,
            self.cashback_category_name
        )
    }
}
