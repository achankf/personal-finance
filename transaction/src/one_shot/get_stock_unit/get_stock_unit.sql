SELECT person_id,
    account_type_id,
    security_id,
    unit,
    -- multiplier for adjusting unit disposition, because calculations will only be done using the BigDecimal library
    CASE
        WHEN DEBIT IS NOT NULL THEN 1
        ELSE -1
    END AS "sign_multiplier!:i32"
FROM FinancialEntry
    INNER JOIN StockAccountEntry USING (account_id)
    INNER JOIN StockAccountHolder USING (stock_account_holder_id)
    INNER JOIN AccountSubtype USING (account_subtype_id)
    INNER JOIN AccountType USING (account_type_id)
    INNER JOIN SECURITY USING (security_id)
WHERE account_subtype = 'STOCK'
    AND date < ?