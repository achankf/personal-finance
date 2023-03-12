use std::ops::Add;

use crate::GlBreakDown;
use num_traits::Zero;

impl GlBreakDown {
    pub fn has_activity(&self) -> bool {
        !self.disposition_capital_gl.is_zero()
            || !self.total_cash_distribution.is_zero()
            || !self.total_non_cash_distribution.is_zero()
            || !self.return_of_capital.is_zero()
            || !self.distribution_capital_gain.is_zero()
            || !self.eligible_dividend.is_zero()
            || !self.non_eligible_dividend.is_zero()
            || !self.foreign_business_income.is_zero()
            || !self.foreign_non_business_income.is_zero()
            || !self.other_income.is_zero()
            || !self.foreign_business_income_tax_paid.is_zero()
            || !self.foreign_non_business_income_tax_paid.is_zero()
            || !self.foreign_distribution.is_zero()
    }
}

impl Add for GlBreakDown {
    type Output = GlBreakDown;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            disposition_capital_gl: self.disposition_capital_gl + rhs.disposition_capital_gl,
            total_cash_distribution: self.total_cash_distribution + rhs.total_cash_distribution,
            total_non_cash_distribution: self.total_non_cash_distribution
                + rhs.total_non_cash_distribution,
            return_of_capital: self.return_of_capital + rhs.return_of_capital,
            distribution_capital_gain: self.distribution_capital_gain
                + rhs.distribution_capital_gain,
            eligible_dividend: self.eligible_dividend + rhs.eligible_dividend,
            non_eligible_dividend: self.non_eligible_dividend + rhs.non_eligible_dividend,
            foreign_business_income: self.foreign_business_income + rhs.foreign_business_income,
            foreign_non_business_income: self.foreign_non_business_income
                + rhs.foreign_non_business_income,
            other_income: self.other_income + rhs.other_income,
            foreign_business_income_tax_paid: self.foreign_business_income_tax_paid
                + rhs.foreign_business_income_tax_paid,
            foreign_non_business_income_tax_paid: self.foreign_non_business_income_tax_paid
                + rhs.foreign_non_business_income_tax_paid,
            foreign_distribution: self.foreign_distribution + rhs.foreign_distribution,
        }
    }
}
