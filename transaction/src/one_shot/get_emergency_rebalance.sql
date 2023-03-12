SELECT first_name,
    last_name,
    account_name,
    currency AS "currency!:String",
    currency_symbol AS "currency_symbol!:String",
    unallocated_fund AS "unallocated_fund!:f64",
    injection_needed AS "injection_needed!:f64"
FROM CashView
    INNER JOIN CashAccountHolder USING (cash_account_holder_id)
    INNER JOIN CashAccountProduct USING (account_type_id)
    INNER JOIN Person USING (person_id)
    INNER JOIN Currency USING (currency_id)
    INNER JOIN AccountType USING (account_type_id)
WHERE NOT is_closed
    AND (
        unallocated_fund <> 0
        OR injection_needed <> 0
    )
    AND account_type <> 'PRESTO' -- no need to rebalance Presto cards
ORDER BY first_name,
    last_name,
    account_name