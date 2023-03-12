use std::{collections::HashMap, str::FromStr};

use bigdecimal::BigDecimal;
use chrono::{Days, Local};
use common::convert_to_bigdecimal;
use futures_util::StreamExt;

use crate::Transaction;

pub struct ExpenseByCategoryResult {
    pub account_type: String,
    pub balance: BigDecimal,
}

impl Transaction<'_> {
    pub async fn get_expense_balance_by_category(
        &mut self,
        days_prior: u64,
    ) -> Result<Vec<ExpenseByCategoryResult>, Box<dyn std::error::Error>> {
        let acc = {
            let start_date = Local::now()
                .checked_sub_days(Days::new(days_prior))
                .expect("unable to subtract days")
                .timestamp();

            struct RawData {
                account_type_id: i64,
                unit: String,
                debit: Option<String>,
            }

            let mut stream = sqlx::query_file_as!(
                RawData,
                "src/one_shot/get_expense_by_category.sql",
                start_date
            )
            .fetch(&mut *self.0);

            let mut acc: HashMap<i64, BigDecimal> = Default::default();

            while let Some(data) = stream.next().await {
                let raw = data?;
                let unit = BigDecimal::from_str(&raw.unit)?;
                let debit = convert_to_bigdecimal(&raw.debit)?;
                let entry = acc.entry(raw.account_type_id).or_default();

                *entry += (unit * debit).with_scale_round(2, bigdecimal::RoundingMode::HalfUp);
            }

            acc
        };

        let account_types = self.get_account_types().await?;

        Ok(acc
            .into_iter()
            .map(|(account_type_id, balance)| ExpenseByCategoryResult {
                account_type: account_types.get(&account_type_id).unwrap().clone(),
                balance,
            })
            .collect())
    }
}
