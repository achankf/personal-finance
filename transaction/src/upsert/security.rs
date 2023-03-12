use common::{is_numeric, Id};
use serde::Deserialize;
use serde_trim::string_trim;

use db::Query;

#[derive(Deserialize, Debug)]
pub struct Security {
    #[serde(deserialize_with = "string_trim")]
    pub exchange_key: String,
    #[serde(deserialize_with = "string_trim")]
    pub currency: String,
    #[serde(deserialize_with = "string_trim")]
    pub ticker: String,
    #[serde(deserialize_with = "string_trim")]
    pub security_name: String,
    #[serde(deserialize_with = "is_numeric")]
    pub price: String,
}

impl Id for Security {
    type IdType = String;

    fn id(&self) -> Self::IdType {
        self.ticker.to_string()
    }
}

impl Query for Security {
    fn query(&self) -> db::SqlQuery {
        sqlx::query_file!(
            "src/upsert/security.sql",
            self.exchange_key,
            self.currency,
            self.ticker,
            self.security_name,
            self.price
        )
    }
}
