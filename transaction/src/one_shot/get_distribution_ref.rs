use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use bigdecimal::BigDecimal;
use chrono::{DateTime, Local};
use futures::StreamExt;
use serde::Deserialize;

use crate::Transaction;

#[derive(Debug, PartialEq, Eq)]
pub struct DistributionRefResult {
    pub distribution_id: i64,

    pub ex_dividend_date: DateTime<Local>,
    pub record_date: DateTime<Local>,
    pub payment_date: DateTime<Local>,

    pub total_cash_distribution: BigDecimal,
    // decreases ACB
    pub total_non_cash_distribution: BigDecimal,
    // increases ACB
    pub return_of_capital: BigDecimal,

    pub capital_gain: BigDecimal,
    pub eligible_dividend: BigDecimal,
    pub non_eligible_dividend: BigDecimal,
    pub foreign_business_income: BigDecimal,
    pub foreign_non_business_income: BigDecimal,
    pub other_income: BigDecimal,
    pub foreign_distribution: BigDecimal,
}

// security_id -> payment date (timestamp) -> result
pub type DistributionRefMap = HashMap<i64, BTreeMap<DateTime<Local>, DistributionRefResult>>;

impl Transaction<'_> {
    pub async fn get_distribution_ref(
        &mut self,
    ) -> Result<DistributionRefMap, Box<dyn std::error::Error>> {
        #[derive(Clone, Deserialize, Debug)]
        struct RawRecord {
            security_id: i64,
            distribution_id: i64,
            ex_dividend_date: DateTime<Local>,
            record_date: DateTime<Local>,
            payment_date: DateTime<Local>,
            total_cash_distribution: String,
            total_non_cash_distribution: String,
            return_of_capital: String,
            capital_gain: String,
            eligible_dividend: String,
            non_eligible_dividend: String,
            foreign_business_income: String,
            foreign_non_business_income: String,
            other_income: String,
            foreign_distribution: String,
        }

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
                eligible_dividend,
                non_eligible_dividend,
                foreign_business_income,
                foreign_non_business_income,
                other_income,
                foreign_distribution,
            } = record?;

            let total_cash_distribution = BigDecimal::from_str(&total_cash_distribution)?;
            let total_non_cash_distribution = BigDecimal::from_str(&total_non_cash_distribution)?;
            let return_of_capital = BigDecimal::from_str(&return_of_capital)?;
            let capital_gain = BigDecimal::from_str(&capital_gain)?;
            let eligible_dividend = BigDecimal::from_str(&eligible_dividend)?;
            let non_eligible_dividend = BigDecimal::from_str(&non_eligible_dividend)?;
            let foreign_business_income = BigDecimal::from_str(&foreign_business_income)?;
            let foreign_non_business_income = BigDecimal::from_str(&foreign_non_business_income)?;
            let other_income = BigDecimal::from_str(&other_income)?;
            let foreign_distribution = BigDecimal::from_str(&foreign_distribution)?;

            distribution_ref_map.entry(security_id).or_default().insert(
                payment_date.clone(),
                DistributionRefResult {
                    distribution_id,
                    ex_dividend_date,
                    record_date,
                    payment_date,
                    total_cash_distribution,
                    total_non_cash_distribution,
                    return_of_capital,
                    capital_gain,
                    eligible_dividend,
                    non_eligible_dividend,
                    foreign_business_income,
                    foreign_non_business_income,
                    other_income,
                    foreign_distribution,
                },
            );
        }

        Ok(distribution_ref_map)
    }
}
