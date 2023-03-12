use std::collections::BTreeMap;

use chrono::{DateTime, Datelike, Days, Local, NaiveDate};
use futures_util::StreamExt;

use crate::{MyBigDecimal, MyTransaction};

pub struct GetContributionsResult {
    pub year: u32,
    pub ytd_balance: MyBigDecimal,
    pub ytd_contributions: MyBigDecimal,
}

#[derive(Default)]
struct TempAccResult {
    ytd_balance: MyBigDecimal,
    ytd_contributions: MyBigDecimal,
}

impl From<(u32, TempAccResult)> for GetContributionsResult {
    fn from(
        (
            year,
            TempAccResult {
                ytd_balance,
                ytd_contributions,
            },
        ): (u32, TempAccResult),
    ) -> Self {
        Self {
            year,
            ytd_balance,
            ytd_contributions,
        }
    }
}

pub fn get_rrsp_first_day(year: i32) -> DateTime<Local> {
    NaiveDate::from_ymd_opt(year - 1, 12, 31)
        .expect("naive date to be created")
        .checked_add_days(Days::new(61))
        .expect("adding 60 days to be valid")
        .and_hms_opt(0, 0, 0)
        .expect("time to be augmented")
        .and_local_timezone(Local)
        .single()
        .expect("unambiguous time")
}

impl MyTransaction<'_> {
    pub async fn get_contributions(
        &mut self,
        person_key: &str,
        tax_shelter_type: &str,
    ) -> Result<Vec<GetContributionsResult>, Box<dyn std::error::Error>> {
        let is_rrsp = tax_shelter_type == "RRSP";

        let mut acc: BTreeMap<u32, TempAccResult> = Default::default(); // year -> temp result

        {
            struct RawData {
                date: DateTime<Local>,
                forex_rate: MyBigDecimal,
                unit: MyBigDecimal,
                debit: MyBigDecimal,
                credit: MyBigDecimal,
            }

            let mut stream = sqlx::query_file_as!(
                RawData,
                "src/one_shot/get_contributions.sql",
                person_key,
                tax_shelter_type
            )
            .fetch(&mut *self.0);

            while let Some(record) = stream.next().await {
                let RawData {
                    date,
                    forex_rate,
                    unit,
                    debit,
                    credit,
                } = record?;

                // adjust year in case of rrsp
                let chronological_year = date.year();
                let year = if is_rrsp {
                    let start_of_year = get_rrsp_first_day(chronological_year);
                    if date < start_of_year {
                        chronological_year - 1
                    } else {
                        chronological_year
                    }
                } else {
                    chronological_year
                };

                let entry = acc
                    .entry(
                        year.try_into()
                            .expect("year to be non-negative and within range"),
                    )
                    .or_default();

                let balance = (forex_rate.clone() * unit * (debit.clone() - credit)).round2();
                entry.ytd_balance += balance;
                entry.ytd_contributions += forex_rate * debit;
            }
        }

        Ok(acc
            .into_iter()
            .rev() // sort in descending order
            .map(Into::into)
            .collect())
    }
}
