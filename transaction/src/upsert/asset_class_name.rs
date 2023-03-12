use common::Id;
use serde::Deserialize;
use serde_trim::string_trim;

use db::Query;

#[derive(Deserialize, Debug)]
pub struct AssetClassName {
    #[serde(deserialize_with = "string_trim")]
    pub asset_class_name: String,
}

impl Id for AssetClassName {
    type IdType = String;

    fn id(&self) -> Self::IdType {
        self.asset_class_name.clone()
    }
}

impl Query for AssetClassName {
    fn query(&self) -> db::SqlQuery {
        sqlx::query_file!("src/upsert/asset_class_name.sql", self.asset_class_name)
    }
}
