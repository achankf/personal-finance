INSERT INTO AccountSubtype (
        account_kind_id,
        account_subtype
    )
VALUES (
        (
            SELECT account_kind_id
            FROM AccountKind
            WHERE account_kind = ?
        ),
        ?
    ) ON CONFLICT(account_subtype) DO
UPDATE
SET account_kind_id = excluded.account_kind_id
WHERE excluded.account_kind_id <> account_kind_id;