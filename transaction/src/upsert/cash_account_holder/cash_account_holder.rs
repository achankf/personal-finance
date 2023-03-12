use std::path::PathBuf;

use common::{deserialize_into_map, Id};
use db::SqlResult;
use owo_colors::OwoColorize;

use sqlx::Sqlite;

use crate::Transaction;

use super::CashAccountHolder;

impl Id for CashAccountHolder {
    type IdType = (String, String);

    fn id(&self) -> Self::IdType {
        (self.person_key.clone(), self.account_type.clone())
    }
}

impl CashAccountHolder {
    pub fn to_account_key(&self) -> String {
        format!("{}-{}", self.person_key, self.account_type).to_uppercase()
    }
}

impl Transaction<'_> {
    pub async fn upsert_cash_account_holder(
        &mut self,
        csv_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("updating data with {}", csv_path.to_string_lossy());

        let parsed_records = deserialize_into_map::<CashAccountHolder>(csv_path)?;

        for (_, record) in &parsed_records {
            let account_key = record.to_account_key();

            println!(
                "> inserting supplementary accounts into Account {}",
                account_key
            );
            self.upsert_cash_account_helper(record, &account_key)
                .await?;

            println!("> inserting account holder");
            let cash_account_holder_id = self.upsert_cash_account_holder_helper(record).await?;

            if let Some(cash_account_holder_id) = cash_account_holder_id {
                println!("> upsert mapping");
                self.upsert_cash_account_mapping_helper(cash_account_holder_id, &account_key)
                    .await
                    .ok();
            }
        }

        Ok(())
    }

    async fn upsert_cash_account_holder_helper(
        &mut self,
        record: &CashAccountHolder,
    ) -> Result<Option<i64>, Box<dyn std::error::Error>> {
        let result = sqlx::query_file!(
            "src/upsert/cash_account_holder/upsert_cash_account_holder.sql",
            record.person_key,
            record.account_type,
            record.emergency_target,
            record.is_closed
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

                    Ok(None)
                } else {
                    Ok(Some(result.last_insert_rowid()))
                }
            }
            Err(err) => {
                return Err(format!("{}, record: {:?}", err.to_string(), record).into());
            }
        }
    }

    async fn upsert_cash_account_mapping_helper(
        &mut self,
        cash_account_holder_id: i64,
        account_key: &str,
    ) -> SqlResult<<Sqlite as sqlx::Database>::QueryResult> {
        let search_term = format!("{account_key}%");

        sqlx::query_file!(
            "src/upsert/cash_account_holder/upsert_cash_account_mapping.sql",
            cash_account_holder_id,
            search_term
        )
        .execute(&mut *self.0)
        .await
    }

    async fn upsert_cash_account_helper(
        &mut self,
        record: &CashAccountHolder,
        account_key: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let result = sqlx::query_file!(
            "src/upsert/cash_account_holder/upsert_cash_account.sql",
            account_key,
            record.account_type,
        )
        .execute(&mut *self.0)
        .await;

        match result {
            Ok(result) => {
                // cash, fees, interest, open balance (digit & fiat), withholding tax, bonus, forex (revenue, expense)
                const NUM_SUBTYPE_ACCOUNTS: u64 = 9;

                if result.rows_affected() != NUM_SUBTYPE_ACCOUNTS {
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
