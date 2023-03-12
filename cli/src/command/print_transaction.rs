use bigdecimal::{BigDecimal, Zero};
use owo_colors::OwoColorize;
use tabled::{object::Columns, Alignment, ModifyObject, Style, Table, Tabled};
use transaction::{GetTransaction, Transaction};

pub async fn print_transaction(
    transaction: &mut Transaction<'_>,
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

    fn format_balance(value: BigDecimal) -> String {
        format!(
            "${}",
            if value > BigDecimal::zero() {
                value.to_string().green().to_string()
            } else if value < BigDecimal::zero() {
                value.to_string().red().to_string()
            } else {
                value.to_string()
            }
        )
    }

    fn format_option_balance(value: Option<BigDecimal>) -> String {
        value.map(|value| format_balance(value)).unwrap_or_default()
    }

    let mut overall_balance = BigDecimal::zero();

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
            overall_balance = (overall_balance.clone() + balance.clone())
                .with_scale_round(2, bigdecimal::RoundingMode::HalfUp);

            FormattedTransaction {
                item_id,
                date: date.to_string(),
                account_key,
                account_kind,
                forex_rate: format_option_balance(forex_rate),
                unit: unit.to_string(),
                debit: format_option_balance(debit),
                credit: format_option_balance(credit),
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
