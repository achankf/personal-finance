WITH LatestTransaction AS (
    SELECT cash_account_holder_id,
        MAX(date) AS latest_transaction
    FROM FinancialEntry
        INNER JOIN CashAccountEntry USING (account_id)
        INNER JOIN CashAccountHolder USING (cash_account_holder_id)
        INNER JOIN CashAccountProduct USING (account_type_id)
        INNER JOIN AccountSubtype USING (account_subtype_id)
    WHERE inactive_fee_months < 120
    GROUP BY cash_account_holder_id
)
SELECT first_name,
    last_name,
    inactive_fee_months,
    latest_transaction AS "latest_transaction!:DateTime<Local>",
    account_name
FROM LatestTransaction
    INNER JOIN CashAccountHolder USING (cash_account_holder_id)
    INNER JOIN CashAccountProduct USING (account_type_id)
    INNER JOIN Person USING (person_id)
ORDER BY person_id,
    account_name;