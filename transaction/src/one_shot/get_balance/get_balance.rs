use std::{collections::HashMap, ops::Range, str::FromStr};

use bigdecimal::BigDecimal;
use chrono::{DateTime, Local};
use common::{all_time_until_now, convert_to_bigdecimal};
use futures_util::StreamExt;

use crate::{BalanceRecord, Transaction};

async fn project_data(
    transaction: &mut Transaction<'_>,
    source: &HashMap<Option<i64>, HashMap<i64, BigDecimal>>,
) -> Result<Vec<BalanceRecord>, Box<dyn std::error::Error>> {
    let full_names = transaction.get_full_names().await?;
    let account_names = transaction.get_account_names().await?;

    let mut ret = Vec::new();
    let empty_string = String::new();

    for (person_id, rest) in source {
        let full_name = person_id
            .map(|person_id| full_names.get(&person_id).unwrap())
            .unwrap_or(&empty_string);
        for (account_id, balance) in rest {
            let account_name = account_names.get(&account_id).unwrap();
            ret.push(BalanceRecord {
                name: full_name.clone(),
                account_name: account_name.clone(),
                balance: balance.clone(),
            });
        }
    }

    ret.retain_mut(|record| !bigdecimal::Zero::is_zero(&record.balance));
    ret.sort();

    Ok(ret)
}

impl Transaction<'_> {
    async fn get_balance(
        &mut self,
        kind: &str,
        range: Option<Range<DateTime<Local>>>,
    ) -> Result<Vec<BalanceRecord>, Box<dyn std::error::Error>> {
        let range = range.unwrap_or_else(all_time_until_now);

        let start_ts = range.start.timestamp();
        let end_ts = range.end.timestamp();

        let mut acc: HashMap<Option<i64>, HashMap<i64, BigDecimal>> = Default::default();

        {
            struct RawData {
                person_id: Option<i64>,
                account_id: i64,
                unit: String,
                debit: Option<String>,
                credit: Option<String>,
                is_debit_balance: bool,
            }

            let mut stream = sqlx::query_file_as!(
                RawData,
                "src/one_shot/get_balance/get_balance.sql",
                kind,
                start_ts,
                end_ts,
            )
            .fetch(&mut *self.0);

            while let Some(record) = stream.next().await {
                let RawData {
                    person_id,
                    account_id,
                    unit,
                    debit,
                    credit,
                    is_debit_balance,
                } = record?;

                let entry = acc
                    .entry(person_id)
                    .or_default()
                    .entry(account_id)
                    .or_default();

                let unit = BigDecimal::from_str(&unit)?;
                let debit = convert_to_bigdecimal(&debit)?;
                let credit = convert_to_bigdecimal(&credit)?;

                let balance =
                    (unit * (debit - credit)).with_scale_round(2, bigdecimal::RoundingMode::HalfUp);
                *entry += if is_debit_balance { balance } else { -balance };
            }
        }

        Ok(project_data(self, &acc).await?)
    }

    pub async fn get_revenue_balance(
        &mut self,
        range: Option<Range<DateTime<Local>>>,
    ) -> Result<Vec<BalanceRecord>, Box<dyn std::error::Error>> {
        self.get_balance("REVENUE", range).await
    }

    pub async fn get_expense_balance(
        &mut self,
        range: Option<Range<DateTime<Local>>>,
    ) -> Result<Vec<BalanceRecord>, Box<dyn std::error::Error>> {
        self.get_balance("EXPENSE", range).await
    }

    pub async fn get_cash_balance(
        &mut self,
        range: Option<Range<DateTime<Local>>>,
    ) -> Result<Vec<BalanceRecord>, Box<dyn std::error::Error>> {
        let range = range.unwrap_or_else(all_time_until_now);

        let start_ts = range.start.timestamp();
        let end_ts = range.end.timestamp();

        let mut acc: HashMap<Option<i64>, HashMap<i64, BigDecimal>> = Default::default();

        {
            struct RawData {
                person_id: i64,
                account_id: i64,
                unit: String,
                debit: Option<String>,
                credit: Option<String>,
            }

            let mut stream = sqlx::query_file_as!(
                RawData,
                "src/one_shot/get_balance/get_cash_balance.sql",
                start_ts,
                end_ts
            )
            .fetch(&mut *self.0);

            while let Some(record) = stream.next().await {
                let RawData {
                    person_id,
                    account_id,
                    unit,
                    debit,
                    credit,
                } = record?;

                let balance = acc
                    .entry(Some(person_id))
                    .or_default()
                    .entry(account_id)
                    .or_default();

                let unit = BigDecimal::from_str(&unit)?;
                let debit = convert_to_bigdecimal(&debit)?;
                let credit = convert_to_bigdecimal(&credit)?;

                *balance += (unit.clone() * (debit.clone() - credit.clone()))
                    .with_scale_round(2, bigdecimal::RoundingMode::HalfUp);
            }
        }

        Ok(project_data(self, &acc).await?)
    }

    pub async fn get_liabilities_balance(
        &mut self,
        range: Option<Range<DateTime<Local>>>,
    ) -> Result<Vec<BalanceRecord>, Box<dyn std::error::Error>> {
        self.get_balance("LIABILITIES", range).await
    }

    pub async fn get_equity_balance(
        &mut self,
        range: Option<Range<DateTime<Local>>>,
    ) -> Result<Vec<BalanceRecord>, Box<dyn std::error::Error>> {
        self.get_balance("EQUITY", range).await
    }
}
