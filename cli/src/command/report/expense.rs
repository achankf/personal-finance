use bigdecimal::BigDecimal;
use tabled::{object::Columns, Alignment, ModifyObject, Style, Table, Tabled};
use transaction::{ExpenseByCategoryResult, Transaction};

pub async fn report_expense(
    transaction: &mut Transaction<'_>,
    days_prior: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let spendings = transaction
        .get_expense_balance_by_category(days_prior)
        .await?;

    let total: BigDecimal = spendings
        .iter()
        .fold(bigdecimal::Zero::zero(), |acc, record| {
            acc + record.balance.clone()
        });

    #[derive(Tabled)]
    struct FormattedRecord {
        #[tabled(rename = "Account Type")]
        pub account_type: String,
        #[tabled(rename = "Balance")]
        pub balance: String,
    }

    impl From<ExpenseByCategoryResult> for FormattedRecord {
        fn from(value: ExpenseByCategoryResult) -> Self {
            Self {
                account_type: value.account_type,
                balance: format!("{}", value.balance),
            }
        }
    }

    let formatted = spendings.into_iter().map(FormattedRecord::from);

    println!();
    println!("Expense for trailing {days_prior} days, total=${total}");
    println!(
        "{}",
        Table::new(formatted)
            .with(Style::rounded())
            .with(Columns::single(1).modify().with(Alignment::right()))
            .to_string()
    );

    Ok(())
}
