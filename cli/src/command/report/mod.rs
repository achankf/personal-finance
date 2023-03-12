mod acb;
mod balance;
mod cashflow;
mod contributions;
mod expense;
mod gl;
mod stock_transaction;
mod stock_unit;
mod transaction;

use ::transaction::{BalanceRecord, MyBigDecimal, MyTransaction, NetBalanceRecord};
use balance::report_balance;
use cashflow::report_cashflow;
use chrono::{Local, NaiveDate};
use clap::Subcommand;
use common::start_of_day;
use contributions::report_contributions;
use db::Db;
use expense::report_expense;
use tabled::Tabled;

use crate::tax_shelter_type::TaxShelterTypeValue;

use self::{
    acb::report_acb, gl::report_gl, stock_transaction::report_stock_transaction,
    stock_unit::report_stock_unit, transaction::report_transaction,
};

#[derive(Tabled)]
struct BalanceRecordFormatted {
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Account Name")]
    pub account_name: String,
    #[tabled(rename = "Balance")]
    pub balance: String,
}

impl From<BalanceRecord> for BalanceRecordFormatted {
    fn from(value: BalanceRecord) -> Self {
        Self {
            name: value.name,
            account_name: value.account_name,
            balance: format!("{}", value.balance),
        }
    }
}

#[derive(Tabled)]
struct BalanceRecordWithOwnerFormatted {
    #[tabled(rename = "Holder")]
    pub name: String,
    #[tabled(rename = "Account Name")]
    pub account_name: String,
    #[tabled(rename = "Balance")]
    pub balance: MyBigDecimal,
}

impl From<BalanceRecord> for BalanceRecordWithOwnerFormatted {
    fn from(value: BalanceRecord) -> Self {
        Self {
            name: value.name,
            account_name: value.account_name,
            balance: value.balance,
        }
    }
}

#[derive(Tabled)]
pub struct NetBalanceFormatted {
    #[tabled(rename = "Holder Name")]
    pub name: String,
    #[tabled(rename = "Balance")]
    pub balance: String,
}

impl From<NetBalanceRecord> for NetBalanceFormatted {
    fn from(value: NetBalanceRecord) -> Self {
        Self {
            name: format!("{} {}", value.first_name, value.last_name),
            balance: format!(
                "{:.2}{} {}",
                value.balance, value.currency_symbol, value.currency
            ),
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum ReportCommand {
    /// Generate a report of the adjusted cost base for a specific year.
    Acb {
        person_key: String,
        ticker: String,
        /// The `year` field is an optional integer that specifies the year for the report.
        /// If the field is not provided, the default value is the current year.
        year: Option<i32>,
    },
    /// Display your total assets, calculated as equity minus liabilities.
    Balance,
    /// Generate a report of your cash flow, including revenue and expenses, for a specific year.
    Cashflow {
        /// The `year` field is an optional integer that specifies the year for the report.
        /// If the field is not provided, the default value is the current year.
        year: Option<i32>,
    },
    /// Display expenses from the previous X days.
    Expense {
        /// The `days_prior` field is an optional unsigned 64-bit integer that specifies
        /// the number of days for which to display expense history.
        #[clap(default_value_t = 30)]
        days_prior: u64,
    },
    Gl {
        person_key: String,
        year: Option<i32>,
    },
    /// List all stock-related transactions for a specific ticker, up to a specified limit.
    StockTransaction {
        /// The `ticker` field is a string that specifies the ticker to be searched for.
        ticker: String,
        /// The `limit` field is an optional integer that specifies the maximum number of rows to be displayed.
        #[clap(default_value_t = 10)]
        limit: u32,
    },
    /// Show the number of units each person has per account by the given date.
    StockUnit {
        /// The `date` field is an optional date that specifies the date before which
        /// stock ownership should be shown. If the field is not provided, ownership will be
        /// shown up to the start of the current day (i.e., midnight). This command is useful
        /// for calculating distributions by querying data with the ex-dividend date.
        date: Option<NaiveDate>,
    },
    /// Display all transactions for a specific account_key within the specified time period.
    Transaction {
        /// The `account_key` field is a string that specifies the account to search for transactions.
        account_key: String,
        /// The `days_prior` field is an optional unsigned 64-bit integer that specifies
        /// the duration of the search range in days.
        #[clap(default_value_t = 30)]
        days_prior: u64,
    },
    ///
    Contribution {
        person_key: String,
        #[clap(value_enum)]
        tax_shelter_type: TaxShelterTypeValue,
    },
}

impl ReportCommand {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut db = Db::new().await?;
        let mut transaction = MyTransaction::try_new(&mut db).await?;

        match self {
            Self::Acb {
                person_key,
                ticker,
                year,
            } => {
                report_acb(&mut transaction, person_key, ticker, year.clone()).await?;
            }
            Self::Balance => {
                report_balance(&mut transaction).await?;
            }
            Self::Cashflow { year } => {
                report_cashflow(&mut transaction, year.clone()).await?;
            }
            Self::Expense { days_prior } => {
                report_expense(&mut transaction, *days_prior).await?;
            }
            Self::Gl { person_key, year } => {
                report_gl(&mut transaction, person_key, year).await?;
            }
            Self::StockTransaction { ticker, limit } => {
                report_stock_transaction(&mut transaction, ticker, *limit).await?;
            }
            Self::StockUnit { date } => {
                let datetime = date
                    .map(|date| start_of_day(&date))
                    .unwrap_or_else(Local::now);
                report_stock_unit(&mut transaction, datetime).await?;
            }
            Self::Transaction {
                account_key,
                days_prior,
            } => {
                report_transaction(&mut transaction, &account_key, *days_prior).await?;
            }
            Self::Contribution {
                person_key,
                tax_shelter_type,
            } => {
                //
                report_contributions(&mut transaction, person_key, tax_shelter_type).await?;
            }
        };

        transaction.commit().await?;

        Ok(())
    }
}
