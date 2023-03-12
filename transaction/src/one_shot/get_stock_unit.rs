use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use bigdecimal::BigDecimal;
use chrono::{DateTime, Local};
use db::SqlResult;
use futures_util::StreamExt;

use crate::Transaction;

pub struct StockUnit {
    pub name: String,
    pub account_name: String,
    pub ticker: String,
    pub total_unit: BigDecimal,
    pub market_value: BigDecimal,
}

async fn get_tickers(
    transaction: &mut Transaction<'_>,
) -> SqlResult<HashMap<i64, (String, BigDecimal)>> {
    struct RawData {
        security_id: i64,
        ticker: String,
        price: String,
    }

    Ok(sqlx::query_as!(
        RawData,
        r#"
SELECT
    security_id,
    ticker,
    price
FROM
    SECURITY
"#
    )
    .fetch_all(&mut *transaction.0)
    .await?
    .into_iter()
    .map(|raw| {
        (
            raw.security_id,
            (raw.ticker, BigDecimal::from_str(&raw.price).unwrap()),
        )
    })
    .collect())
}

async fn get_account_names(transaction: &mut Transaction<'_>) -> SqlResult<HashMap<i64, String>> {
    struct RawData {
        account_type_id: i64,
        account_name: String,
    }

    Ok(sqlx::query_as!(
        RawData,
        r#"
SELECT
    account_type_id,
    account_name
FROM
    AccountType
    INNER JOIN StockAccount USING (account_type_id)
    INNER JOIN CashAccountProduct USING (account_type_id)
"#
    )
    .fetch_all(&mut *transaction.0)
    .await?
    .into_iter()
    .map(|raw| (raw.account_type_id, raw.account_name))
    .collect())
}

impl Transaction<'_> {
    pub async fn get_stock_unit(&mut self, date: &DateTime<Local>) -> SqlResult<Vec<StockUnit>> {
        let timestamp = date.timestamp();

        pub struct RawData {
            pub person_id: i64,
            pub account_type_id: i64,
            pub security_id: i64,
            pub unit: String,
        }

        let mut acc: HashMap<i64, HashMap<i64, HashMap<i64, BigDecimal>>> = Default::default();

        {
            let mut stream = sqlx::query_as!(
                RawData,
                r#"
SELECT
    person_id,
    account_type_id,
    security_id,
    CASE
        WHEN DEBIT IS NOT NULL THEN unit
        ELSE - unit
    END AS unit
FROM
    FinancialEntry
    INNER JOIN StockAccountEntry USING (account_id)
    INNER JOIN StockAccountHolder USING (stock_account_holder_id)
    INNER JOIN AccountSubtype USING (account_subtype_id)
    INNER JOIN AccountType USING (account_type_id)
WHERE
    account_subtype = 'STOCK'
    AND date < ?;
"#,
                timestamp
            )
            .fetch(&mut *self.0);

            while let Some(result) = stream.next().await {
                let raw = result?;

                let unit = BigDecimal::from_str(&raw.unit).expect("convert to BigDecimal");

                let entry = acc
                    .entry(raw.person_id)
                    .or_default()
                    .entry(raw.account_type_id)
                    .or_default()
                    .entry(raw.security_id)
                    .or_default();
                *entry += unit;
            }
        }

        let mut ret: BTreeMap<(String, String, String), (BigDecimal, BigDecimal)> =
            Default::default();

        let full_names = self.get_full_names().await?;
        let account_names = get_account_names(self).await?;
        let tickers = get_tickers(self).await?;

        for (person_id, rest) in acc {
            let name = full_names.get(&person_id).unwrap();
            for (account_type_id, rest) in rest {
                let account_name = account_names.get(&account_type_id).unwrap();
                for (security_id, total_unit) in rest {
                    let (ticker, price) = tickers.get(&security_id).unwrap();

                    let entry = ret
                        .entry((name.clone(), account_name.clone(), ticker.clone()))
                        .or_default();
                    entry.0 += total_unit.clone();
                    entry.1 +=
                        (total_unit * price).with_scale_round(2, bigdecimal::RoundingMode::HalfUp);
                }
            }
        }

        Ok(ret
            .into_iter()
            .filter(|(_, (total_unit, _))| total_unit > &bigdecimal::Zero::zero())
            .map(
                |((name, account_name, ticker), (total_unit, market_value))| StockUnit {
                    name,
                    account_name,
                    ticker,
                    total_unit,
                    market_value,
                },
            )
            .collect())
    }
}
