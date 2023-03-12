use std::ops::Range;

use chrono::{DateTime, Datelike, Days, Local, TimeZone};
use db::SqlResult;
use serde::Deserialize;
use sqlx::Row;

use crate::Transaction;

#[derive(Deserialize, Debug)]
pub struct TransactionByAccountKey {
    pub transaction_id: Option<i64>,
    pub item_id: Option<i64>,
    pub date: DateTime<Local>,
    pub unit: f64,
    pub debit: Option<f64>,
    pub credit: Option<f64>,
    pub exchange_rate: Option<f64>,
    pub total_amount: f64,
    pub description: String,
}

impl Transaction<'_> {
    pub async fn get_transaction_by_account_key(
        &mut self,
        account_key: &str,
        range: Range<DateTime<Local>>,
    ) -> SqlResult<Vec<TransactionByAccountKey>> {
        let start_ts = range.start.timestamp();
        let end_ts = range.end.timestamp();

        let open_balance_end_date = range
            .start
            .checked_sub_days(Days::new(1))
            .expect("unable to subtract days");
        let open_balance_end_date = Local
            .with_ymd_and_hms(
                open_balance_end_date.year(),
                open_balance_end_date.month(),
                open_balance_end_date.day(),
                23,
                59,
                59,
            )
            .single()
            .expect("cannot get the start of today");

        let result = sqlx::query(
            r#"
WITH Data AS (
    SELECT
        transaction_id,
        item_id,
        date,
        unit,
        debit,
        credit,
        exchange_rate,
        ROUND(
            ROUND(
                (
                    CASE
                        WHEN account_kind IN ('ASSET', 'EXPENSE') THEN COALESCE(debit, 0) - COALESCE(credit, 0)
                        ELSE COALESCE(credit, 0) - COALESCE(debit, 0)
                    END
                ) * unit,
                2
            ) * COALESCE(exchange_rate, 1.0),
            2
        ) AS total_amount,
        description,
        account_subtype,
        account_kind
    FROM
        FinancialEntry
        INNER JOIN Account USING (account_id)
        INNER JOIN AccountSubtype USING(account_subtype_id)
        INNER JOIN AccountKind USING(account_kind_id)
        LEFT JOIN TransactionForex USING (transaction_id)
    WHERE
        account_key = ?
)
SELECT
    -- append an opening balance for assets and liabilities, to make the balance for trailing X days more meaningful
    NULL,
    NULL,
    ?,
    1.0,
    -- to avoid too many digits from accumulated data, I place the total balace for either credit or debit
    -- also only one of credit or debit column must exist for each row
    CASE
        WHEN SUM(total_amount) > 0 AND account_kind = 'ASSET' THEN SUM(total_amount)
        WHEN SUM(total_amount) < 0 AND account_kind = 'LIABILITIES' THEN SUM(total_amount)
        ELSE NULL
    END,
    CASE
        WHEN SUM(total_amount) < 0 AND account_kind = 'ASSET' THEN SUM(total_amount)
        WHEN SUM(total_amount) > 0 AND account_kind = 'LIABILITIES' THEN SUM(total_amount)
        ELSE NULL
    END,
    NULL,
    SUM (total_amount),
    'OPEN BALANCE'
FROM
    Data
WHERE
    account_kind IN ('ASSET', 'LIABILITIES')
    AND account_subtype <> 'STOCK'
    AND date < ?
UNION
ALL
SELECT
    transaction_id,
    item_id,
    date,
    unit,
    debit,
    credit,
    exchange_rate,
    total_amount,
    description
FROM
    Data
WHERE
    date BETWEEN ?
    AND ?
"#)
        .bind(account_key)
        .bind(open_balance_end_date)
        .bind(start_ts)
        .bind(start_ts)
        .bind(end_ts)
        .fetch_all(&mut *self.0)
        .await?.into_iter().map(|record| TransactionByAccountKey {
            transaction_id: record.get(0),
            item_id: record.get(1),
            date: record.get(2),
            unit: record.get(3),
            debit: record.get(4),
            credit: record.get(5),
            exchange_rate: record.get(6),
            total_amount: record.get(7),
            description: record.get(8),
        }).collect();

        Ok(result)
    }
}
