use common::Id;
use serde::Deserialize;
use serde_trim::string_trim;

use db::Query;

#[derive(Deserialize, Debug)]
pub struct Person {
    #[serde(deserialize_with = "string_trim")]
    pub person_key: String,
    #[serde(deserialize_with = "string_trim")]
    pub first_name: String,
    #[serde(deserialize_with = "string_trim")]
    pub last_name: String,
}

impl Id for Person {
    type IdType = String;

    fn id(&self) -> Self::IdType {
        self.person_key.clone()
    }
}

impl Query for Person {
    fn query(&self) -> db::SqlQuery {
        sqlx::query_file!(
            "src/upsert/person.sql",
            self.person_key,
            self.first_name,
            self.last_name
        )
    }
}
