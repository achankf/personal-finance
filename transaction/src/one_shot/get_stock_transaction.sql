SELECT first_name || ' ' || last_name AS "name!:String",
    account_type,
    ticker,
    date AS "date:DateTime<Local>",
    unit,
    COALESCE(debit, 0) AS "debit!:String",
    COALESCE(credit, 0) AS "credit!:String"
FROM FinancialEntry
    INNER JOIN StockAccountEntry USING (account_id)
    INNER JOIN StockAccountHolder USING (stock_account_holder_id)
    INNER JOIN AccountSubtype USING (account_subtype_id)
    INNER JOIN SECURITY USING (security_id)
    INNER JOIN AccountType USING (account_type_id)
    INNER JOIN Person USING (person_id)
WHERE account_subtype = 'STOCK'
    AND ticker = ?
ORDER BY date DESC
LIMIT ?