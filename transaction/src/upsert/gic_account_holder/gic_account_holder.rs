use std::path::PathBuf;

use chrono::NaiveDate;
use common::{deserialize_into_map, Id};
use owo_colors::OwoColorize;

use crate::{MyBigDecimal, MyTransaction};

use super::GicAccountHolder;

impl Id for GicAccountHolder {
    type IdType = (String, String, NaiveDate, NaiveDate, String);

    fn id(&self) -> Self::IdType {
        (
            self.person_key.clone(),
            self.account_type.clone(),
            self.issue_date.clone(),
            self.maturity_date.clone(),
            self.apr.to_string(),
        )
    }
}

impl GicAccountHolder {
    pub fn get_account_key(&self) -> String {
        format!(
            "{}-{}-{}-{}-{}",
            self.person_key,
            self.account_type,
            self.issue_date,
            self.maturity_date,
            self.get_apr_percent()
        )
    }

    pub fn get_apr_percent(&self) -> String {
        let apr = MyBigDecimal::from(self.apr.clone());
        format!("{}%", (apr * 100).round2())
    }
}

impl MyTransaction<'_> {
    pub async fn upsert_gic_account_holder(
        &mut self,
        csv_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("updating data with {}", csv_path.to_string_lossy());

        let parsed_records = deserialize_into_map::<GicAccountHolder>(csv_path)?;

        for (_, record) in &parsed_records {
            let account_key = record.get_account_key();

            println!("> inserting GIC account {}", account_key.yellow());
            self.upsert_gic_account_helper(&account_key, &record)
                .await?;

            println!("> inserting GIC account holder {}", account_key.yellow());
            let id = self.upsert_gic_account_holder_helper(record).await?;

            if let Some(gic_account_holder_id) = id {
                println!("> inserting GIC account mapping {}", account_key.yellow());
                self.upsert_gic_account_mapping_helper(gic_account_holder_id, &account_key)
                    .await?
            }
        }

        Ok(())
    }

    async fn upsert_gic_account_helper(
        &mut self,
        account_key: &str,
        record: &GicAccountHolder,
    ) -> Result<Option<i64>, Box<dyn std::error::Error>> {
        let apr = record.get_apr_percent();

        let result = sqlx::query_file!(
            "src/upsert/gic_account_holder/upsert_gic_account.sql",
            account_key,
            record.account_type,
            apr
        )
        .execute(&mut *self.0)
        .await;

        match result {
            Ok(result) => {
                // principal, interest, open balance
                const NUM_SUBTYPE_ACCOUNTS: u64 = 3;

                if result.rows_affected() != NUM_SUBTYPE_ACCOUNTS {
                    println!(
                        "{}: no row was affected, record: {}, {}",
                        "Warning".bold().yellow(),
                        account_key,
                        record.account_type
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
                record.account_type
            )
            .into()),
        }
    }

    async fn upsert_gic_account_holder_helper(
        &mut self,
        record: &GicAccountHolder,
    ) -> Result<Option<i64>, Box<dyn std::error::Error>> {
        let result = sqlx::query_file!(
            "src/upsert/gic_account_holder/upsert_gic_account_holder.sql",
            record.person_key,
            record.account_type,
            record.issue_date,
            record.maturity_date,
            record.apr
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

    async fn upsert_gic_account_mapping_helper(
        &mut self,
        gic_account_holder_id: i64,
        account_key: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let search_term = format!("{account_key}%");

        sqlx::query_file!(
            "src/upsert/gic_account_holder/upsert_gic_account_mapping.sql",
            gic_account_holder_id,
            search_term
        )
        .execute(&mut *self.0)
        .await?;

        Ok(())
    }
}
