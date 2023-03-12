use common::excel_date_optional_time_format;
use common::is_numeric;
use common::Id;
use serde::Deserialize;
use serde_trim::string_trim;

use db::Query;

#[derive(Debug, Deserialize)]
pub struct Distribution {
    pub distribution_id: i64,
    #[serde(deserialize_with = "string_trim")]
    pub ticker: String,
    #[serde(deserialize_with = "excel_date_optional_time_format")]
    pub ex_dividend_date: i64,
    #[serde(deserialize_with = "excel_date_optional_time_format")]
    pub record_date: i64,
    #[serde(deserialize_with = "excel_date_optional_time_format")]
    pub payment_date: i64,
    #[serde(deserialize_with = "is_numeric")]
    pub total_cash_distribution: String,
    #[serde(deserialize_with = "is_numeric")]
    pub total_non_cash_distribution: String,
    #[serde(deserialize_with = "string_trim")]
    pub distribution_tax_breakdown_type: String,
    #[serde(deserialize_with = "is_numeric")]
    pub capital_gain: String,
    #[serde(deserialize_with = "is_numeric")]
    pub capital_gain_pre_20240624: String,
    #[serde(deserialize_with = "is_numeric")]
    pub capital_gain_post_20240624: String,
    #[serde(deserialize_with = "is_numeric")]
    pub eligible_dividend: String,
    #[serde(deserialize_with = "is_numeric")]
    pub foreign_non_business_income: String,
    #[serde(deserialize_with = "is_numeric")]
    pub other_income: String,
    #[serde(deserialize_with = "is_numeric")]
    pub return_of_capital: String,
    #[serde(deserialize_with = "is_numeric")]
    pub foreign_non_business_income_tax_paid: String,
    #[serde(deserialize_with = "is_numeric")]
    pub non_eligible_dividend: String,
    #[serde(deserialize_with = "is_numeric")]
    pub foreign_business_income: String,
    #[serde(deserialize_with = "is_numeric")]
    pub non_reportable_distribution: String,
    #[serde(deserialize_with = "is_numeric")]
    pub capital_gains_eligible_for_deduction: String,
    #[serde(deserialize_with = "is_numeric")]
    pub foreign_business_income_tax_paid: String,
    #[serde(deserialize_with = "is_numeric")]
    pub foreign_distribution: String,
}

impl Id for Distribution {
    type IdType = (String, i64);

    fn id(&self) -> Self::IdType {
        (self.ticker.clone(), self.distribution_id)
    }
}

impl Query for Distribution {
    fn query(&self) -> db::SqlQuery {
        sqlx::query_file!(
            "src/upsert/distribution.sql",
            self.ticker,
            self.distribution_id,
            self.ex_dividend_date,
            self.record_date,
            self.payment_date,
            self.total_cash_distribution,
            self.total_non_cash_distribution,
            self.distribution_tax_breakdown_type,
            self.capital_gain,
            self.capital_gain_pre_20240624,
            self.capital_gain_post_20240624,
            self.eligible_dividend,
            self.non_eligible_dividend,
            self.foreign_business_income,
            self.foreign_non_business_income,
            self.other_income,
            self.return_of_capital,
            self.non_reportable_distribution,
            self.capital_gains_eligible_for_deduction,
            self.foreign_business_income_tax_paid,
            self.foreign_non_business_income_tax_paid,
            self.foreign_distribution
        )
    }
}
