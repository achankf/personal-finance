use num_traits::{Signed, Zero};
use owo_colors::OwoColorize;
use tabled::{object::Columns, Alignment, ModifyObject, Style, Table, Tabled};
use transaction::{GetTransaction, MyBigDecimal, MyTransaction};

pub async fn print_transaction(
    transaction: &mut MyTransaction<'_>,
    transaction_id: &i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let records = transaction.get_transaction(transaction_id).await?;

    #[derive(Tabled)]
    struct FormattedTransaction {
        #[tabled(rename = "Item #")]
        pub item_id: i64,
        #[tabled(rename = "Date")]
        pub date: String,
        #[tabled(rename = "Key")]
        pub account_key: String,
        #[tabled(rename = "Kind")]
        pub account_kind: String,
        #[tabled(rename = "Forex")]
        pub forex_rate: String,
        #[tabled(rename = "Unit")]
        pub unit: String,
        #[tabled(rename = "Debit")]
        pub debit: String,
        #[tabled(rename = "Credit")]
        pub credit: String,
        #[tabled(rename = "Balance")]
        pub balance: String,
        #[tabled(rename = "Description")]
        pub description: String,
    }

    fn format_balance(value: MyBigDecimal) -> String {
        format!(
            "${}",
            if value.is_positive() {
                value.to_string().green().to_string()
            } else if value.is_negative() {
                value.to_string().red().to_string()
            } else {
                value.to_string()
            }
        )
    }

    let mut overall_balance: MyBigDecimal = Zero::zero();

    let records = records.into_iter().map(
        |GetTransaction {
             item_id,
             date,
             account_key,
             account_kind,
             forex_rate,
             unit,
             debit,
             credit,
             balance,
             description,
         }| {
            overall_balance = (overall_balance.clone() + balance.clone()).round2();

            FormattedTransaction {
                item_id,
                date: date.to_string(),
                account_key,
                account_kind,
                forex_rate: format_balance(forex_rate),
                unit: unit.to_string(),
                debit: format_balance(debit),
                credit: format_balance(credit),
                balance: format_balance(balance),
                description,
            }
        },
    );

    println!();
    println!(
        "{}",
        Table::new(records)
            .with(Style::rounded())
            .with(Columns::new(1..).modify().with(Alignment::right()))
            .to_string()
    );

    if !overall_balance.is_zero() {
        println!(
            "Transaction isn't balanced; overall balance is {}",
            format_balance(overall_balance)
        )
    }

    Ok(())
}
