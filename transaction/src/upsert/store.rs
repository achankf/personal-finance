use common::Id;
use serde::Deserialize;
use serde_trim::string_trim;

use db::Query;

#[derive(Debug, Deserialize)]
pub struct Store {
    #[serde(deserialize_with = "string_trim")]
    pub store_key: String,
    #[serde(deserialize_with = "string_trim")]
    pub store_name: String,
}

impl Id for Store {
    type IdType = String;

    fn id(&self) -> Self::IdType {
        self.store_key.clone()
    }
}

impl Query for Store {
    fn query(&self) -> db::SqlQuery {
        sqlx::query_file!("src/upsert/store.sql", self.store_key, self.store_name)
    }
}
