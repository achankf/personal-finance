INSERT INTO CreditCardHolder (
        person_id,
        account_type_id,
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
        ?
    ) ON CONFLICT(person_id, account_type_id) DO
UPDATE
SET is_closed = excluded.is_closed
WHERE is_closed <> excluded.is_closed