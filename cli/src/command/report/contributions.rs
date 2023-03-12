use tabled::{object::Columns, Alignment, ModifyObject, Style, Table, Tabled};
use transaction::{GetContributionsResult, MyTransaction};

use crate::{tax_shelter_type::TaxShelterTypeValue, utils::format_cad::format_colored_cad};

pub async fn report_contributions(
    transaction: &mut MyTransaction<'_>,
    person_key: &str,
    tax_shelter_type: &TaxShelterTypeValue,
) -> Result<(), Box<dyn std::error::Error>> {
    let records = transaction
        .get_contributions(person_key, &tax_shelter_type.to_string())
        .await?;

    #[derive(Tabled)]
    struct FormattedContribution {
        #[tabled(rename = "Year")]
        pub year: u32,
        #[tabled(rename = "YTD Balance")]
        pub ytd_balance: String,
        #[tabled(rename = "YTD Contributions")]
        pub ytd_contributions: String,
    }

    let records = records.into_iter().map(
        |GetContributionsResult {
             year,
             ytd_balance: balance,
             ytd_contributions,
         }| {
            FormattedContribution {
                year,
                ytd_balance: format_colored_cad(&balance),
                ytd_contributions: format_colored_cad(&ytd_contributions),
            }
        },
    );

    println!();
    println!("Note:");
    println!("- YTD Balance include deposits and withdrawals to accounts of that year (prior years excluded)");
    println!(
        "- YTD Contributions include only deposits to accounts of that year (prior years excluded)"
    );
    println!("- for RRSP, the start date is on the 61st day of the year");
    println!(
        "{}",
        Table::new(records)
            .with(Style::rounded())
            .with(Columns::new(1..).modify().with(Alignment::right()))
            .to_string()
    );

    Ok(())
}
