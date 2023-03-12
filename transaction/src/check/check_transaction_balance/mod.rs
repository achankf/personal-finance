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

pub struct PendingEntry {
    pub transaction_id: i64,
    pub debit: BigDecimal,
    pub credit: BigDecimal,
    pub balance: BigDecimal,
    pub note: String,
}

pub struct CheckResult {
    pub imbalances: Vec<AssertTransactionBalance>,
    pub pending_entries: Vec<PendingEntry>,
    pub total_debit: BigDecimal,
    pub total_credit: BigDecimal,
}

impl Transaction<'_> {
    pub async fn check_transaction_balance(
        &mut self,
    ) -> Result<CheckResult, Box<dyn std::error::Error>> {
        struct RawPendingEntry {
            transaction_id: i64,
            note: String,
        }

        let raw_pending_list: HashMap<i64, String> = {
            sqlx::query_file_as!(
                RawPendingEntry,
                "src/check/check_transaction_balance/get_pending_list.sql"
            )
            .fetch_all(&mut *self.0)
            .await?
            .into_iter()
            .map(
                |RawPendingEntry {
                     transaction_id,
                     note,
                 }| (transaction_id, note),
            )
            .collect()
        };

        struct RawData {
            transaction_id: i64,
            forex_rate: Option<String>,
            unit: String,
            debit: Option<String>,
            credit: Option<String>,
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
            debit: BigDecimal,
            credit: BigDecimal,
            balance: BigDecimal,
        }
        let mut temp_result: HashMap<i64, TempResult> = Default::default();
        let mut total_debit = BigDecimal::zero();
        let mut total_credit = BigDecimal::zero();

        // aggregate data with BigDecimals
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

                if !bigdecimal::Zero::is_zero(&value.balance) {
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
