SELECT person_id,
    account_id,
    COALESCE(forex_rate, 1) AS "forex_rate!:String",
    unit,
    COALESCE(debit, 0) AS "debit!:String",
    COALESCE(credit, 0) AS "credit!:String",
    CASE
        WHEN account_kind IN ('ASSET', 'EXPENSE') THEN 1
        ELSE 0
    END AS "is_debit_balance!:bool"
FROM FinancialEntry
    INNER JOIN Account USING (account_id)
    INNER JOIN AccountSubtype USING (account_subtype_id)
    INNER JOIN AccountType USING (account_type_id)
    INNER JOIN AccountKind USING (account_kind_id)
    LEFT JOIN OwnedAccount USING (account_id, account_type_id, account_subtype_id)
WHERE account_kind = ?
    AND date BETWEEN ? AND ?