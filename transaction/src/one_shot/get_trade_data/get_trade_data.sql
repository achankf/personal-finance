SELECT
    person_id,
    security_id,
    date AS "date!:DateTime<Local>",
    forex_rate,
    unit,
    debit,
    credit
FROM
    FinancialEntry
    INNER JOIN StockAccountEntry USING (account_id)
    INNER JOIN StockAccountHolder USING (stock_account_holder_id)
    INNER JOIN CashAccountProduct USING (account_type_id)
    INNER JOIN Account USING (account_id)
    INNER JOIN AccountSubtype USING (account_subtype_id)
    INNER JOIN SECURITY USING (security_id)
    INNER JOIN TaxShelterType USING (tax_shelter_type_id)
WHERE
    account_subtype = 'STOCK'
    AND tax_shelter_type = 'NON-REGISTERED'
ORDER BY
    person_id,
    security_id,
    date