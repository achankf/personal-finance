use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Local};
use futures::StreamExt;
use serde::Deserialize;

use crate::{MyBigDecimal, MyTransaction};

#[derive(Debug, PartialEq, Eq)]
pub struct DistributionRefResult {
    pub distribution_id: i64,

    pub ex_dividend_date: DateTime<Local>,
    pub record_date: DateTime<Local>,
    pub payment_date: DateTime<Local>,

    pub total_cash_distribution: MyBigDecimal,
    // decreases ACB
    pub total_non_cash_distribution: MyBigDecimal,
    // increases ACB
    pub return_of_capital: MyBigDecimal,

    // capital gain
    pub capital_gain: MyBigDecimal, // pre-2023 when 50% of the gain was taxed
    pub capital_gain_pre_20240624: MyBigDecimal,
    pub capital_gain_post_20240624: MyBigDecimal, // 66% of the gain was taxed

    pub eligible_dividend: MyBigDecimal,
    pub non_eligible_dividend: MyBigDecimal,
    pub foreign_business_income: MyBigDecimal,
    pub foreign_non_business_income: MyBigDecimal,
    pub other_income: MyBigDecimal,
    pub foreign_business_income_tax_paid: MyBigDecimal,
    pub foreign_non_business_income_tax_paid: MyBigDecimal,
    pub foreign_distribution: MyBigDecimal,
}

// security_id -> payment date (timestamp) -> result
pub type DistributionRefMap = HashMap<i64, BTreeMap<DateTime<Local>, DistributionRefResult>>;

#[derive(Clone, Deserialize, Debug)]
struct RawRecord {
    security_id: i64,
    distribution_id: i64,
    ex_dividend_date: DateTime<Local>,
    record_date: DateTime<Local>,
    payment_date: DateTime<Local>,
    total_cash_distribution: MyBigDecimal,
    total_non_cash_distribution: MyBigDecimal,
    distribution_tax_breakdown_type_id: i64,
    return_of_capital: MyBigDecimal,
    capital_gain: MyBigDecimal,
    capital_gain_pre_20240624: MyBigDecimal,
    capital_gain_post_20240624: MyBigDecimal,
    eligible_dividend: MyBigDecimal,
    non_eligible_dividend: MyBigDecimal,
    foreign_business_income: MyBigDecimal,
    foreign_non_business_income: MyBigDecimal,
    other_income: MyBigDecimal,
    foreign_business_income_tax_paid: MyBigDecimal,
    foreign_non_business_income_tax_paid: MyBigDecimal,
    foreign_distribution: MyBigDecimal,
}

impl MyTransaction<'_> {
    pub async fn get_distribution_ref(
        &mut self,
    ) -> Result<DistributionRefMap, Box<dyn std::error::Error>> {
        let distribution_tax_breakdown_types = self.get_distribution_tax_breakdown_types().await?;

        let mut stream = sqlx::query_file_as!(RawRecord, "src/one_shot/get_distribution_ref.sql")
            .fetch(&mut *self.0);

        let mut distribution_ref_map: DistributionRefMap = Default::default();

        while let Some(record) = stream.next().await {
            let RawRecord {
                security_id,
                distribution_id,
                ex_dividend_date,
                record_date,
                payment_date,
                total_cash_distribution,
                total_non_cash_distribution,
                return_of_capital,
                capital_gain,
                capital_gain_pre_20240624,
                capital_gain_post_20240624,
                eligible_dividend,
                non_eligible_dividend,
                foreign_business_income,
                foreign_non_business_income,
                other_income,
                foreign_business_income_tax_paid,
                foreign_non_business_income_tax_paid,
                foreign_distribution,
                distribution_tax_breakdown_type_id,
            } = record?;

            let ret: DistributionRefResult = if distribution_tax_breakdown_types
                .is_percent(distribution_tax_breakdown_type_id)
            {
                let total_distribution =
                    total_cash_distribution.clone() + total_non_cash_distribution.clone();

                let to_rate = {
                    let total_distribution = total_distribution.clone();

                    move |percent: MyBigDecimal| total_distribution.clone() * percent / 100
                };

                DistributionRefResult {
                    distribution_id,
                    ex_dividend_date,
                    record_date,
                    payment_date,
                    total_cash_distribution,
                    total_non_cash_distribution,
                    return_of_capital: to_rate(return_of_capital),
                    capital_gain: to_rate(capital_gain),
                    capital_gain_pre_20240624: to_rate(capital_gain_pre_20240624),
                    capital_gain_post_20240624: to_rate(capital_gain_post_20240624),
                    eligible_dividend: to_rate(eligible_dividend),
                    non_eligible_dividend: to_rate(non_eligible_dividend),
                    foreign_business_income: to_rate(foreign_business_income),
                    foreign_non_business_income: to_rate(foreign_non_business_income),
                    other_income: to_rate(other_income),
                    foreign_business_income_tax_paid: to_rate(foreign_business_income_tax_paid),
                    foreign_non_business_income_tax_paid: to_rate(
                        foreign_non_business_income_tax_paid,
                    ),
                    foreign_distribution: to_rate(foreign_distribution),
                }
            } else {
                DistributionRefResult {
                    distribution_id,
                    ex_dividend_date,
                    record_date,
                    payment_date,
                    total_cash_distribution,
                    total_non_cash_distribution,
                    return_of_capital,
                    capital_gain,
                    capital_gain_pre_20240624,
                    capital_gain_post_20240624,
                    eligible_dividend,
                    non_eligible_dividend,
                    foreign_business_income,
                    foreign_non_business_income,
                    other_income,
                    foreign_business_income_tax_paid,
                    foreign_non_business_income_tax_paid,
                    foreign_distribution,
                }
            };

            distribution_ref_map
                .entry(security_id)
                .or_default()
                .insert(payment_date.clone(), ret);
        }

        Ok(distribution_ref_map)
    }
}
