use bigdecimal::BigDecimal;
use common::convert_to_bigdecimal;
use futures_util::TryStreamExt;
use std::str::FromStr;

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

        let mut results = Vec::new();

        let mut cur_transaction_id = -1;
        let mut cur_debit = BigDecimal::default();
        let mut cur_credit = BigDecimal::default();
        let mut cur_balance = BigDecimal::default();

        while let Some(RawData {
            transaction_id,
            unit,
            debit,
            credit,
        }) = raw_results.try_next().await?
        {
            if transaction_id != cur_transaction_id {
                if !bigdecimal::Zero::is_zero(&cur_balance) {
                    results.push(AssertTransactionBalance {
                        transaction_id: cur_transaction_id,
                        debit: cur_debit,
                        credit: cur_credit,
                        balance: cur_balance,
                    });
                }

                cur_transaction_id = transaction_id;
                cur_debit = Default::default();
                cur_credit = Default::default();
                cur_balance = Default::default();
            }

            let unit = BigDecimal::from_str(&unit)?;
            let debit = convert_to_bigdecimal(debit)?;
            let credit = convert_to_bigdecimal(credit)?;

            let debit =
                (unit.clone() * debit).with_scale_round(2, bigdecimal::RoundingMode::HalfUp);
            let credit =
                (unit.clone() * credit).with_scale_round(2, bigdecimal::RoundingMode::HalfUp);
            cur_debit += debit.clone();
            cur_credit += credit.clone();
            cur_balance += debit - credit;
        }

        Ok(results)
    }
}
