WITH UnusedDistributionData (security_id, record_date) AS (
    SELECT security_id,
        record_date
    FROM Distribution
    EXCEPT
    SELECT security_id,
        date
    FROM StockAccountEntry
        INNER JOIN AccountSubtype USING (account_subtype_id)
        INNER JOIN StockAccountHolder s USING (stock_account_holder_id)
        INNER JOIN SECURITY USING (security_id)
        INNER JOIN FinancialEntry f USING (account_id)
    WHERE account_subtype = 'DISTRIBUTION'
)
SELECT ticker AS "ticker!:String",
    record_date AS "record_date!:DateTime<Local>",
    CASE
        WHEN unused_flag THEN 1
        ELSE 0
    END AS "unused_flag:bool",
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
    foreign_business_income_tax_paid
FROM Distribution d
    INNER JOIN SECURITY USING (security_id)
    FULL OUTER JOIN (
        SELECT security_id,
            record_date,
            1 AS unused_flag
        FROM UnusedDistributionData
    ) USING (security_id, record_date)
WHERE -- ignore distributions that haven't been paid
    d.payment_date < strftime('%s', 'now')
ORDER BY "ticker!:String",
    "record_date!:DateTime<Local>"