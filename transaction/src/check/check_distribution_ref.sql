WITH UnusedDistributionData (security_id, payment_date) AS (
    SELECT security_id,
        payment_date
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
    total_cash_distribution + total_non_cash_distribution AS "total_distribution!:f64",
    (
        capital_gain + eligible_dividend + foreign_non_business_income + other_income + return_of_capital + non_eligible_dividend + foreign_business_income + non_reportable_distribution + capital_gains_eligible_for_deduction + foreign_distribution
    ) AS "taxable_gain!:f64",
    (
        foreign_non_business_income_tax_paid + foreign_business_income_tax_paid
    ) AS "foreign_tax!:f64"
FROM Distribution d
    INNER JOIN SECURITY USING (security_id)
    FULL OUTER JOIN (
        SELECT security_id,
            payment_date,
            1 AS unused_flag
        FROM UnusedDistributionData
    ) USING (security_id, payment_date)
WHERE -- ignore distributions that haven't been paid
    d.payment_date < strftime('%s', 'now')
    AND ROUND(
        "total_distribution!:f64" - ("taxable_gain!:f64" - "foreign_tax!:f64"),
        2
    ) <> 0
ORDER BY "ticker!:String",
    "record_date!:DateTime<Local>"