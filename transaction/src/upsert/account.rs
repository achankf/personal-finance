use common::Id;
use serde::Deserialize;

use db::Query;

#[derive(Debug, Deserialize)]
pub struct Account {
    pub account_key: String,
    pub account_subtype: String,
    pub account_type: String,
    pub stock_ticker: Option<String>,
    pub account_name: String,
}

impl Id for Account {
    type IdType = String;

    fn id(&self) -> Self::IdType {
        self.account_key.to_string()
    }
}

impl Query for Account {
    fn query(&self) -> db::SqlQuery {
        sqlx::query_file!(
            "src/upsert/account.sql",
            self.account_key,
            self.account_subtype,
            self.account_type,
            self.account_name,
        )
    }
}
