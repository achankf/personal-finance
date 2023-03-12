SELECT transaction_id,
    COALESCE(forex_rate, 1) AS "forex_rate!:String",
    unit,
    COALESCE(debit, 0) AS "debit!:String",
    COALESCE(credit, 0) AS "credit!:String"
FROM FinancialEntry
ORDER BY transaction_id