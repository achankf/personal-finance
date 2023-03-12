use common::Id;
use serde::Deserialize;
use serde_trim::string_trim;

use db::Query;

#[derive(Debug, Deserialize)]
pub struct AccountSubtype {
    #[serde(deserialize_with = "string_trim")]
    pub account_kind: String,
    #[serde(deserialize_with = "string_trim")]
    pub account_subtype: String,
}

impl Id for AccountSubtype {
    type IdType = String;

    fn id(&self) -> Self::IdType {
        self.account_subtype.clone()
    }
}

impl Query for AccountSubtype {
    fn query(&self) -> db::SqlQuery {
        sqlx::query_file!(
            "src/upsert/account_subtype.sql",
            self.account_kind,
            self.account_subtype,
        )
    }
}
