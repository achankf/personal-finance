use std::{collections::HashMap, str::FromStr};

use bigdecimal::BigDecimal;
use chrono::{Days, Local};
use common::convert_to_bigdecimal;
use futures_util::StreamExt;

use crate::Transaction;

pub struct ExpenseByCategoryResult {
    pub account_type: String,
    pub balance: BigDecimal,
}

impl Transaction<'_> {
    pub async fn get_expense_balance_by_category(
        &mut self,
        days_prior: u64,
    ) -> Result<Vec<ExpenseByCategoryResult>, Box<dyn std::error::Error>> {
        let acc = {
            let start_date = Local::now()
                .checked_sub_days(Days::new(days_prior))
                .expect("unable to subtract days")
                .timestamp();

            struct RawData {
                account_type_id: i64,
                unit: String,
                debit: Option<String>,
            }

            let mut stream = sqlx::query_as!(
                RawData,
                r#"
SELECT
    account_type_id,
    unit,
    debit
FROM
    FinancialEntry
    INNER JOIN Account USING (account_id)
    INNER JOIN AccountSubtype USING (account_subtype_id)
    INNER JOIN AccountType USING (account_type_id)
    INNER JOIN AccountKind USING (account_kind_id)
    LEFT JOIN OwnedAccount USING (account_id, account_type_id, account_subtype_id)
WHERE
    person_id IS NULL
    AND date > ?
    AND account_kind = 'EXPENSE'
"#,
                start_date
            )
            .fetch(&mut *self.0);

            let mut acc: HashMap<i64, BigDecimal> = Default::default();

            while let Some(data) = stream.next().await {
                let raw = data?;
                let unit = BigDecimal::from_str(&raw.unit)?;
                let debit = convert_to_bigdecimal(raw.debit)?;
                let entry = acc.entry(raw.account_type_id).or_default();

                *entry += (unit * debit).with_scale_round(2, bigdecimal::RoundingMode::HalfUp);
            }

            acc
        };

        let account_types = get_account_type(self).await?;

        Ok(acc
            .into_iter()
            .map(|(account_type_id, balance)| ExpenseByCategoryResult {
                account_type: account_types.get(&account_type_id).unwrap().clone(),
                balance,
            })
            .collect())
    }
}

async fn get_account_type(
    transaction: &mut Transaction<'_>,
) -> Result<HashMap<i64, String>, Box<dyn std::error::Error>> {
    struct RawData {
        account_type_id: i64,
        account_type: String,
    }

    Ok(sqlx::query_as!(
        RawData,
        r#"
SELECT
    account_type_id,
    account_type
FROM
    AccountType
"#,
    )
    .fetch_all(&mut *transaction.0)
    .await?
    .into_iter()
    .map(|record| (record.account_type_id, record.account_type))
    .collect())
}
