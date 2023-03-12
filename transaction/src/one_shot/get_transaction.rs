use std::str::FromStr;

use bigdecimal::{BigDecimal, One};
use chrono::{DateTime, Local};
use futures_util::StreamExt;

use crate::Transaction;

pub struct GetTransaction {
    pub item_id: i64,
    pub date: DateTime<Local>,
    pub account_key: String,
    pub account_kind: String,
    pub forex_rate: Option<BigDecimal>,
    pub unit: BigDecimal,
    pub debit: Option<BigDecimal>,
    pub credit: Option<BigDecimal>,
    pub balance: BigDecimal,
    pub description: String,
}

impl Transaction<'_> {
    pub async fn get_transaction(
        &mut self,
        transaction_id: &i64,
    ) -> Result<Vec<GetTransaction>, Box<dyn std::error::Error>> {
        pub struct GetTransactionRaw {
            pub item_id: i64,
            pub date: DateTime<Local>,
            pub account_key: String,
            pub account_kind: String,
            pub forex_rate: Option<String>,
            pub unit: String,
            pub debit: Option<String>,
            pub credit: Option<String>,
            pub description: String,
        }

        let mut stream = sqlx::query_file_as!(
            GetTransactionRaw,
            "src/one_shot/get_transaction.sql",
            transaction_id
        )
        .fetch(&mut *self.0);

        let mut ret: Vec<_> = Default::default();

        while let Some(record) = stream.next().await {
            let GetTransactionRaw {
                item_id,
                date,
                account_key,
                account_kind,
                forex_rate,
                unit,
                debit,
                credit,
                description,
            } = record?;

            let forex_rate = forex_rate.map(|forex_rate| {
                BigDecimal::from_str(&forex_rate).expect("convert to BigDecimal")
            });
            let unit = BigDecimal::from_str(&unit).expect("convert to BigDecimal");
            let credit =
                credit.map(|credit| BigDecimal::from_str(&credit).expect("convert to BigDecimal"));
            let debit =
                debit.map(|debit| BigDecimal::from_str(&debit).expect("convert to BigDecimal"));
            let balance = (forex_rate.clone().unwrap_or(BigDecimal::one())
                * unit.clone()
                * (debit.clone().unwrap_or_default() - credit.clone().unwrap_or_default()))
            .with_scale_round(2, bigdecimal::RoundingMode::HalfUp);

            ret.push(GetTransaction {
                date,
                forex_rate,
                unit,
                debit,
                credit,
                item_id,
                account_key,
                account_kind,
                description,
                balance,
            })
        }

        Ok(ret)
    }
}
