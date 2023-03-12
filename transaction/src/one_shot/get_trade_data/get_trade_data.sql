SELECT person_id,
    security_id,
    date AS "date!:DateTime<Local>",
    COALESCE(forex_rate, 1) AS "forex_rate!:String",
    unit,
    COALESCE(debit, 0) AS "debit!:String",
    COALESCE(credit, 0) AS "credit!:String",
    CASE
        WHEN debit IS NOT NULL THEN TRUE
        ELSE FALSE
    END AS "is_debit_record!:bool"
FROM FinancialEntry
    INNER JOIN StockAccountEntry USING (account_id)
    INNER JOIN StockAccountHolder USING (stock_account_holder_id)
    INNER JOIN CashAccountProduct USING (account_type_id)
    INNER JOIN Account USING (account_id)
    INNER JOIN AccountSubtype USING (account_subtype_id)
    INNER JOIN SECURITY USING (security_id)
    INNER JOIN TaxShelterType USING (tax_shelter_type_id)
WHERE account_subtype = 'STOCK'
    AND tax_shelter_type = 'NON-REGISTERED'
ORDER BY person_id,
    security_id,
    date