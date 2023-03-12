-- Purpose: check if there are financial entries that don't match distribution records, like if using invalid record date or
--          if the credit value (i.e. the amount of distribution per unit) does match.
SELECT transaction_id,
    ticker
FROM StockAccountEntry
    INNER JOIN AccountSubtype USING (account_subtype_id)
    INNER JOIN StockAccountHolder s USING (stock_account_holder_id)
    INNER JOIN SECURITY USING (security_id)
    INNER JOIN FinancialEntry f USING (account_id)
WHERE account_subtype = 'DISTRIBUTION'
    AND transaction_id NOT IN (
        SELECT transaction_id
        FROM StockAccountEntry
            INNER JOIN AccountSubtype USING (account_subtype_id)
            INNER JOIN StockAccountHolder s USING (stock_account_holder_id)
            INNER JOIN FinancialEntry f USING (account_id)
            INNER JOIN Distribution d ON d.security_id = s.security_id
            AND d.record_date = f.date
            AND d.total_cash_distribution = f.credit
        WHERE account_subtype = 'DISTRIBUTION'
    )
    AND ticker NOT IN ('TDB2460', 'TDB888')
ORDER BY date