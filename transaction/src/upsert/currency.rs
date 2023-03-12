use common::is_optional_numeric;
use common::Id;
use serde::Deserialize;
use serde_trim::option_string_trim;
use serde_trim::string_trim;

use db::Query;

#[derive(Debug, Deserialize)]
pub struct Currency {
    #[serde(deserialize_with = "string_trim")]
    pub currency: String,
    #[serde(deserialize_with = "string_trim")]
    pub currency_name: String,
    #[serde(deserialize_with = "option_string_trim")]
    pub currency_symbol: Option<String>,
    #[serde(deserialize_with = "is_optional_numeric")]
    pub market_exchange_rate: Option<String>,
}

impl Id for Currency {
    type IdType = String;

    fn id(&self) -> Self::IdType {
        self.currency.clone()
    }
}

impl Query for Currency {
    fn query(&self) -> db::SqlQuery {
        sqlx::query_file!(
            "src/upsert/currency.sql",
            self.currency,
            self.currency_name,
            self.currency_symbol,
            self.market_exchange_rate
        )
    }
}
