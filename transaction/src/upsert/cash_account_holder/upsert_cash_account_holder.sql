INSERT INTO CashAccountHolder (
        person_id,
        account_type_id,
        emergency_target,
        is_closed
    )
VALUES (
        (
            SELECT person_id
            FROM Person
            WHERE person_key = ?
        ),
        (
            SELECT account_type_id
            FROM AccountType
            WHERE account_type = ?
        ),
        ?,
        ?
    ) ON CONFLICT(person_id, account_type_id) DO
UPDATE
SET emergency_target = excluded.emergency_target,
    is_closed = excluded.is_closed
WHERE emergency_target <> excluded.emergency_target
    OR is_closed <> excluded.is_closed