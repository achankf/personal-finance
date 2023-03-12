SELECT first_name || ' ' || last_name AS name,
    account_type,
    ticker,
    date AS "date:DateTime<Local>",
    CASE
        WHEN debit THEN unit
        ELSE - unit
    END AS unit,
    debit,
    credit
FROM FinancialEntry
    INNER JOIN StockAccountEntry USING (account_id)
    INNER JOIN StockAccountHolder USING (stock_account_holder_id)
    INNER JOIN AccountSubtype USING (account_subtype_id)
    INNER JOIN SECURITY USING (security_id)
    INNER JOIN AccountType USING (account_type_id)
    INNER JOIN Person USING (person_id)
WHERE account_subtype = 'STOCK'
    AND ticker = ?
LIMIT ?