use bigdecimal::{BigDecimal, One, Zero};
use common::convert_to_bigdecimal;
use futures_util::TryStreamExt;
use std::{collections::HashMap, str::FromStr};

use crate::Transaction;

pub struct AssertTransactionBalance {
    pub transaction_id: i64,
    pub debit: BigDecimal,
    pub credit: BigDecimal,
    pub balance: BigDecimal,
}

pub struct CheckResult {
    pub per_entry_imbalances: Vec<AssertTransactionBalance>,
    pub total_debit: BigDecimal,
    pub total_credit: BigDecimal,
}

impl Transaction<'_> {
    pub async fn check_transaction_balance(
        &mut self,
    ) -> Result<CheckResult, Box<dyn std::error::Error>> {
        struct RawData {
            transaction_id: i64,
            forex_rate: Option<String>,
            unit: String,
            debit: Option<String>,
            credit: Option<String>,
        }

        let mut raw_results = sqlx::query_as!(
            RawData,
            r#"
SELECT
    transaction_id,
    COALESCE(forex_rate, '1') AS forex_rate,
    unit,
    debit,
    credit
FROM
    FinancialEntry
ORDER BY
    transaction_id
    "#
        )
        .fetch(&mut *self.0);

        #[derive(Default)]
        struct TempResult {
            debit: BigDecimal,
            credit: BigDecimal,
            balance: BigDecimal,
        }
        let mut temp_result: HashMap<i64, TempResult> = Default::default();
        let mut total_debit = BigDecimal::zero();
        let mut total_credit = BigDecimal::zero();

        while let Some(RawData {
            transaction_id,
            forex_rate,
            unit,
            debit,
            credit,
        }) = raw_results.try_next().await?
        {
            assert!(credit.is_some() ^ debit.is_some());

            let value = temp_result.entry(transaction_id).or_default();

            let unit = BigDecimal::from_str(&unit)?;
            let debit = convert_to_bigdecimal(&debit)?;
            let credit = convert_to_bigdecimal(&credit)?;

            let forex_rate = {
                if let Some(value) = forex_rate {
                    BigDecimal::from_str(&value)?
                } else {
                    BigDecimal::one()
                }
            };

            let debit = (unit.clone() * debit * forex_rate.clone())
                .with_scale_round(2, bigdecimal::RoundingMode::HalfUp);
            let credit = (unit.clone() * credit * forex_rate)
                .with_scale_round(2, bigdecimal::RoundingMode::HalfUp);

            value.debit += debit.clone();
            total_debit += debit.clone();
            value.credit += credit.clone();
            total_credit += credit.clone();
            value.balance += debit - credit;
        }

        Ok(CheckResult {
            per_entry_imbalances: temp_result
                .into_iter()
                .filter(|(_, value)| !bigdecimal::Zero::is_zero(&value.balance))
                .map(|(transaction_id, value)| AssertTransactionBalance {
                    transaction_id,
                    debit: value.debit,
                    credit: value.credit,
                    balance: value.balance,
                })
                .collect(),
            total_debit,
            total_credit,
        })
    }
}
