use chrono::{DateTime, Local};
use common::all_time_in_year;
use owo_colors::OwoColorize;
use tabled::{object::Columns, Alignment, ModifyObject, Style, Table, Tabled};
use transaction::{AcbEntry, AcbEntryType, MyTransaction};

use crate::utils::format_cad::{format_cad, format_colored_cad};

pub async fn report_acb(
    transaction: &mut MyTransaction<'_>,
    person_key: &str,
    ticker: &str,
    year: Option<i32>,
) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Tabled)]
    struct AcbFormatted {
        #[tabled(rename = "Date")]
        pub date: DateTime<Local>,
        #[tabled(rename = "Type")]
        pub r#type: AcbEntryType,
        #[tabled(rename = "Unit")]
        pub unit: String,
        #[tabled(rename = "Price")]
        pub price: String,
        #[tabled(rename = "Amount")]
        pub amount: String,
        #[tabled(rename = "Change in ACB")]
        pub acb_change: String,
        #[tabled(rename = "Capital GL")]
        pub capital_gl: String,
        #[tabled(rename = "ACB balance")]
        pub acb: String,
    }

    impl From<AcbEntry> for AcbFormatted {
        fn from(value: AcbEntry) -> Self {
            let unit = value.unit.round4().to_string();
            let price = format_cad(&value.price.round2());
            let amount = format_cad(&value.amount);
            let acb_change = format_colored_cad(&value.acb_change);
            let capital_gl = format_colored_cad(&value.capital_gl);
            let acb = format_cad(&value.acb.round2());

            Self {
                date: value.date,
                r#type: value.r#type,
                unit,
                price,
                amount,
                acb_change,
                capital_gl,
                acb,
            }
        }
    }

    let person_id = transaction
        .get_person_id(person_key)
        .await?
        .expect("person to exist");
    let security_id = transaction
        .get_security_id(ticker)
        .await?
        .expect("security to exist");
    let records = transaction
        .get_trade_data()
        .await?
        .get_acb(person_id, security_id);
    let records = records.iter().rev().cloned();

    let (records, title): (Box<dyn Iterator<Item = AcbEntry>>, String) = if let Some(year) = year {
        let range = all_time_in_year(year);
        let records = Box::new(records.filter(move |record| range.contains(&record.date)));
        let title = format!("Adjusted Cost Base (ACB) with Capital Gain/Loss for {year}");
        (records, title)
    } else {
        let records = Box::new(records);
        let title = "Adjusted Cost Base (ACB) with Capital Gain/Loss".into();
        (records, title)
    };

    let mut records = records.peekable();
    if records.peek().is_none() {
        return Err(format!("No record for given search criteria").into());
    }

    println!("{}", title.bold());

    println!();

    println!(
        "{}",
        "Reminder for T5008 slips: when disposing units".yellow()
    );
    println!("{}","    - the absolute amount (in red, without the negative sign) is the adjusted book cost (box 20) for disposing that amount of units.".yellow());
    println!("{}","    - the 'Amount' is the proceeds of disposition (box 21) for disposing that amount of units.".yellow());

    let formatted = records.map(AcbFormatted::from);

    println!(
        "{}",
        Table::new(formatted)
            .with(Style::rounded())
            .with(Columns::new(2..).modify().with(Alignment::right()))
            .to_string()
    );
    println!();

    Ok(())
}
