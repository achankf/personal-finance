use db::SqlResult;
use owo_colors::OwoColorize;
use tabled::{object::Columns, Alignment, ModifyObject, Style, Table, Tabled};
use transaction::{
    DistributionCheckDistributionError, DistributionCheckFinancialEntryError,
    DistributionRefSumZeroCheckResult, Transaction,
};

pub async fn print_distribution_check(transaction: &mut Transaction<'_>) -> SqlResult<()> {
    let bad_distribution_refs = transaction.check_distribution_ref().await?;

    if !bad_distribution_refs.is_empty() {
        #[derive(Tabled)]
        struct RecordFormatted {
            #[tabled(rename = "Ticker")]
            ticker: String,
            #[tabled(rename = "Record Date")]
            record_date: String,
            #[tabled(rename = "Unused")]
            unused_flag: String,
            #[tabled(rename = "Total Distribution")]
            total_distribution: f64,
            #[tabled(rename = "Taxable Gain")]
            taxable_gain: String,
            #[tabled(rename = "Foreign Tax")]
            foreign_tax: String,
        }

        impl From<DistributionRefSumZeroCheckResult> for RecordFormatted {
            fn from(value: DistributionRefSumZeroCheckResult) -> Self {
                Self {
                    ticker: value.ticker,
                    record_date: value.record_date.format("%F").to_string(),
                    unused_flag: if value.unused_flag {
                        "✓".green().to_string()
                    } else {
                        "".to_string()
                    },
                    total_distribution: value.total_distribution,
                    taxable_gain: value.taxable_gain.to_string().green().to_string(),
                    foreign_tax: value.foreign_tax.to_string().red().to_string(),
                }
            }
        }

        let formatted = bad_distribution_refs.into_iter().map(RecordFormatted::from);

        println!(
            "{}",
            format!(
                "There are distribution references where total distributions don't match its tax-reportable parts.")
                    .to_string()
                    .yellow()
                    .bold());
        println!(
            "{}",
            format!("Also, remove unused distribution entries!")
                .to_string()
                .yellow()
                .bold()
        );
        println!(
            "{}",
            Table::new(formatted)
                .with(Style::rounded())
                .with(Columns::new(3..=5).modify().with(Alignment::right()))
                .with(Columns::single(2).modify().with(Alignment::center()))
                .to_string()
        );
        println!();
    }

    let bad_distributions = transaction.check_distribution().await?;

    if !bad_distributions.distribution_errors.is_empty() {
        #[derive(Tabled)]
        struct RecordFormatted {
            #[tabled(rename = "Transaction Id")]
            transaction_id: i64,
            #[tabled(rename = "Ticker")]
            ticker: String,
        }

        impl From<DistributionCheckDistributionError> for RecordFormatted {
            fn from(value: DistributionCheckDistributionError) -> Self {
                Self {
                    transaction_id: value.transaction_id,
                    ticker: value.ticker,
                }
            }
        }

        let formatted = bad_distributions
            .distribution_errors
            .into_iter()
            .map(RecordFormatted::from);

        println!(
            "{}",
            format!(
                "These transactions don't match distribution database (either one could be wrong, check payment date and cash distribution).")
                    .to_string()
                    .yellow()
                    .bold());
        println!(
            "{}",
            Table::new(formatted)
                .with(Style::rounded())
                .with(Columns::single(2).modify().with(Alignment::right()))
                .to_string()
        );
        println!();
    }

    if !bad_distributions.financial_entry_errors.is_empty() {
        #[derive(Tabled)]
        struct RecordFormatted {
            #[tabled(rename = "Account Key")]
            account_key: String,
            #[tabled(rename = "Ticker")]
            ticker: String,
            #[tabled(rename = "Ex-dividend Date")]
            ex_dividend_date: String,
            #[tabled(rename = "Payment Date")]
            payment_date: String,
            #[tabled(rename = "Transaction Id")]
            transaction_id: String,
            #[tabled(rename = "Held Unit")]
            held_unit: String,
        }

        impl From<DistributionCheckFinancialEntryError> for RecordFormatted {
            fn from(value: DistributionCheckFinancialEntryError) -> Self {
                Self {
                    transaction_id: value
                        .transaction_id
                        .map(|id| id.to_string())
                        .unwrap_or("".to_string()),
                    ticker: value.ticker,
                    ex_dividend_date: value.ex_dividend_date.format("%F").to_string(),
                    payment_date: value.payment_date.format("%F").to_string(),
                    account_key: value.account_key,
                    held_unit: value
                        .held_unit
                        .map(|unit| format!("{:.4}", unit))
                        .unwrap_or("".to_string()),
                }
            }
        }

        let formatted = bad_distributions
            .financial_entry_errors
            .into_iter()
            .map(RecordFormatted::from);

        println!(
            "{}",
            format!("There are distributions that don't have a Financial Entry.")
                .to_string()
                .yellow()
                .bold()
        );
        println!(
            "{}",
            Table::new(formatted)
                .with(Style::rounded())
                .with(Columns::single(5).modify().with(Alignment::right()))
                .to_string()
        );
        println!();
    }

    Ok(())
}
