use common::Id;
use serde::Deserialize;
use serde_trim::string_trim;

use db::Query;

#[derive(Debug, Deserialize)]
pub struct Institution {
    #[serde(deserialize_with = "string_trim")]
    institution_name: String,
}

impl Id for Institution {
    type IdType = String;

    fn id(&self) -> Self::IdType {
        self.institution_name.clone()
    }
}

impl Query for Institution {
    fn query(&self) -> db::SqlQuery {
        sqlx::query_file!("src/upsert/institution.sql", self.institution_name)
    }
}
