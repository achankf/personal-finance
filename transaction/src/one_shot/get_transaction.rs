use chrono::{DateTime, Local};
use futures_util::StreamExt;

use crate::{MyBigDecimal, MyTransaction};

pub struct GetTransaction {
    pub item_id: i64,
    pub date: DateTime<Local>,
    pub account_key: String,
    pub account_kind: String,
    pub forex_rate: MyBigDecimal,
    pub unit: MyBigDecimal,
    pub debit: MyBigDecimal,
    pub credit: MyBigDecimal,
    pub balance: MyBigDecimal,
    pub description: String,
}

impl MyTransaction<'_> {
    pub async fn get_transaction(
        &mut self,
        transaction_id: &i64,
    ) -> Result<Vec<GetTransaction>, Box<dyn std::error::Error>> {
        pub struct GetTransactionRaw {
            pub item_id: i64,
            pub date: DateTime<Local>,
            pub account_key: String,
            pub account_kind: String,
            pub forex_rate: MyBigDecimal,
            pub unit: MyBigDecimal,
            pub debit: MyBigDecimal,
            pub credit: MyBigDecimal,
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

            let balance =
                (forex_rate.clone() * unit.clone() * (debit.clone() - credit.clone())).round2();

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
