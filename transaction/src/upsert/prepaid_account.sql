INSERT
    OR IGNORE INTO PrepaidAccount (account_type_id)
VALUES (
        (
            SELECT account_type_id
            FROM AccountType
            WHERE account_type = ?
        )
    )