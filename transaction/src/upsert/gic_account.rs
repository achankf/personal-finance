use std::path::PathBuf;

use common::{deserialize_into_map, Id};
use owo_colors::OwoColorize;
use serde::Deserialize;
use serde_trim::string_trim;

use crate::MyTransaction;

#[derive(Deserialize, Debug)]
pub struct GicAccount {
    #[serde(deserialize_with = "string_trim")]
    pub account_type: String,
}

impl Id for GicAccount {
    type IdType = String;

    fn id(&self) -> String {
        self.account_type.clone()
    }
}

impl MyTransaction<'_> {
    pub async fn upsert_gic_account(
        &mut self,
        csv_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("updating data with {}", csv_path.to_string_lossy());

        let parsed_records = deserialize_into_map::<GicAccount>(csv_path)?;

        for (_, record) in &parsed_records {
            if !self.has_cash_account(&record.account_type).await? {
                return Err(format!(
                    "cannot insert {} as a stock account unless it is also a cash account",
                    record.account_type
                )
                .into());
            }

            self.upsert_gic_gic_account_helper(record).await?;
        }

        Ok(())
    }

    async fn upsert_gic_gic_account_helper(
        &mut self,
        record: &GicAccount,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let result = sqlx::query_file!("src/upsert/gic_account.sql", record.account_type)
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
