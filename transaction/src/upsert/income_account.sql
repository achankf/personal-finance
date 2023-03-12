INSERT INTO IncomeAccount (account_type_id, currency_id, account_name)
VALUES (
        (
            SELECT account_type_id
            FROM AccountType
            WHERE account_type = ?
        ),
        (
            SELECT currency_id
            FROM Currency
            WHERE currency = ?
        ),
        ?
    ) ON CONFLICT(account_type_id) DO
UPDATE
SET currency_id = excluded.currency_id,
    account_name = excluded.account_name
WHERE currency_id <> excluded.currency_id
    OR account_name <> excluded.account_name