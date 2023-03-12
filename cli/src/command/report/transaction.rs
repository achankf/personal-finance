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
        pub unit: BigDecimal,
        #[tabled(rename = "Debit")]
        pub debit: String,
        #[tabled(rename = "Credit")]
        pub credit: String,
        #[tabled(rename = "Forex")]
        pub forex_rate: String,
        #[tabled(rename = "Total $")]
        pub total_amount: BigDecimal,
        #[tabled(rename = "Description")]
        pub description: String,
    }

    impl From<TransactionByAccountKey> for TransactionByAccountKeyFormatted {
        fn from(value: TransactionByAccountKey) -> Self {
            let debit = value
                .debit
                .map(|debit| debit.to_string())
                .unwrap_or_default();

            let credit = value
                .credit
                .map(|credit| credit.to_string())
                .unwrap_or_default();

            let forex_rate = value
                .forex_rate
                .map(|forex_rate| forex_rate.to_string())
                .unwrap_or("N/A".into());

            Self {
                transaction_id: to_string_or_na(value.transaction_id),
                item_id: to_string_or_na(value.item_id),
                date: value.date,
                unit: value.unit,
                debit,
                credit,
                forex_rate,
                total_amount: value.total_amount,
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
        .map(TransactionByAccountKeyFormatted::from);

    println!();
    println!("Transaction for {account_key} trailing {days_prior} days; total debit=${total_debit}, credit=${total_credit}, balance=${total}");
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
