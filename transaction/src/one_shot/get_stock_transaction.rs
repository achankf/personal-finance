use bigdecimal::BigDecimal;
use chrono::{DateTime, Local};
use std::{cmp::Reverse, str::FromStr};

use crate::Transaction;

pub struct StockTransaction {
    pub name: String,
    pub account_type: String,
    pub ticker: String,
    pub date: DateTime<Local>,
    pub unit: BigDecimal,
    pub debit: Option<BigDecimal>,
    pub credit: Option<BigDecimal>,
}

struct StockTransactionRaw {
    pub name: String,
    pub account_type: String,
    pub ticker: String,
    pub date: DateTime<Local>,
    pub unit: String,
    pub debit: Option<String>,
    pub credit: Option<String>,
}

impl Transaction<'_> {
    pub async fn get_stock_transaction(
        &mut self,
        ticker: &str,
        limit: u32,
    ) -> Result<Vec<StockTransaction>, Box<dyn std::error::Error>> {
        let raw_results = sqlx::query_as!(
            StockTransactionRaw,
            r#"
SELECT
    first_name || ' ' || last_name AS name,
    account_type,
    ticker,
    date AS "date:DateTime<Local>",
    CASE
        WHEN debit THEN unit
        ELSE - unit
    END AS unit,
    debit,
    credit
FROM
    FinancialEntry
    INNER JOIN StockAccountEntry USING (account_id)
    INNER JOIN StockAccountHolder USING (stock_account_holder_id)
    INNER JOIN AccountSubtype USING (account_subtype_id)
    INNER JOIN SECURITY USING (security_id)
    INNER JOIN AccountType USING (account_type_id)
    INNER JOIN Person USING (person_id)
WHERE
    account_subtype = 'STOCK'
    AND ticker = ?
LIMIT
    ?
"#,
            ticker,
            limit
        )
        .fetch_all(&mut *self.0)
        .await?;

        let mut ret: Vec<_> = raw_results
            .into_iter()
            .map(|record| StockTransaction {
                name: record.name,
                account_type: record.account_type,
                ticker: record.ticker,
                date: record.date,
                unit: BigDecimal::from_str(&record.unit).expect("convert to BigDecimal"),
                debit: record
                    .debit
                    .map(|debit| BigDecimal::from_str(&debit).expect("convert to BigDecimal")),
                credit: record
                    .credit
                    .map(|credit| BigDecimal::from_str(&credit).expect("convert to BigDecimal")),
            })
            .collect();

        ret.sort_by_key(|record| Reverse(record.date));

        Ok(ret)
    }
}
