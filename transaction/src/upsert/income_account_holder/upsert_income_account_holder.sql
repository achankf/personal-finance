INSERT
    OR IGNORE INTO IncomeAccountHolder (
        person_id,
        account_type_id
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
        )
    )