use std::{collections::BTreeSet, path::PathBuf};

use common::{deserialize_into_map, Id};
use owo_colors::OwoColorize;
use serde::Deserialize;
use serde_trim::string_trim;

use crate::MyTransaction;

#[derive(Debug, Deserialize)]
pub struct AccountType {
    #[serde(deserialize_with = "string_trim")]
    pub account_type: String,
}

impl Id for AccountType {
    type IdType = String;

    fn id(&self) -> Self::IdType {
        self.account_type.clone()
    }
}

impl MyTransaction<'_> {
    pub async fn upsert_account_type(
        &mut self,
        csv_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("updating data with {}", csv_path.to_string_lossy());

        struct RawData {
            account_type: String,
        }

        let existing_types: BTreeSet<_> =
            sqlx::query_as!(RawData, "select account_type from AccountType")
                .map(|record| record.account_type)
                .fetch_all(&mut *self.0)
                .await?
                .into_iter()
                .collect();

        let parsed_records: BTreeSet<_> = deserialize_into_map::<AccountType>(csv_path)?
            .into_keys()
            .collect();
        println!("{:#?}", parsed_records);

        for account_type in parsed_records.difference(&existing_types) {
            self.upsert_account_type_helper(&account_type).await?;
        }

        Ok(())
    }

    pub async fn upsert_account_type_helper(
        &mut self,
        account_type: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("inserting account type {account_type}");

        let result = sqlx::query_file!("src/upsert/account_type.sql", account_type)
            .execute(&mut *self.0)
            .await;

        match result {
            Ok(result) => {
                if result.rows_affected() != 1 {
                    println!(
                        "{}: account type already existed",
                        "Warning".bold().yellow(),
                    );
                }
                Ok(())
            }
            Err(err) => Err(format!("{}, record: {}", err.to_string(), account_type).into()),
        }
    }
}
