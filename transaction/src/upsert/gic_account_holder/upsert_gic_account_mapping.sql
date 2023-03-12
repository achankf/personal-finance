INSERT INTO GicEntry (
        gic_account_holder_id,
        account_subtype_id,
        account_id
    )
SELECT ?,
    account_subtype_id,
    account_id
FROM Account
WHERE account_key LIKE ? ON CONFLICT(gic_account_holder_id, account_subtype_id) DO
UPDATE
SET account_id = excluded.account_id
WHERE account_id <> excluded.account_id