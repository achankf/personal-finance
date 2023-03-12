use common::excel_date_optional_time_format;
use common::is_numeric;
use common::Id;
use serde::Deserialize;
use serde_trim::string_trim;

use db::Query;

#[derive(Debug, Deserialize)]
pub struct Distribution {
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
    #[serde(deserialize_with = "is_numeric")]
    pub capital_gain: String,
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
}

impl Id for Distribution {
    type IdType = (String, i64);

    fn id(&self) -> Self::IdType {
        (self.ticker.clone(), self.record_date.clone())
    }
}

impl Query for Distribution {
    fn query(&self) -> db::SqlQuery {
        sqlx::query!(
            r#"
INSERT INTO
    CadDistribution (
        security_id,
        ex_dividend_date,
        record_date,
        payment_date,
        total_cash_distribution,
        total_non_cash_distribution,
        capital_gain,
        eligible_dividend,
        non_eligible_dividend,
        foreign_business_income,
        foreign_non_business_income,
        other_income,
        return_of_capital,
        non_reportable_distribution,
        capital_gains_eligible_for_deduction,
        foreign_business_income_tax_paid,
        foreign_non_business_income_tax_paid
    )
VALUES
    (
        (
            SELECT
                security_id
            FROM
                SECURITY
            WHERE
                ticker = ?
        ),
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?
    ) ON CONFLICT(security_id, record_date) DO
UPDATE
SET
    ex_dividend_date = excluded.ex_dividend_date,
    record_date = excluded.record_date,
    payment_date = excluded.payment_date,
    total_cash_distribution = excluded.total_cash_distribution,
    total_non_cash_distribution = excluded.total_non_cash_distribution,
    capital_gain = excluded.capital_gain,
    eligible_dividend = excluded.eligible_dividend,
    non_eligible_dividend = excluded.non_eligible_dividend,
    foreign_business_income = excluded.foreign_business_income,
    foreign_non_business_income = excluded.foreign_non_business_income,
    other_income = excluded.other_income,
    return_of_capital = excluded.return_of_capital,
    non_reportable_distribution = excluded.non_reportable_distribution,
    capital_gains_eligible_for_deduction = excluded.capital_gains_eligible_for_deduction,
    foreign_business_income_tax_paid = excluded.foreign_business_income_tax_paid,
    foreign_non_business_income_tax_paid = excluded.foreign_non_business_income_tax_paid
WHERE
    ex_dividend_date <> excluded.ex_dividend_date
    OR payment_date <> excluded.payment_date
    OR total_cash_distribution <> excluded.total_cash_distribution
    OR total_non_cash_distribution <> excluded.total_non_cash_distribution
    OR capital_gain <> excluded.capital_gain
    OR eligible_dividend <> excluded.eligible_dividend
    OR non_eligible_dividend <> excluded.non_eligible_dividend
    OR foreign_business_income <> excluded.foreign_business_income
    OR foreign_non_business_income <> excluded.foreign_non_business_income
    OR other_income <> excluded.other_income
    OR return_of_capital <> excluded.return_of_capital
    OR non_reportable_distribution <> excluded.non_reportable_distribution
    OR capital_gains_eligible_for_deduction <> excluded.capital_gains_eligible_for_deduction
    OR foreign_business_income_tax_paid <> excluded.foreign_business_income_tax_paid
    OR foreign_non_business_income_tax_paid <> excluded.foreign_non_business_income_tax_paid
"#,
            self.ticker,
            self.ex_dividend_date,
            self.record_date,
            self.payment_date,
            self.total_cash_distribution,
            self.total_non_cash_distribution,
            self.capital_gain,
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
        )
    }
}
