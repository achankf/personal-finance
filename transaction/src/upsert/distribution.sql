INSERT INTO Distribution (
        security_id,
        distribution_id,
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
        foreign_non_business_income_tax_paid,
        foreign_distribution
    )
VALUES (
        (
            SELECT security_id
            FROM SECURITY
            WHERE ticker = ?
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
        ?,
        ?,
        ?
    ) ON CONFLICT(security_id, distribution_id) DO
UPDATE
SET ex_dividend_date = excluded.ex_dividend_date,
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
    foreign_non_business_income_tax_paid = excluded.foreign_non_business_income_tax_paid,
    foreign_distribution = excluded.foreign_distribution
WHERE ex_dividend_date <> excluded.ex_dividend_date
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
    OR foreign_distribution <> excluded.foreign_distribution