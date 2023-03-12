SELECT date AS "date!:DateTime<Local>",
    COALESCE(forex_rate, 1) AS "forex_rate!:String",
    unit,
    COALESCE(debit, 0) AS "debit!:String",
    COALESCE(credit, 0) AS "credit!:String"
FROM FinancialEntry
    INNER JOIN CashAccountEntry USING (account_id)
    INNER JOIN CashAccountHolder USING (cash_account_holder_id)
    INNER JOIN CashAccountProduct USING (account_type_id)
    INNER JOIN TaxShelterType USING (tax_shelter_type_id)
WHERE person_id = (
        SELECT person_id
        FROM Person
        WHERE person_key = ?
    )
    AND tax_shelter_type_id = (
        SELECT tax_shelter_type_id
        FROM TaxShelterType
        WHERE tax_shelter_type = ?
    )
    AND (
        -- i.e. selecting all transfers between registered and non-registered cash accounts
        tax_shelter_type <> 'NON-REGISTERED'
        AND transaction_id IN (
            SELECT transaction_id
            FROM FinancialEntry
                INNER JOIN Account USING (account_id)
                INNER JOIN CashAccountProduct USING (account_type_id)
                INNER JOIN TaxShelterType USING (tax_shelter_type_id)
            WHERE tax_shelter_type = 'NON-REGISTERED'
        )
    )
ORDER BY "year!:u32";