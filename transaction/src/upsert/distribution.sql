INSERT
    OR REPLACE INTO Distribution (
        security_id,
        distribution_id,
        ex_dividend_date,
        record_date,
        payment_date,
        total_cash_distribution,
        total_non_cash_distribution,
        distribution_tax_breakdown_type_id,
        capital_gain,
        capital_gain_pre_20240624,
        capital_gain_post_20240624,
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
        (
            SELECT distribution_tax_breakdown_type_id
            FROM DistributionTaxBreakdownType
            WHERE distribution_tax_breakdown_type = ?
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
        ?
    )