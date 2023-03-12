SELECT account_type_id,
    account_name
FROM AccountType
    INNER JOIN StockAccount USING (account_type_id)
    INNER JOIN CashAccountProduct USING (account_type_id)