use std::collections::BTreeMap;

use tabled::{object::Columns, Alignment, ModifyObject, Style, Table, Tabled};
use transaction::{GlBreakDown, MyTransaction};

use crate::utils::format_cad::format_colored_cad;

pub async fn report_gl<'a>(
    transaction: &mut MyTransaction<'_>,
    person_key: &str,
    year: &Option<i32>,
) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Tabled)]
    struct GlBreakdownFormatted {
        ticker: String,
        #[tabled(rename = "Disposition")]
        pub disposition_capital_gl: String,
        #[tabled(rename = "Cash\nDistribution")]
        pub total_cash_distribution: String,
        #[tabled(rename = "None-Cash\nDistribution")]
        pub total_non_cash_distribution: String,
        #[tabled(rename = "Distribution from\nForeign Asset")]
        pub foreign_distribution: String,
        #[tabled(rename = "Return Of\nCapital\n(Box 42)")]
        pub return_of_capital: String,
        #[tabled(rename = "Capital Gain\nFrom Dividend\n(Box 21)")]
        pub distribution_capital_gain: String,
        #[tabled(rename = "Eligible\nDividend\n(Box 49)")]
        pub eligible_dividend: String,
        #[tabled(rename = "Non-eligible\nDividend\n(Box 23)")]
        pub non_eligible_dividend: String,
        #[tabled(rename = "Foreign Bus-\niness Income\n(Box 24)")]
        pub foreign_business_income: String,
        #[tabled(rename = "Foreign Non-\nbusiness Income\n(Box 25)")]
        pub foreign_non_business_income: String,
        #[tabled(rename = "Other Income\n\n(Box 26)")]
        pub other_income: String,
        #[tabled(rename = "Foreign Business\nIncome Tax Paid\n(Box 33)")]
        pub foreign_business_income_tax_paid: String,
        #[tabled(rename = "Foreign Non-business\nIncome Tax Paid\n(Box 34)")]
        pub foreign_non_business_income_tax_paid: String,
    }

    let person_id = match transaction.get_person_id(person_key).await? {
        Some(person_id) => Ok(person_id),
        None => Err("Unable to find person key"),
    }?;

    let tickers = transaction.get_tickers().await?;

    let format_breakdown = |ticker: &str, value: &GlBreakDown| -> GlBreakdownFormatted {
        GlBreakdownFormatted {
            ticker: ticker.to_string(),
            disposition_capital_gl: format_colored_cad(&value.disposition_capital_gl),
            total_cash_distribution: format_colored_cad(&value.total_cash_distribution),
            total_non_cash_distribution: format_colored_cad(&value.total_non_cash_distribution),
            return_of_capital: format_colored_cad(&value.return_of_capital),
            distribution_capital_gain: format_colored_cad(&value.distribution_capital_gain),
            eligible_dividend: format_colored_cad(&value.eligible_dividend),
            non_eligible_dividend: format_colored_cad(&value.non_eligible_dividend),
            foreign_business_income: format_colored_cad(&value.foreign_business_income),
            foreign_non_business_income: format_colored_cad(&value.foreign_non_business_income),
            other_income: format_colored_cad(&value.other_income),
            foreign_business_income_tax_paid: format_colored_cad(
                &value.foreign_business_income_tax_paid,
            ),
            foreign_non_business_income_tax_paid: format_colored_cad(
                &value.foreign_non_business_income_tax_paid,
            ),
            foreign_distribution: format_colored_cad(&value.foreign_distribution),
        }
    };

    let security_data = transaction.get_trade_data().await?;

    let mut total: GlBreakDown = Default::default();

    let mut records: BTreeMap<&str, GlBreakdownFormatted> = Default::default();
    for security_id in security_data.get_owned_securities(person_id) {
        let ticker = tickers
            .get(&security_id)
            .expect("security_id should be matched");

        let breakdown = security_data.get_single_breakdown(person_id, security_id, year);
        if breakdown.has_activity() {
            records.insert(ticker, format_breakdown(ticker, &breakdown));
        }

        total = total + breakdown;
    }

    if records.is_empty() {
        return Err(format!("No record for given search criteria").into());
    }

    let mut formatted: Vec<_> = records
        .into_values()
        .map(GlBreakdownFormatted::from)
        .collect();
    formatted.push(format_breakdown("Total", &total));

    println!();
    println!(
        "{}",
        Table::new(formatted)
            .with(Style::rounded())
            .with(Columns::new(1..).modify().with(Alignment::right()))
            .to_string()
    );
    println!();

    Ok(())
}
