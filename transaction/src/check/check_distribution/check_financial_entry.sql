-- Purpose: checks if there is a financial entry for all given distribution event.
--          In particular, there should be a financial entry when accounts held units before the ex-dividend date.
SELECT account_key,
    ticker,
    ex_dividend_date AS "ex_dividend_date!:DateTime<Local>",
    record_date AS "record_date!:DateTime<Local>",
    payment_date AS "payment_date!:DateTime<Local>",
    transaction_id,
    held_unit AS "held_unit!:f64"
FROM StockAccountHolder
    INNER JOIN StockAccountEntry USING (stock_account_holder_id)
    INNER JOIN Account USING (account_id)
    INNER JOIN AccountSubtype USING (account_subtype_id)
    INNER JOIN SECURITY USING (security_id)
    INNER JOIN (
        SELECT security_id,
            distribution_id,
            ex_dividend_date,
            record_date,
            payment_date,
            total_cash_distribution
        FROM Distribution
        WHERE total_cash_distribution > 0
    ) USING (security_id)
    LEFT JOIN FinancialEntry ON StockAccountEntry.account_id = FinancialEntry.account_id
    AND record_date = FinancialEntry.date
    LEFT JOIN (
        -- calculate how many stock units for each CASH (i.e. income and not non-cash) distribution
        SELECT stock_account_holder_id,
            record_date,
            SUM(
                CASE
                    WHEN debit IS NOT NULL THEN unit
                    ELSE - unit
                END
            ) AS held_unit
        FROM StockAccountHolder
            INNER JOIN StockAccountEntry USING (stock_account_holder_id)
            INNER JOIN AccountSubtype USING (account_subtype_id)
            INNER JOIN (
                SELECT security_id,
                    ex_dividend_date,
                    record_date
                FROM Distribution
                WHERE total_cash_distribution > 0
            ) CashDistribution USING (security_id)
            INNER JOIN SECURITY USING (security_id)
            INNER JOIN FinancialEntry ON FinancialEntry.account_id = StockAccountEntry.account_id
            AND FinancialEntry.date < CashDistribution.ex_dividend_date
        WHERE account_subtype = 'STOCK'
        GROUP BY stock_account_holder_id,
            record_date
    ) USING (stock_account_holder_id, record_date)
WHERE account_subtype = 'DISTRIBUTION'
    AND (
        -- i.e. at that particular I held the stock but haven't recorded a distribution
        (
            transaction_id IS NULL
            AND held_unit > 0
        ) -- i.e. I didn't own that stock at that moment but I entered a distribution entry (i.e. probably wrong date)
        OR (
            transaction_id IS NOT NULL
            AND held_unit = 0
        )
    )
    AND ticker NOT IN ('TDB2460', 'TDB888')
    AND ROUND(total_cash_distribution * held_unit, 2) > 0 -- Only care about distribution amount > 0 (i.e. actually having a payout)
ORDER BY person_id,
    security_id,
    record_date;