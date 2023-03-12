use futures_util::TryStreamExt;
use num_traits::Zero;
use std::collections::HashMap;

use crate::{MyBigDecimal, MyTransaction};

pub struct AssertTransactionBalance {
    pub transaction_id: i64,
    pub debit: MyBigDecimal,
    pub credit: MyBigDecimal,
    pub balance: MyBigDecimal,
}

pub struct PendingEntry {
    pub transaction_id: i64,
    pub debit: MyBigDecimal,
    pub credit: MyBigDecimal,
    pub balance: MyBigDecimal,
    pub note: String,
}

pub struct CheckResult {
    pub imbalances: Vec<AssertTransactionBalance>,
    pub pending_entries: Vec<PendingEntry>,
    pub total_debit: MyBigDecimal,
    pub total_credit: MyBigDecimal,
}

impl MyTransaction<'_> {
    pub async fn check_transaction_balance(
        &mut self,
    ) -> Result<CheckResult, Box<dyn std::error::Error>> {
        struct RawPendingEntry {
            transaction_id: i64,
            note: String,
        }

        let raw_pending_list: HashMap<_, _> = sqlx::query_file_as!(
            RawPendingEntry,
            "src/check/check_transaction_balance/get_pending_list.sql"
        )
        .map(
            |RawPendingEntry {
                 transaction_id,
                 note,
             }| (transaction_id, note),
        )
        .fetch_all(&mut *self.0)
        .await?
        .into_iter()
        .collect();

        #[derive(sqlx::FromRow)]
        struct RawData {
            transaction_id: i64,
            forex_rate: MyBigDecimal,
            unit: MyBigDecimal,
            debit: MyBigDecimal,
            credit: MyBigDecimal,
        }

        let mut raw_results = {
            sqlx::query_file_as!(
                RawData,
                "src/check/check_transaction_balance/get_entries.sql"
            )
            .fetch(&mut *self.0)
        };

        #[derive(Default)]
        struct TempResult {
            debit: MyBigDecimal,
            credit: MyBigDecimal,
            balance: MyBigDecimal,
        }
        let mut temp_result: HashMap<i64, TempResult> = Default::default();
        let mut total_debit = MyBigDecimal::zero();
        let mut total_credit = MyBigDecimal::zero();

        // aggregate data with MyBigDecimals
        while let Some(RawData {
            transaction_id,
            forex_rate,
            unit,
            debit,
            credit,
        }) = raw_results.try_next().await?
        {
            // assert!(credit.is_some() ^ debit.is_some());

            let value = temp_result.entry(transaction_id).or_default();

            let debit = (unit.clone() * debit * forex_rate.clone()).round2();
            let credit = (unit.clone() * credit * forex_rate).round2();

            value.debit += debit.clone();
            total_debit += debit.clone();
            value.credit += credit.clone();
            total_credit += credit.clone();
            value.balance += debit - credit;
        }

        // extract results
        let (imbalances, pending_entries) = {
            let mut imbalances = Vec::default();
            let mut pending_entries = Vec::default();

            for (transaction_id, value) in temp_result {
                if let Some(note) = raw_pending_list.get(&transaction_id) {
                    pending_entries.push(PendingEntry {
                        transaction_id,
                        debit: value.debit,
                        credit: value.credit,
                        balance: value.balance,
                        note: note.to_string(),
                    });

                    continue;
                }

                if !value.balance.is_zero() {
                    imbalances.push(AssertTransactionBalance {
                        transaction_id,
                        debit: value.debit,
                        credit: value.credit,
                        balance: value.balance,
                    });
                }
            }

            (imbalances, pending_entries)
        };

        Ok(CheckResult {
            imbalances,
            pending_entries,
            total_debit,
            total_credit,
        })
    }
}
