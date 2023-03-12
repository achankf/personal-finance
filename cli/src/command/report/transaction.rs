use chrono::{DateTime, Local};
use common::days_prior_until_end_of_today;
use num_traits::One;
use tabled::{object::Columns, Alignment, ModifyObject, Style, Table, Tabled};
use transaction::MyBigDecimal;
use transaction::{MyTransaction, TransactionByAccountKey};

use crate::utils::format_cad::format_cad;

fn to_string_or_na<T: ToString>(val: Option<T>) -> String {
    val.map(|val| val.to_string()).unwrap_or("N/A".to_string())
}

pub async fn report_transaction(
    transaction: &mut MyTransaction<'_>,
    account_key: &str,
    days_prior: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Tabled)]
    struct TransactionByAccountKeyFormatted {
        #[tabled(rename = "Transaction ID")]
        pub transaction_id: String,
        #[tabled(rename = "Item ID")]
        pub item_id: String,
        #[tabled(rename = "Transaction Date")]
        pub date: DateTime<Local>,
        #[tabled(rename = "Unit")]
        pub unit: MyBigDecimal,
        #[tabled(rename = "Debit")]
        pub debit: String,
        #[tabled(rename = "Credit")]
        pub credit: String,
        #[tabled(rename = "Forex")]
        pub forex_rate: String,
        #[tabled(rename = "Balance")]
        pub balance: String,
        #[tabled(rename = "Description")]
        pub description: String,
    }

    impl From<TransactionByAccountKey> for TransactionByAccountKeyFormatted {
        fn from(
            TransactionByAccountKey {
                transaction_id,
                item_id,
                date,
                unit,
                debit,
                credit,
                forex_rate,
                balance,
                description,
            }: TransactionByAccountKey,
        ) -> Self {
            let forex_rate = if forex_rate.is_one() {
                // meaning transaction is done in CAD so not applicable
                "N/A".into()
            } else {
                forex_rate.to_string()
            };

            Self {
                transaction_id: to_string_or_na(transaction_id),
                item_id: to_string_or_na(item_id),
                date,
                unit,
                debit: format_cad(&debit),
                credit: format_cad(&credit),
                forex_rate,
                balance: format_cad(&balance),
                description,
            }
        }
    }

    let records = transaction
        .get_transaction_by_account_key(account_key, days_prior_until_end_of_today(days_prior))
        .await?;

    let (total_debit, total_credit, total_balance) = records.summarize_total();

    let formatted = records
        .into_iter()
        .map(TransactionByAccountKeyFormatted::from);

    println!();
    println!("Transaction for {account_key} trailing {days_prior} days; total debit=${total_debit}, credit=${total_credit}, balance=${total_balance}");
    println!(
        "{}",
        Table::new(formatted)
            .with(Style::rounded())
            .with(Columns::new(0..=1).modify().with(Alignment::right()))
            .with(Columns::new(3..=7).modify().with(Alignment::right()))
            .to_string()
    );

    Ok(())
}
