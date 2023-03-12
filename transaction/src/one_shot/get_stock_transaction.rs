use bigdecimal::BigDecimal;
use chrono::{DateTime, Local};
use futures::StreamExt;
use std::{cmp::Reverse, str::FromStr};

use crate::Transaction;

pub struct StockTransaction {
    pub name: String,
    pub account_type: String,
    pub ticker: String,
    pub date: DateTime<Local>,
    pub unit: BigDecimal,
    pub debit: Option<BigDecimal>,
    pub credit: Option<BigDecimal>,
}

impl Transaction<'_> {
    pub async fn get_stock_transaction(
        &mut self,
        ticker: &str,
        limit: u32,
    ) -> Result<Vec<StockTransaction>, Box<dyn std::error::Error>> {
        struct StockTransactionRaw {
            pub name: String,
            pub account_type: String,
            pub ticker: String,
            pub date: DateTime<Local>,
            pub unit: String,
            pub debit: Option<String>,
            pub credit: Option<String>,
        }

        let mut stream = sqlx::query_file_as!(
            StockTransactionRaw,
            "src/one_shot/get_stock_transaction.sql",
            ticker,
            limit
        )
        .fetch(&mut *self.0);

        let mut ret: Vec<_> = Default::default();

        while let Some(record) = stream.next().await {
            let StockTransactionRaw {
                name,
                account_type,
                ticker,
                date,
                unit,
                debit,
                credit,
            } = record?;

            ret.push(StockTransaction {
                name,
                account_type,
                ticker,
                date,
                unit: BigDecimal::from_str(&unit).expect("convert to BigDecimal"),
                debit: debit
                    .map(|debit| BigDecimal::from_str(&debit).expect("convert to BigDecimal")),
                credit: credit
                    .map(|credit| BigDecimal::from_str(&credit).expect("convert to BigDecimal")),
            })
        }

        ret.sort_by_key(|record| Reverse(record.date));

        Ok(ret)
    }
}
