INSERT INTO PendingTransaction (transaction_id, note)
SELECT ?,
    ?
WHERE EXISTS (
        -- insert only if there's a FinancialEntry
        SELECT 1
        FROM FinancialEntry f
        WHERE f.transaction_id = transaction_id
    );