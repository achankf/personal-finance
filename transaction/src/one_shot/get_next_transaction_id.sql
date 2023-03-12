SELECT COALESCE(MAX(transaction_id), 0) + 1 AS transaction_id
FROM FinancialEntry;