SELECT item_id,
    date AS "date!:DateTime<Local>",
    account_key,
    account_kind,
    COALESCE(forex_rate, 1) AS "forex_rate!:String",
    unit,
    COALESCE(debit, 0) AS "debit!:String",
    COALESCE(credit, 0) AS "credit!:String",
    description
FROM FinancialEntry
    INNER JOIN Account USING (account_id)
    INNER JOIN AccountType USING (account_type_id)
    INNER JOIN AccountSubtype USING (account_subtype_id)
    INNER JOIN AccountKind USING (account_kind_id)
WHERE transaction_id = ?
ORDER BY item_id;