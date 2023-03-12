use bigdecimal::BigDecimal;
use chrono::{DateTime, Local};
use common::days_prior_until_end_of_today;
use tabled::{object::Columns, Alignment, ModifyObject, Style, Table, Tabled};
use transaction::{Transaction, TransactionByAccountKey};

fn to_string_or_na<T: ToString>(val: Option<T>) -> String {
    val.map(|val| val.to_string()).unwrap_or("N/A".to_string())
}

pub async fn report_transaction(
    transaction: &mut Transaction<'_>,
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
        pub unit: String,
        #[tabled(rename = "Debit")]
        pub debit: String,
        #[tabled(rename = "Credit")]
        pub credit: String,
        #[tabled(rename = "Forex")]
        pub exchange_rate: String,
        #[tabled(rename = "Total $")]
        pub total_amount: String,
        #[tabled(rename = "Description")]
        pub description: String,
    }

    impl From<TransactionByAccountKey> for TransactionByAccountKeyFormatted {
        fn from(value: TransactionByAccountKey) -> Self {
            Self {
                transaction_id: to_string_or_na(value.transaction_id),
                item_id: to_string_or_na(value.item_id),
                date: value.date,
                unit: format!("{:.4}", value.unit),
                debit: if let Some(debit) = value.debit {
                    format!("${:.2}", debit)
                } else {
                    "".into()
                },
                credit: if let Some(credit) = value.credit {
                    format!("${:.2}", credit)
                } else {
                    "".into()
                },
                exchange_rate: if let Some(exchange_rate) = value.exchange_rate {
                    format!("{:.4}", exchange_rate)
                } else {
                    "N/A".into()
                },
                total_amount: format!("${:.2}", value.total_amount),
                description: value.description,
            }
        }
    }

    let records = transaction
        .get_transaction_by_account_key(account_key, days_prior_until_end_of_today(days_prior))
        .await?;

    let (total_debit, total_credit) = records.iter().fold(
        (bigdecimal::Zero::zero(), bigdecimal::Zero::zero()),
        |(debit, credit), record| {
            let unit: BigDecimal = record.unit.clone();
            let debit: BigDecimal = debit + unit.clone() * record.debit.clone().unwrap_or_default();
            let credit: BigDecimal = credit + unit * record.credit.clone().unwrap_or_default();
            (debit, credit)
        },
    );
    let total = total_debit.clone() - total_credit.clone();

    let formatted = records
        .into_iter()
        .filter(|record| record.debit.is_some() || record.credit.is_some())
        .map(TransactionByAccountKeyFormatted::from);

    println!();
    println!("Transaction for {account_key} trailing {days_prior} days; total debit=${total_debit:.2}, credit=${total_credit:.2}, balance=${total:.2}");
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
