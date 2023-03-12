use chrono::{DateTime, Local};
use tabled::{object::Columns, Alignment, ModifyObject, Style, Table, Tabled};
use transaction::{MyBigDecimal, MyTransaction, StockTransaction};

use crate::utils::format_cad::format_cad;

pub async fn report_stock_transaction(
    transaction: &mut MyTransaction<'_>,
    ticker: &str,
    limit: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Tabled)]
    struct StockTransactionFormatted {
        #[tabled(rename = "Owner")]
        pub name: String,
        #[tabled(rename = "Account Name")]
        pub account_type: String,
        #[tabled(rename = "Ticker")]
        pub ticker: String,
        #[tabled(rename = "Date")]
        pub date: DateTime<Local>,
        #[tabled(rename = "Unit")]
        pub unit: MyBigDecimal,
        #[tabled(rename = "Debit")]
        pub debit: String,
        #[tabled(rename = "Credit")]
        pub credit: String,
        #[tabled(rename = "Balance")]
        pub balance: String,
    }

    impl From<StockTransaction> for StockTransactionFormatted {
        fn from(
            StockTransaction {
                name,
                account_type,
                ticker,
                date,
                unit,
                debit,
                credit,
                balance,
            }: StockTransaction,
        ) -> Self {
            Self {
                name,
                account_type,
                ticker,
                date,
                unit,
                debit: format_cad(&debit),
                credit: format_cad(&credit),
                balance: format_cad(&balance),
            }
        }
    }

    let records = transaction.get_stock_transaction(ticker, limit).await?;

    let formatted = records.into_iter().map(StockTransactionFormatted::from);

    println!();
    println!("Stock Transactions");
    println!(
        "{}",
        Table::new(formatted)
            .with(Style::rounded())
            .with(Columns::new(4..).modify().with(Alignment::right()))
            .to_string()
    );

    Ok(())
}
