use std::path::PathBuf;

use common::{deserialize_into_map, is_numeric, Id};
use owo_colors::OwoColorize;
use serde::Deserialize;
use serde_trim::string_trim;

use crate::Transaction;

#[derive(Debug, Deserialize)]
pub struct CashAccountProduct {
    #[serde(deserialize_with = "string_trim")]
    pub account_name: String,
    #[serde(deserialize_with = "string_trim")]
    pub account_type: String,
    #[serde(deserialize_with = "string_trim")]
    pub institution_name: String,
    #[serde(deserialize_with = "string_trim")]
    pub tax_shelter_type: String,
    #[serde(deserialize_with = "string_trim")]
    pub currency: String,
    #[serde(deserialize_with = "is_numeric")]
    pub min_balance_waiver: String,
    #[serde(default = "i64max")]
    pub inactive_fee_months: i64,
}

pub fn i64max() -> i64 {
    // essentially no inactivity fee
    i64::MAX
}

impl Id for CashAccountProduct {
    type IdType = String;

    fn id(&self) -> Self::IdType {
        self.account_type.clone()
    }
}

impl Transaction<'_> {
    pub async fn upsert_cash_account(
        &mut self,
        csv_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("updating data with {}", csv_path.to_string_lossy());

        let parsed_records = deserialize_into_map::<CashAccountProduct>(csv_path)?;

        for (_, record) in &parsed_records {
            self.upsert_account_type_helper(&record.account_type)
                .await?;

            self.upsert_cash_cash_account_helper(&record).await?;
        }

        Ok(())
    }

    async fn upsert_cash_cash_account_helper(
        &mut self,
        record: &CashAccountProduct,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("> inserting cash account");

        let result = sqlx::query_file!(
            "src/upsert/cash_account.sql",
            record.account_type,
            record.institution_name,
            record.tax_shelter_type,
            record.currency,
            record.min_balance_waiver,
            record.inactive_fee_months,
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
            Err(err) => {
                return Err(format!("{}, record: {:?}", err.to_string(), record).into());
            }
        }
    }
}
