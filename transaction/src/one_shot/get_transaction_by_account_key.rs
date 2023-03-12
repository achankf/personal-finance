use std::{collections::VecDeque, ops::Range, str::FromStr};

use bigdecimal::{BigDecimal, One};
use chrono::{DateTime, Local};

use serde::Deserialize;

use crate::Transaction;

#[derive(Deserialize, Debug)]
pub struct TransactionByAccountKey {
    pub transaction_id: Option<i64>,
    pub item_id: Option<i64>,
    pub date: DateTime<Local>,
    pub unit: BigDecimal,
    pub debit: Option<BigDecimal>,
    pub credit: Option<BigDecimal>,
    pub forex_rate: Option<BigDecimal>,
    pub total_amount: BigDecimal,
    pub description: String,
}

impl Transaction<'_> {
    pub async fn get_transaction_by_account_key(
        &mut self,
        account_key: &str,
        range: Range<DateTime<Local>>,
    ) -> Result<Vec<TransactionByAccountKey>, Box<dyn std::error::Error>> {
        let end_ts = range.end.timestamp();

        struct RawData {
            transaction_id: i64,
            item_id: i64,
            date: DateTime<Local>,
            unit: String,
            debit: Option<String>,
            credit: Option<String>,
            forex_rate: Option<String>,
            description: String,
            is_debit_balance: bool,
        }

        let result = sqlx::query_as!(
            RawData,
            r#"
SELECT
    transaction_id,
    item_id,
    date AS "date!:DateTime<Local>",
    unit,
    debit,
    credit,
    forex_rate,
    description,
    account_kind IN ('ASSET', 'EXPENSE') AS "is_debit_balance!:bool"
FROM
    FinancialEntry
    INNER JOIN Account USING (account_id)
    INNER JOIN AccountSubtype USING(account_subtype_id)
    INNER JOIN AccountKind USING (account_kind_id)
WHERE
    account_key = ?
    AND date < ?
ORDER BY
    "date!:DateTime<Local>"
"#,
            account_key,
            end_ts
        )
        .fetch_all(&mut *self.0)
        .await?;

        let mut ret = VecDeque::<TransactionByAccountKey>::with_capacity(result.len());

        let mut open_balance = TransactionByAccountKey {
            transaction_id: None,
            item_id: None,
            date: range.start.clone(),
            unit: 1.into(),
            debit: Some(bigdecimal::Zero::zero()),
            credit: Some(bigdecimal::Zero::zero()),
            forex_rate: None,
            total_amount: bigdecimal::Zero::zero(),
            description: "Open Balance".to_string(),
        };

        for record in result.into_iter() {
            let unit = BigDecimal::from_str(&record.unit).expect("unit to be BigDecimal");
            let debit = record
                .debit
                .map(|debit| BigDecimal::from_str(&debit).expect("to parse debit"));
            let credit = record
                .credit
                .map(|credit| BigDecimal::from_str(&credit).expect("to parse credit"));
            let forex_rate = record.forex_rate.map(|forex_rate| {
                BigDecimal::from_str(&forex_rate).expect("to parse exchange rate")
            });
            let total_amount = unit.clone()
                * if record.is_debit_balance {
                    debit.clone().unwrap_or_default() - credit.clone().unwrap_or_default()
                } else {
                    credit.clone().unwrap_or_default() - debit.clone().unwrap_or_default()
                }
                * forex_rate.clone().unwrap_or(One::one());

            let total_amount = total_amount.with_scale_round(2, bigdecimal::RoundingMode::HalfUp);

            // note: records are sorted
            if record.date < range.start {
                open_balance.debit = open_balance
                    .debit
                    .and_then(|prev_debit| Some(prev_debit + debit.clone().unwrap_or_default()));
                open_balance.credit = open_balance
                    .credit
                    .and_then(|prev_credit| Some(prev_credit + credit.clone().unwrap_or_default()));
                open_balance.total_amount += total_amount;
            } else {
                ret.push_back(TransactionByAccountKey {
                    transaction_id: Some(record.transaction_id),
                    item_id: Some(record.item_id),
                    date: record.date,
                    unit: unit.clone(),
                    debit,
                    credit,
                    forex_rate,
                    total_amount,
                    description: record.description,
                })
            }
        }

        ret.push_front(open_balance);

        Ok(ret.into())
    }
}
