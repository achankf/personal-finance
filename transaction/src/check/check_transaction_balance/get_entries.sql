SELECT transaction_id,
    COALESCE(forex_rate, '1') AS forex_rate,
    unit,
    debit,
    credit
FROM FinancialEntry
ORDER BY transaction_id