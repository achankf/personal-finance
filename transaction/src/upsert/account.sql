INSERT INTO Account (
        account_key,
        account_subtype_id,
        account_type_id,
        account_name
    )
VALUES (
        ?,
        (
            SELECT account_subtype_id
            FROM AccountSubtype
            WHERE account_subtype = ?
        ),
        (
            SELECT account_type_id
            FROM AccountType
            WHERE account_type = ?
        ),
        ?
    ) ON CONFLICT(account_key) DO
UPDATE
SET account_subtype_id = excluded.account_subtype_id,
    account_type_id = excluded.account_type_id,
    account_type_id = excluded.account_type_id,
    account_name = excluded.account_name
WHERE account_subtype_id <> excluded.account_subtype_id
    OR account_type_id <> excluded.account_type_id
    OR account_type_id <> excluded.account_type_id
    OR account_name <> excluded.account_name