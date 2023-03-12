use common::Id;
use serde::Deserialize;
use serde_trim::string_trim;

use db::Query;

#[derive(Debug, Deserialize)]
pub struct PrepaidAccount {
    #[serde(deserialize_with = "string_trim")]
    account_type: String,
}

impl Id for PrepaidAccount {
    type IdType = String;

    fn id(&self) -> Self::IdType {
        self.account_type.clone()
    }
}

impl Query for PrepaidAccount {
    fn query(&self) -> db::SqlQuery {
        sqlx::query_file!("src/upsert/prepaid_account.sql", self.account_type)
    }
}
