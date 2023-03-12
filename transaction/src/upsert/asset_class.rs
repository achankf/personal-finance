use std::path::PathBuf;

use common::{is_numeric, Id};
use serde::Deserialize;
use serde_trim::string_trim;

use db::Query;

use crate::Transaction;

#[derive(Deserialize, Debug)]
pub struct AssetClass {
    #[serde(deserialize_with = "string_trim")]
    pub person: String,
    #[serde(deserialize_with = "string_trim")]
    pub parent: String,
    #[serde(deserialize_with = "string_trim")]
    pub asset_class_name: String,
    #[serde(deserialize_with = "is_numeric")]
    pub weight: String,
}

impl Id for AssetClass {
    type IdType = (String, String);

    fn id(&self) -> Self::IdType {
        (self.person.clone(), self.asset_class_name.clone())
    }
}

impl Query for AssetClass {
    fn query(&self) -> db::SqlQuery {
        println!("{:#?}", self);
        sqlx::query_file!(
            "src/upsert/asset_class.sql",
            self.person,
            self.parent,
            self.person,
            self.asset_class_name,
            self.weight,
        )
    }
}

impl Transaction<'_> {
    pub async fn upsert_asset_class(
        &mut self,
        csv_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.upsert_all_in_order::<AssetClass>(&csv_path).await?;

        struct Check {
            is_each_person_has_model: bool,
        }

        // make sure each person has a model
        let result = sqlx::query_as!(
            Check,
            r#"
SELECT
    NOT EXISTS (
        SELECT
            person_id
        FROM
            Person
        WHERE
            person_id NOT IN (
                SELECT
                    person_id
                FROM
                    AssetClass
            )
    ) AS "is_each_person_has_model!:bool"
"#
        )
        .fetch_optional(&mut *self.0)
        .await?;

        if let Some(Check {
            is_each_person_has_model,
        }) = result
        {
            if is_each_person_has_model {
                return Ok(());
            }
        }

        Err("Someone does not have a model (AssetClass)".into())
    }
}
