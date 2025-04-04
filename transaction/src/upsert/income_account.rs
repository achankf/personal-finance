use std::path::PathBuf;

use common::{deserialize_into_map, Id};
use owo_colors::OwoColorize;
use serde::Deserialize;
use serde_trim::string_trim;

use crate::MyTransaction;

#[derive(Deserialize, Debug)]
pub struct IncomeAccount {
    #[serde(deserialize_with = "string_trim")]
    pub account_type: String,
    #[serde(deserialize_with = "string_trim")]
    pub account_name: String,
    #[serde(deserialize_with = "string_trim")]
    pub currency: String,
}

impl Id for IncomeAccount {
    type IdType = String;

    fn id(&self) -> Self::IdType {
        self.account_type.clone()
    }
}

impl MyTransaction<'_> {
    pub async fn upsert_income_account(
        &mut self,
        csv_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("updating data with {}", csv_path.to_string_lossy());

        let parsed_records = deserialize_into_map::<IncomeAccount>(csv_path)?;

        for (_, record) in &parsed_records {
            self.upsert_account_type_helper(&record.account_type)
                .await?;

            self.upsert_income_income_account_helper(record).await?;
        }

        Ok(())
    }

    async fn upsert_income_income_account_helper(
        &mut self,
        record: &IncomeAccount,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let result = sqlx::query_file!(
            "src/upsert/income_account.sql",
            record.account_type,
            record.currency,
            record.account_name
        )
        .execute(&mut *self.0)
        .await;

        match result {
            Ok(result) => {
                if result.rows_affected() != 1 {
                    println!(
                        "{}: no row was affected, record: {:?}",
                        "Warning".bold().yellow(),
                        record
                    );
                }

                Ok(())
            }
            Err(err) => Err(format!("{}, record: {:?}", err.to_string(), record).into()),
        }
    }
}
