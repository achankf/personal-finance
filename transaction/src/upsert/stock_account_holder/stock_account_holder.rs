use std::path::PathBuf;

use common::{deserialize_into_map, Id};
use owo_colors::OwoColorize;

use crate::Transaction;

use super::StockAccountHolder;

impl Id for StockAccountHolder {
    type IdType = (String, String, String);

    fn id(&self) -> Self::IdType {
        (
            self.person_key.clone(),
            self.account_type.clone(),
            self.ticker.clone(),
        )
    }
}

impl StockAccountHolder {
    pub fn create_account_key(&self) -> String {
        format!("{}-{}-{}", self.person_key, self.account_type, self.ticker)
    }
}

impl Transaction<'_> {
    pub async fn upsert_stock_account_holder(
        &mut self,
        csv_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("updating data with {}", csv_path.to_string_lossy());

        let parsed_records = deserialize_into_map::<StockAccountHolder>(csv_path)?;

        for (_, record) in &parsed_records {
            let account_key = record.create_account_key();

            println!("> inserting into Account {account_key}");
            self.upsert_stock_account_helper(record, &account_key)
                .await?;

            println!("> inserting into StockAccountHolder {account_key}");
            let id = self.upsert_stock_account_holder_helper(record).await?;

            if let Some(stock_account_holder_id) = id {
                println!("> inserting stock account mapping {account_key}");
                self.upsert_stock_account_mapping_helper(stock_account_holder_id, &account_key)
                    .await?
            }
        }

        Ok(())
    }

    async fn upsert_stock_account_helper(
        &mut self,
        record: &StockAccountHolder,
        account_key: &str,
    ) -> Result<Option<i64>, Box<dyn std::error::Error>> {
        let result = sqlx::query_file!(
            "src/upsert/stock_account_holder/upsert_stock_account.sql",
            account_key,
            record.account_type,
            record.ticker
        )
        .execute(&mut *self.0)
        .await;

        match result {
            Ok(result) => {
                // stock, distribution, commission, open balance, withholding tax, capital gain/loss/bookkeeping (equity)
                const NUM_SUBTYPE_ACCOUNTS: u64 = 9;

                if result.rows_affected() != NUM_SUBTYPE_ACCOUNTS {
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
            Err(err) => Err(format!("{}, record: {:?}", err.to_string(), record).into()),
        }
    }

    async fn upsert_stock_account_holder_helper(
        &mut self,
        record: &StockAccountHolder,
    ) -> Result<Option<i64>, Box<dyn std::error::Error>> {
        let result = sqlx::query_file!(
            "src/upsert/stock_account_holder/upsert_stock_account_holder.sql",
            record.person_key,
            record.account_type,
            record.ticker,
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
            Err(err) => Err(format!("{}, record: {:?}", err.to_string(), record).into()),
        }
    }

    async fn upsert_stock_account_mapping_helper(
        &mut self,
        stock_account_holder_id: i64,
        account_key: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let search_term = format!("{account_key}%");

        sqlx::query_file!(
            "src/upsert/stock_account_holder/upsert_stock_account_mapping.sql",
            stock_account_holder_id,
            search_term
        )
        .execute(&mut *self.0)
        .await?;

        Ok(())
    }
}
