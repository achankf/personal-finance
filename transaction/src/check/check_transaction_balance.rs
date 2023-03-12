use bigdecimal::BigDecimal;
use common::convert_to_bigdecimal;
use futures_util::TryStreamExt;
use std::{borrow::Borrow, collections::HashMap, str::FromStr};

use crate::Transaction;

pub struct AssertTransactionBalance {
    pub transaction_id: i64,
    pub debit: BigDecimal,
    pub credit: BigDecimal,
    pub balance: BigDecimal,
}

impl Transaction<'_> {
    pub async fn check_transaction_balance(
        &mut self,
    ) -> Result<Vec<AssertTransactionBalance>, Box<dyn std::error::Error>> {
        struct RawData {
            transaction_id: i64,
            unit: String,
            debit: Option<String>,
            credit: Option<String>,
        }

        let mut raw_results = sqlx::query_as!(
            RawData,
            r#"
SELECT
    transaction_id,
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

        let mut cur_transaction_id = -1;
        let mut cur_debit = BigDecimal::default();
        let mut cur_credit = BigDecimal::default();
        let mut cur_balance = BigDecimal::default();

        #[derive(Default)]
        struct TempResult {
            debit: BigDecimal,
            credit: BigDecimal,
            balance: BigDecimal,
        }
        let mut temp_result: HashMap<i64, TempResult> = Default::default();

        while let Some(RawData {
            transaction_id,
            unit,
            debit,
            credit,
        }) = raw_results.try_next().await?
        {
            let value = temp_result.entry(transaction_id).or_default();

            let unit = BigDecimal::from_str(&unit)?;
            let debit = convert_to_bigdecimal(&debit)?;
            let credit = convert_to_bigdecimal(&credit)?;

            let debit =
                (unit.clone() * debit).with_scale_round(2, bigdecimal::RoundingMode::HalfUp);
            let credit =
                (unit.clone() * credit).with_scale_round(2, bigdecimal::RoundingMode::HalfUp);
            value.debit += debit.clone();
            value.credit += credit.clone();
            value.balance += debit - credit;
        }

        Ok(temp_result
            .into_iter()
            .filter(|(_, value)| !bigdecimal::Zero::is_zero(&value.balance))
            .map(|(transaction_id, value)| AssertTransactionBalance {
                transaction_id,
                debit: value.debit,
                credit: value.credit,
                balance: value.balance,
            })
            .collect())
    }
}
