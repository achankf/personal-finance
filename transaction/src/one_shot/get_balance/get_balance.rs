use std::{collections::HashMap, ops::Range};

use chrono::{DateTime, Local};
use common::all_time_until_now;
use futures_util::StreamExt;
use num_traits::Zero;

use crate::{BalanceRecord, MyBigDecimal, MyTransaction};

async fn project_data(
    transaction: &mut MyTransaction<'_>,
    source: &HashMap<Option<i64>, HashMap<i64, MyBigDecimal>>,
) -> Result<Vec<BalanceRecord>, Box<dyn std::error::Error>> {
    let full_names = transaction.get_full_names().await?;
    let account_names = transaction.get_account_names().await?;

    let mut ret = Vec::new();
    let empty_string = String::new();

    for (person_id, rest) in source {
        let full_name = person_id
            .map(|person_id| {
                full_names
                    .get(&person_id)
                    .expect(&format!("person id {person_id} to exist"))
            })
            .unwrap_or(&empty_string);
        for (account_id, balance) in rest {
            let account_name = account_names
                .get(&account_id)
                .expect(&format!("account id {account_id} to exist"));
            ret.push(BalanceRecord {
                name: full_name.clone(),
                account_name: account_name.clone(),
                balance: balance.clone(),
            });
        }
    }

    ret.retain_mut(|record| !record.balance.is_zero());
    ret.sort();

    Ok(ret)
}

impl MyTransaction<'_> {
    async fn get_balance(
        &mut self,
        kind: &str,
        range: Option<Range<DateTime<Local>>>,
    ) -> Result<Vec<BalanceRecord>, Box<dyn std::error::Error>> {
        let range = range.unwrap_or_else(all_time_until_now);

        let start_ts = range.start.timestamp();
        let end_ts = range.end.timestamp();

        let mut acc: HashMap<Option<i64>, HashMap<i64, MyBigDecimal>> = Default::default();

        {
            struct RawData {
                person_id: Option<i64>,
                account_id: i64,
                forex_rate: MyBigDecimal,
                unit: MyBigDecimal,
                debit: MyBigDecimal,
                credit: MyBigDecimal,
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
                    forex_rate,
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

                let balance = (forex_rate * unit * (debit - credit)).round2();

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

        let mut acc: HashMap<Option<i64>, HashMap<i64, MyBigDecimal>> = Default::default();

        {
            struct RawData {
                person_id: i64,
                account_id: i64,
                unit: MyBigDecimal,
                debit: MyBigDecimal,
                credit: MyBigDecimal,
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

                *balance += (unit.clone() * (debit.clone() - credit.clone())).round2();
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
