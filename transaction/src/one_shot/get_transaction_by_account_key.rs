use chrono::{DateTime, Local};
use num_traits::{One, Zero};
use serde::Deserialize;
use std::{collections::VecDeque, ops::Range};

use crate::{MyBigDecimal, MyTransaction};

#[derive(Deserialize, Debug)]
pub struct TransactionByAccountKey {
    pub transaction_id: Option<i64>,
    pub item_id: Option<i64>,
    pub date: DateTime<Local>,
    pub unit: MyBigDecimal,
    pub debit: MyBigDecimal,
    pub credit: MyBigDecimal,
    pub forex_rate: MyBigDecimal,
    pub balance: MyBigDecimal,
    pub description: String,
}

pub struct TransactionByAccountKeySummary(Vec<TransactionByAccountKey>);

impl TransactionByAccountKeySummary {
    /// Sum up the total (debit, credit, balance)
    pub fn summarize_total(&self) -> (MyBigDecimal, MyBigDecimal, MyBigDecimal) {
        let (total_debit, total_credit): (MyBigDecimal, MyBigDecimal) =
            self.0
                .iter()
                .fold((Zero::zero(), Zero::zero()), |(debit, credit), record| {
                    let unit = record.unit.clone();
                    let debit = debit + unit.clone() * record.debit.clone();
                    let credit = credit + unit * record.credit.clone();
                    (debit, credit)
                });

        let total_balance = total_debit.clone() - total_credit.clone();

        (total_debit, total_credit, total_balance)
    }
}

impl IntoIterator for TransactionByAccountKeySummary {
    type Item = TransactionByAccountKey;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl MyTransaction<'_> {
    pub async fn get_transaction_by_account_key(
        &mut self,
        account_key: &str,
        range: Range<DateTime<Local>>,
    ) -> Result<TransactionByAccountKeySummary, Box<dyn std::error::Error>> {
        let end_ts = range.end.timestamp();

        struct RawData {
            transaction_id: i64,
            item_id: i64,
            date: DateTime<Local>,
            unit: MyBigDecimal,
            debit: MyBigDecimal,
            credit: MyBigDecimal,
            forex_rate: MyBigDecimal,
            description: String,
            sign_multiplier: MyBigDecimal,
        }

        let result = sqlx::query_file_as!(
            RawData,
            "src/one_shot/get_transaction_by_account_key.sql",
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
            debit: Zero::zero(),
            credit: Zero::zero(),
            forex_rate: One::one(),
            balance: Zero::zero(),
            description: "Open Balance".to_string(),
        };

        for record in result.into_iter() {
            let RawData {
                unit,
                debit,
                credit,
                forex_rate,
                ..
            } = record;

            let balance = unit.clone()
                * (debit.clone() - credit.clone())
                * record.sign_multiplier.clone()
                * forex_rate.clone();
            let balance = balance.round2();

            if record.date < range.start {
                // note: records are sorted in ascending order, so this block will trigger in early entries up to the cutoff point (range.start)
                open_balance.debit = open_balance.debit + debit.clone();
                open_balance.credit = open_balance.credit + credit.clone();
                open_balance.balance += balance;
            } else {
                ret.push_back(TransactionByAccountKey {
                    transaction_id: Some(record.transaction_id),
                    item_id: Some(record.item_id),
                    date: record.date,
                    unit: unit.clone(),
                    debit,
                    credit,
                    forex_rate,
                    balance,
                    description: record.description,
                })
            }
        }

        ret.push_front(open_balance);

        Ok(TransactionByAccountKeySummary(ret.into()))
    }
}
