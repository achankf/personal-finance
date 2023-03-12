use std::collections::HashMap;

use chrono::{Days, Local};
use futures_util::StreamExt;

use crate::{MyBigDecimal, MyTransaction};

pub struct ExpenseByCategoryResult {
    pub account_type: String,
    pub balance: MyBigDecimal,
}

impl MyTransaction<'_> {
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
                unit: MyBigDecimal,
                debit: MyBigDecimal,
            }

            let mut stream = sqlx::query_file_as!(
                RawData,
                "src/one_shot/get_expense_by_category.sql",
                start_date
            )
            .fetch(&mut *self.0);

            let mut acc: HashMap<i64, MyBigDecimal> = Default::default();

            while let Some(data) = stream.next().await {
                let RawData {
                    unit,
                    debit,
                    account_type_id,
                } = data?;
                let entry = acc.entry(account_type_id).or_default();

                *entry += (unit * debit).round2();
            }

            acc
        };

        let account_types = self.get_account_types().await?;

        Ok(acc
            .into_iter()
            .map(|(account_type_id, balance)| ExpenseByCategoryResult {
                account_type: account_types
                    .get(&account_type_id)
                    .expect("account type to exist")
                    .clone(),
                balance,
            })
            .collect())
    }
}
