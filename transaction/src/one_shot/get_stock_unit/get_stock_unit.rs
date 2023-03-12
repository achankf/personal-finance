use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Local};
use db::SqlResult;
use futures_util::StreamExt;
use num_traits::sign::Signed;

use crate::{MyBigDecimal, MyTransaction, StockUnit};

async fn get_tickers(
    transaction: &mut MyTransaction<'_>,
) -> SqlResult<HashMap<i64, (String, MyBigDecimal)>> {
    struct RawData {
        security_id: i64,
        ticker: String,
        price: MyBigDecimal,
    }

    Ok(
        sqlx::query_file_as!(RawData, "src/one_shot/get_stock_unit/get_tickers.sql")
            .fetch_all(&mut *transaction.0)
            .await?
            .into_iter()
            .map(|raw| (raw.security_id, (raw.ticker, raw.price)))
            .collect(),
    )
}

async fn get_account_names(transaction: &mut MyTransaction<'_>) -> SqlResult<HashMap<i64, String>> {
    struct RawData {
        account_type_id: i64,
        account_name: String,
    }

    Ok(
        sqlx::query_file_as!(RawData, "src/one_shot/get_stock_unit/get_account_names.sql")
            .fetch_all(&mut *transaction.0)
            .await?
            .into_iter()
            .map(|raw| (raw.account_type_id, raw.account_name))
            .collect(),
    )
}

impl MyTransaction<'_> {
    pub async fn get_stock_unit(&mut self, date: &DateTime<Local>) -> SqlResult<Vec<StockUnit>> {
        let timestamp = date.timestamp();

        struct RawData {
            pub person_id: i64,
            pub account_type_id: i64,
            pub security_id: i64,
            pub unit: MyBigDecimal,
            pub sign_multiplier: i32,
        }

        let mut acc: HashMap<i64, HashMap<i64, HashMap<i64, MyBigDecimal>>> = Default::default();

        {
            let mut stream = sqlx::query_file_as!(
                RawData,
                "src/one_shot/get_stock_unit/get_stock_unit.sql",
                timestamp
            )
            .fetch(&mut *self.0);

            while let Some(result) = stream.next().await {
                let RawData {
                    person_id,
                    account_type_id,
                    security_id,
                    unit,
                    sign_multiplier,
                } = result?;

                let unit = unit * sign_multiplier;

                let entry = acc
                    .entry(person_id)
                    .or_default()
                    .entry(account_type_id)
                    .or_default()
                    .entry(security_id)
                    .or_default();
                *entry += unit;
            }
        }

        // (person name, account name, ticker) -> (total unit, amount)
        let mut ret: BTreeMap<(String, String, String), (MyBigDecimal, MyBigDecimal)> =
            Default::default();

        let full_names = self.get_full_names().await?;
        let account_names = get_account_names(self).await?;
        let tickers = get_tickers(self).await?;

        for (person_id, rest) in acc {
            let name = full_names
                .get(&person_id)
                .expect(&format!("person id {person_id} to exist"));
            for (account_type_id, rest) in rest {
                let account_name = account_names
                    .get(&account_type_id)
                    .expect(&format!("account type id {account_type_id} to exist"));
                for (security_id, total_unit) in rest {
                    let (ticker, price) = tickers
                        .get(&security_id)
                        .expect(&format!("security id {security_id} to exist"));

                    let entry = ret
                        .entry((name.clone(), account_name.clone(), ticker.clone()))
                        .or_default();
                    entry.0 += total_unit.clone();
                    entry.1 += (total_unit * price.clone()).round2();
                }
            }
        }

        Ok(ret
            .into_iter()
            .filter(|(_, (total_unit, _))| total_unit.is_positive())
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
