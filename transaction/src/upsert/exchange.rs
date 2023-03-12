use common::Id;
use serde::Deserialize;
use serde_trim::string_trim;

use db::Query;

#[derive(Debug, Deserialize)]
pub struct Exchange {
    #[serde(deserialize_with = "string_trim")]
    pub exchange_key: String,
    #[serde(deserialize_with = "string_trim")]
    pub exchange_name: String,
}

impl Id for Exchange {
    type IdType = String;

    fn id(&self) -> Self::IdType {
        self.exchange_key.clone()
    }
}

impl Query for Exchange {
    fn query(&self) -> db::SqlQuery {
        sqlx::query_file!(
            "src/upsert/exchange.sql",
            self.exchange_key,
            self.exchange_name
        )
    }
}
