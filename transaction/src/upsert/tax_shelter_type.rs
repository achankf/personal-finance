use common::Id;
use serde::Deserialize;
use serde_trim::string_trim;

use db::Query;

#[derive(Debug, Deserialize)]
pub struct TaxShelterType {
    #[serde(deserialize_with = "string_trim")]
    tax_shelter_type: String,
    #[serde(deserialize_with = "string_trim")]
    tax_shelter_name: String,
}

impl Id for TaxShelterType {
    type IdType = String;

    fn id(&self) -> Self::IdType {
        self.tax_shelter_type.clone()
    }
}

impl Query for TaxShelterType {
    fn query(&self) -> db::SqlQuery {
        sqlx::query_file!(
            "src/upsert/tax_shelter_type.sql",
            self.tax_shelter_type,
            self.tax_shelter_name
        )
    }
}
