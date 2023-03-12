use chrono::{DateTime, Local};
use db::SqlResult;
use owo_colors::OwoColorize;
use tabled::{object::Columns, Alignment, ModifyObject, Style, Table, Tabled};
use transaction::{DistributionCheckResult, DistributionRefSumZeroCheckResult, Transaction};

pub async fn print_distribution_check(transaction: &mut Transaction<'_>) -> SqlResult<()> {
    let bad_distribution_refs = transaction.check_distribution_ref().await?;

    if !bad_distribution_refs.is_empty() {
        #[derive(Tabled)]
        struct RecordFormatted {
            #[tabled(rename = "Ticker")]
            ticker: String,
            #[tabled(rename = "Record Date")]
            record_date: DateTime<Local>,
            #[tabled(rename = "Unused")]
            unused_flag: String,
            #[tabled(rename = "Total Distribution")]
            total_distribution: String,
            #[tabled(rename = "Taxable Gain")]
            taxable_gain: String,
            #[tabled(rename = "Foreign Tax")]
            foreign_tax: String,
        }

        impl From<DistributionRefSumZeroCheckResult> for RecordFormatted {
            fn from(value: DistributionRefSumZeroCheckResult) -> Self {
                Self {
                    ticker: value.ticker,
                    record_date: value.record_date,
                    unused_flag: if value.unused_flag {
                        "✓".green().to_string()
                    } else {
                        "".to_string()
                    },
                    total_distribution: format!("${:.2}", value.total_distribution),
                    taxable_gain: format!("${:.2}", value.taxable_gain).green().to_string(),
                    foreign_tax: format!("${:.2}", value.foreign_tax).red().to_string(),
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

    if !bad_distributions.is_empty() {
        #[derive(Tabled)]
        struct RecordFormatted {
            #[tabled(rename = "Transaction Id")]
            transaction_id: i64,
            #[tabled(rename = "Ticker")]
            ticker: String,
        }

        impl From<DistributionCheckResult> for RecordFormatted {
            fn from(value: DistributionCheckResult) -> Self {
                Self {
                    transaction_id: value.transaction_id,
                    ticker: value.ticker,
                }
            }
        }

        let formatted = bad_distributions.into_iter().map(RecordFormatted::from);

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

    Ok(())
}
