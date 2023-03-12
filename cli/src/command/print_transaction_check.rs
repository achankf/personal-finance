use bigdecimal::{BigDecimal, Zero};
use owo_colors::OwoColorize;
use tabled::{object::Columns, Alignment, ModifyObject, Style, Table, Tabled};
use transaction::{AssertTransactionBalance, CheckTransactionStore, Transaction};

pub async fn print_transaction_check(
    transaction: &mut Transaction<'_>,
) -> Result<(), Box<dyn std::error::Error>> {
    {
        #[derive(Tabled)]
        struct RecordFormatted {
            #[tabled(rename = "Account Key")]
            account_key: String,
            #[tabled(rename = "Date")]
            date: String,
            #[tabled(rename = "Transaction ID")]
            transaction_id: i64,
            #[tabled(rename = "Balance")]
            balance: String,
            #[tabled(rename = "Description")]
            description: String,
        }

        impl From<CheckTransactionStore> for RecordFormatted {
            fn from(value: CheckTransactionStore) -> Self {
                Self {
                    account_key: value.account_key,
                    date: value.date.to_string(),
                    transaction_id: value.transaction_id,
                    balance: format!("${:.2}", value.balance),
                    description: value.description,
                }
            }
        }

        let transaction_store = transaction.check_transaction_store().await?;

        if !transaction_store.is_empty() {
            let formatted = transaction_store.into_iter().map(RecordFormatted::from);

            println!(
                "{}",
                format!("The following purchases have incomplete merchant information")
                    .yellow()
                    .bold()
            );
            println!(
                "{}",
                Table::new(formatted)
                    .with(Style::rounded())
                    .with(Columns::new(2..=3).modify().with(Alignment::right()))
                    .to_string()
            );
            println!();
        }
    }

    let results = transaction.check_transaction_balance().await?;

    if !results.per_entry_imbalances.is_empty() {
        #[derive(Tabled)]
        struct RecordFormatted {
            transaction_id: i64,
            debit: String,
            credit: String,
            balance: String,
        }

        impl From<AssertTransactionBalance> for RecordFormatted {
            fn from(value: AssertTransactionBalance) -> Self {
                Self {
                    transaction_id: value.transaction_id,
                    debit: format!("${}", value.debit),
                    credit: format!("${}", value.credit),
                    balance: format!("${}", value.balance),
                }
            }
        }

        let formatted = results
            .per_entry_imbalances
            .into_iter()
            .map(RecordFormatted::from);

        println!(
            "{}",
            format!("Credit & debit aren't balanced!")
                .to_string()
                .yellow()
                .bold()
        );
        println!(
            "{}",
            Table::new(formatted)
                .with(Style::rounded())
                .with(Columns::new(2..).modify().with(Alignment::right()))
                .to_string()
        );
        println!();
    } else {
        let total_debit = results.total_debit;
        let total_credit = results.total_credit;

        if !BigDecimal::is_zero(&(total_debit.clone() - total_credit.clone())) {
            println!(
                "{}",
                format!(
                    "Asset balance doesn't match its double-entry counterpart, {}|{}",
                    total_debit, total_credit
                )
                .bold()
            );
            println!();
        }
    }

    Ok(())
}
