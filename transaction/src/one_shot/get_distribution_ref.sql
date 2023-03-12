SELECT security_id,
    distribution_id,
    ex_dividend_date AS "ex_dividend_date!:DateTime<Local>",
    record_date AS "record_date!:DateTime<Local>",
    payment_date AS "payment_date!:DateTime<Local>",
    total_cash_distribution,
    -- decreases acb
    total_non_cash_distribution,
    -- increases acb
    return_of_capital,
    -- T3-related fields
    distribution_tax_breakdown_type_id,
    capital_gain,
    capital_gain_pre_20240624,
    capital_gain_post_20240624,
    eligible_dividend,
    non_eligible_dividend,
    foreign_business_income,
    foreign_non_business_income,
    other_income,
    foreign_distribution,
    foreign_business_income_tax_paid,
    foreign_non_business_income_tax_paid
FROM Distribution