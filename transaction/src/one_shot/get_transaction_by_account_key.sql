SELECT transaction_id,
    item_id,
    date AS "date!:DateTime<Local>",
    unit,
    COALESCE(debit, 0) AS "debit!:String",
    COALESCE(credit, 0) AS "credit!:String",
    COALESCE(forex_rate, 1) AS "forex_rate!:String",
    description,
    CASE
        WHEN account_kind IN ('ASSET', 'EXPENSE') THEN 1
        ELSE -1
    END AS "sign_multiplier!:String"
FROM FinancialEntry
    INNER JOIN Account USING (account_id)
    INNER JOIN AccountSubtype USING(account_subtype_id)
    INNER JOIN AccountKind USING (account_kind_id)
WHERE account_key = ?
    AND date < ?
ORDER BY "date!:DateTime<Local>"