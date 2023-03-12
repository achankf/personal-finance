SELECT account_type_id,
    unit,
    debit
FROM FinancialEntry
    INNER JOIN Account USING (account_id)
    INNER JOIN AccountSubtype USING (account_subtype_id)
    INNER JOIN AccountType USING (account_type_id)
    INNER JOIN AccountKind USING (account_kind_id)
    LEFT JOIN OwnedAccount USING (account_id, account_type_id, account_subtype_id)
WHERE person_id IS NULL
    AND date > ?
    AND account_kind = 'EXPENSE'