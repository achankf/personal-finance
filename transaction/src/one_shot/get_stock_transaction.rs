use chrono::{DateTime, Local};

use crate::{MyBigDecimal, MyTransaction};

pub struct StockTransaction {
    pub name: String,
    pub account_type: String,
    pub ticker: String,
    pub date: DateTime<Local>,
    pub unit: MyBigDecimal,
    pub debit: MyBigDecimal,
    pub credit: MyBigDecimal,
    pub balance: MyBigDecimal,
}

pub struct RawStockTransaction {
    pub name: String,
    pub account_type: String,
    pub ticker: String,
    pub date: DateTime<Local>,
    pub unit: MyBigDecimal,
    pub debit: MyBigDecimal,
    pub credit: MyBigDecimal,
}

impl From<RawStockTransaction> for StockTransaction {
    fn from(
        RawStockTransaction {
            name,
            account_type,
            ticker,
            date,
            unit,
            debit,
            credit,
        }: RawStockTransaction,
    ) -> Self {
        let balance = unit.clone() * (credit.clone() - debit.clone());
        let balance = balance.round2();

        StockTransaction {
            name,
            account_type,
            ticker,
            date,
            unit,
            debit,
            credit,
            balance,
        }
    }
}

impl MyTransaction<'_> {
    pub async fn get_stock_transaction(
        &mut self,
        ticker: &str,
        limit: u32,
    ) -> Result<Vec<StockTransaction>, Box<dyn std::error::Error>> {
        let ret = sqlx::query_file_as!(
            RawStockTransaction,
            "src/one_shot/get_stock_transaction.sql",
            ticker,
            limit
        )
        .map(Into::into)
        .fetch_all(&mut *self.0)
        .await?;

        Ok(ret)
    }
}
