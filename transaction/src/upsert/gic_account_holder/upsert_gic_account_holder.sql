INSERT
    OR IGNORE INTO GicAccountHolder (
        person_id,
        account_type_id,
        issue_date,
        maturity_date,
        apr
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
        ?,
        ?
    )