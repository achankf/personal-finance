INSERT
    OR IGNORE INTO Account (
        account_key,
        account_type_id,
        account_subtype_id,
        account_name
    ) WITH BaseRecord (account_key, account_type_id) AS (
        SELECT ?,
            (
                SELECT account_type_id
                FROM AccountType
                WHERE account_type = ?
            )
    ),
    WithAccountName AS (
        SELECT *,
            (
                SELECT account_name
                FROM BaseRecord
                    INNER JOIN CreditCardProduct USING(account_type_id)
            ) AS account_name
        FROM BaseRecord
    )
SELECT account_key || '-' || account_subtype,
    account_type_id,
    account_subtype_id,
    account_name || ', ' || account_subtype
FROM WithAccountName
    CROSS JOIN CreditCardEntryType
    INNER JOIN AccountSubtype USING (account_subtype_id) ON CONFLICT(account_key) DO
UPDATE
SET account_type_id = excluded.account_type_id,
    account_subtype_id = excluded.account_subtype_id,
    account_name = excluded.account_name
WHERE account_type_id <> excluded.account_type_id
    OR account_subtype_id <> excluded.account_subtype_id
    OR account_name <> excluded.account_name