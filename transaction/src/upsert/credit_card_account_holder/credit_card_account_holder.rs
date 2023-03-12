use std::path::PathBuf;

use common::{deserialize_into_map, Id};
use db::SqlResult;
use owo_colors::OwoColorize;

use crate::MyTransaction;

use super::CreditCardHolder;

impl Id for CreditCardHolder {
    type IdType = (String, String);

    fn id(&self) -> Self::IdType {
        (self.person_key.clone(), self.account_type.clone())
    }
}

impl CreditCardHolder {
    pub fn get_account_key(&self) -> String {
        format!("{}-{}", self.person_key, self.account_type)
    }
}

impl MyTransaction<'_> {
    pub async fn upsert_credit_card_holder(
        &mut self,
        csv_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("updating data with {}", csv_path.to_string_lossy());

        let parsed_records = deserialize_into_map::<CreditCardHolder>(csv_path)?;

        for (_, record) in &parsed_records {
            let account_key = record.get_account_key();

            println!("> inserting credit card account {account_key}");
            self.upsert_credit_card_account_helper(&account_key, &record.account_type)
                .await?;

            println!("> inserting credit card account holder");
            let id = self
                .upsert_credit_card_account_holder_helper(record)
                .await?;

            if let Some(credit_card_holder_id) = id {
                println!("> inserting credit card account mapping");
                self.upsert_credit_card_account_mapping_helper(credit_card_holder_id, &account_key)
                    .await?;

                println!("> inserting credit card PAD source");
                self.upsert_credit_card_pad_source(credit_card_holder_id, &record.pad_source)
                    .await?;
            }
        }

        Ok(())
    }

    async fn upsert_credit_card_pad_source(
        &mut self,
        credit_card_holder_id: i64,
        pad_source: &Option<String>,
    ) -> SqlResult<()> {
        if let Some(pad_source) = pad_source {
            sqlx::query_file!(
                "src/upsert/credit_card_account_holder/upsert_credit_card_pad_source.sql",
                credit_card_holder_id,
                pad_source
            )
            .execute(&mut *self.0)
            .await?;
        } else {
            sqlx::query_file!(
                "src/upsert/credit_card_account_holder/remove_credit_card_pad_source.sql",
                credit_card_holder_id
            )
            .execute(&mut *self.0)
            .await?;
        }

        Ok(())
    }

    async fn upsert_credit_card_account_helper(
        &mut self,
        account_key: &str,
        account_type: &str,
    ) -> Result<Option<i64>, Box<dyn std::error::Error>> {
        let result = sqlx::query_file!(
            "src/upsert/credit_card_account_holder/upsert_credit_card_account.sql",
            account_key,
            account_type,
        )
        .execute(&mut *self.0)
        .await;

        match result {
            Ok(result) => {
                // debt, fees, interest (debt), cashback, open balance, bonus
                const NUM_SUBTYPE_ACCOUNTS: u64 = 6;

                if result.rows_affected() != NUM_SUBTYPE_ACCOUNTS {
                    println!(
                        "{}: no row was affected, record: {}, {}",
                        "Warning".bold().yellow(),
                        account_key,
                        account_type
                    );

                    Ok(None)
                } else {
                    Ok(Some(result.last_insert_rowid()))
                }
            }
            Err(err) => Err(format!(
                "{}, record: {}, {}",
                err.to_string(),
                account_key,
                account_type
            )
            .into()),
        }
    }

    async fn upsert_credit_card_account_holder_helper(
        &mut self,
        record: &CreditCardHolder,
    ) -> Result<Option<i64>, Box<dyn std::error::Error>> {
        let result = sqlx::query_file!(
            "src/upsert/credit_card_account_holder/upsert_credit_card_account_holder.sql",
            record.person_key,
            record.account_type,
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

    async fn upsert_credit_card_account_mapping_helper(
        &mut self,
        credit_card_holder_id: i64,
        account_key: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let search_term = format!("{account_key}%");

        sqlx::query_file!(
            "src/upsert/credit_card_account_holder/upsert_credit_card_account_mapping.sql",
            credit_card_holder_id,
            search_term
        )
        .execute(&mut *self.0)
        .await?;

        Ok(())
    }
}
