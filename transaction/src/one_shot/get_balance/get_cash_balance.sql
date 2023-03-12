SELECT person_id,
    account_id,
    unit,
    COALESCE(debit, 0) AS "debit!:String",
    COALESCE(credit, 0) AS "credit!:String"
FROM FinancialEntry
    INNER JOIN Account USING (account_id)
    INNER JOIN AccountSubtype USING (account_subtype_id)
    INNER JOIN AccountType USING (account_type_id)
    INNER JOIN AccountKind USING (account_kind_id)
    LEFT JOIN OwnedAccount USING (account_id, account_type_id, account_subtype_id)
WHERE account_kind = 'ASSET'
    AND account_subtype IN ('CASH', 'PRINCIPAL')
    AND date BETWEEN ? AND ?