use num_traits::{Signed, Zero};
use owo_colors::OwoColorize;
use tabled::{object::Columns, Alignment, ModifyObject, Style, Table, Tabled};
use transaction::{
    AssertTransactionBalance, CheckTransactionStore, MyBigDecimal, MyTransaction, PendingEntry,
};

pub async fn print_transaction_check(
    transaction: &mut MyTransaction<'_>,
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

    if !results.pending_entries.is_empty() {
        #[derive(Tabled)]
        struct RecordFormatted {
            transaction_id: i64,
            debit: String,
            credit: String,
            balance: String,
            note: String,
        }

        impl From<PendingEntry> for RecordFormatted {
            fn from(value: PendingEntry) -> Self {
                Self {
                    transaction_id: value.transaction_id,
                    debit: format_balance(value.debit),
                    credit: format_balance(value.credit),
                    balance: format_balance(value.balance),
                    note: value.note,
                }
            }
        }

        let formatted = results
            .pending_entries
            .into_iter()
            .map(RecordFormatted::from);

        println!(
            "{}",
            format!("Pending Transactions").to_string().yellow().bold()
        );
        println!(
            "{}",
            Table::new(formatted)
                .with(Style::rounded())
                .with(Columns::new(1..=3).modify().with(Alignment::right()))
                .to_string()
        );
        println!();
    }

    if !results.imbalances.is_empty() {
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
                    debit: format_balance(value.debit),
                    credit: format_balance(value.credit),
                    balance: format_balance(value.balance),
                }
            }
        }

        let formatted = results.imbalances.into_iter().map(RecordFormatted::from);

        println!(
            "{}",
            format!("Credit & debit aren't balanced!")
                .to_string()
                .red()
                .bold()
        );
        println!(
            "{}",
            Table::new(formatted)
                .with(Style::rounded())
                .with(Columns::new(1..).modify().with(Alignment::right()))
                .to_string()
        );
        println!();
    } else {
        let total_debit = results.total_debit;
        let total_credit = results.total_credit;

        if !(total_debit.clone() - total_credit.clone()).is_zero() {
            println!(
                "{}",
                format!(
                    "Asset balance doesn't match its double-entry counterpart, {}|{}",
                    total_debit, total_credit
                )
                .red()
                .bold()
            );
            println!();
        }
    }

    Ok(())
}
