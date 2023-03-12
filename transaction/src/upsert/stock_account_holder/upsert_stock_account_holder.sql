INSERT
    OR IGNORE INTO StockAccountHolder (
        person_id,
        account_type_id,
        security_id
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
        (
            SELECT security_id
            FROM SECURITY
            WHERE ticker = ?
        )
    )