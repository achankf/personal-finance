use chrono::{DateTime, Local};
use futures_util::StreamExt;
use num_traits::Zero;

use crate::{MyBigDecimal, MyTransaction};

pub struct DistributionRefSumZeroCheckResult {
    pub ticker: String,
    pub record_date: DateTime<Local>,
    pub unused_flag: bool,
    pub total_distribution: MyBigDecimal,
    pub taxable_gain: MyBigDecimal,
    pub foreign_tax: MyBigDecimal,
}

#[derive(sqlx::FromRow)]
struct RawData {
    pub ticker: String,
    pub record_date: DateTime<Local>,
    pub unused_flag: bool,
    pub total_cash_distribution: MyBigDecimal,
    pub total_non_cash_distribution: MyBigDecimal,
    pub distribution_tax_breakdown_type_id: i64,
    pub capital_gain: MyBigDecimal,
    pub capital_gain_pre_20240624: MyBigDecimal,
    pub capital_gain_post_20240624: MyBigDecimal,
    pub eligible_dividend: MyBigDecimal,
    pub foreign_non_business_income: MyBigDecimal,
    pub other_income: MyBigDecimal,
    pub return_of_capital: MyBigDecimal,
    pub non_eligible_dividend: MyBigDecimal,
    pub foreign_business_income: MyBigDecimal,
    pub non_reportable_distribution: MyBigDecimal,
    pub capital_gains_eligible_for_deduction: MyBigDecimal,
    pub foreign_distribution: MyBigDecimal,
    pub foreign_non_business_income_tax_paid: MyBigDecimal,
    pub foreign_business_income_tax_paid: MyBigDecimal,
}

impl MyTransaction<'_> {
    pub async fn check_distribution_ref(
        &mut self,
    ) -> Result<Vec<DistributionRefSumZeroCheckResult>, Box<dyn std::error::Error>> {
        let distribution_tax_breakdown_types = self.get_distribution_tax_breakdown_types().await?;

        let mut stream = sqlx::query_file_as!(RawData, "src/check/check_distribution_ref.sql")
            .fetch(&mut *self.0);

        let mut result: Vec<_> = vec![];

        while let Some(record) = stream.next().await {
            let RawData {
                ticker,
                record_date,
                unused_flag,
                total_cash_distribution,
                total_non_cash_distribution,
                distribution_tax_breakdown_type_id,
                capital_gain,
                capital_gain_pre_20240624,
                capital_gain_post_20240624,
                eligible_dividend,
                foreign_non_business_income,
                other_income,
                return_of_capital,
                non_eligible_dividend,
                foreign_business_income,
                non_reportable_distribution,
                capital_gains_eligible_for_deduction,
                foreign_distribution,
                foreign_non_business_income_tax_paid,
                foreign_business_income_tax_paid,
            } = record?;

            let total_distribution = total_cash_distribution.clone() + total_non_cash_distribution;
            let total_capital_gain =
                capital_gain + capital_gain_pre_20240624 + capital_gain_post_20240624;

            let taxable_gain = total_capital_gain
                + eligible_dividend
                + foreign_non_business_income
                + other_income
                + return_of_capital
                + non_eligible_dividend
                + foreign_business_income
                + non_reportable_distribution
                + capital_gains_eligible_for_deduction
                + foreign_distribution;

            let foreign_tax =
                foreign_non_business_income_tax_paid + foreign_business_income_tax_paid;

            if {
                // sanity check to make sure CDS data match with double counting
                let total_distribution = total_distribution.clone().round(5); // CDS spreadsheet rounds values to 5-decimal places
                let calculated_diff = (taxable_gain.clone() - foreign_tax.clone()).round(5);

                if distribution_tax_breakdown_types.is_foreign(distribution_tax_breakdown_type_id)
                    || distribution_tax_breakdown_types.is_rate(distribution_tax_breakdown_type_id)
                {
                    !(total_distribution - calculated_diff).is_zero()
                } else {
                    assert!(distribution_tax_breakdown_types
                        .is_percent(distribution_tax_breakdown_type_id));

                    calculated_diff != MyBigDecimal::from(100) // gain - tax should sum to 100%; total_distribution not tested
                }
            } {
                result.push(DistributionRefSumZeroCheckResult {
                    ticker,
                    record_date,
                    unused_flag,
                    total_distribution,
                    taxable_gain,
                    foreign_tax,
                });
            }
        }

        Ok(result)
    }
}
