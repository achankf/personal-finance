SELECT transaction_id,
    account_key,
    date AS "date!:DateTime<Local>",
    ROUND(
        unit * (COALESCE(debit, 0.0) - COALESCE(credit, 0.0)),
        2
    ) AS "balance!:f64",
    description
FROM CashbackTransaction
    INNER JOIN Account USING (account_id)
WHERE transaction_id NOT IN (
        SELECT transaction_id
        FROM TransactionStore
    )
ORDER BY account_key,
    date