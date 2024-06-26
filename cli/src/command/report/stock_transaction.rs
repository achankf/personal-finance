use bigdecimal::BigDecimal;
use chrono::{DateTime, Local};
use tabled::{object::Columns, Alignment, ModifyObject, Style, Table, Tabled};
use transaction::{StockTransaction, Transaction};

pub async fn report_stock_transaction(
    transaction: &mut Transaction<'_>,
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
        pub unit: String,
        #[tabled(rename = "Debit")]
        pub debit: String,
        #[tabled(rename = "Credit")]
        pub credit: String,
    }

    impl From<StockTransaction> for StockTransactionFormatted {
        fn from(value: StockTransaction) -> Self {
            fn format_currency(value: Option<BigDecimal>) -> String {
                value.map(|value| format!("${}", value)).unwrap_or_default()
            }

            Self {
                name: value.name,
                account_type: value.account_type,
                ticker: value.ticker,
                date: value.date,
                unit: format!("{}", value.unit),
                debit: format_currency(value.debit),
                credit: format_currency(value.credit),
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
