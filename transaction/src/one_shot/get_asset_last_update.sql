SELECT first_name AS "first_name!:String",
    last_name AS "last_name!:String",
    account_name AS "account_name!:String",
    max(date) AS "last_update!:DateTime<Local>"
FROM CashAccountHolder
    INNER JOIN CashAccountEntry USING (cash_account_holder_id)
    INNER JOIN CashAccountProduct USING (account_type_id)
    INNER JOIN AccountType USING (account_type_id)
    INNER JOIN AccountSubtype USING (account_subtype_id)
    INNER JOIN FinancialEntry USING (account_id)
    INNER JOIN Person USING (person_id)
WHERE NOT is_closed
    AND account_type NOT IN ('FIAT-CAD', 'PRESTO')
    AND account_subtype = 'CASH'
GROUP BY person_id,
    account_id