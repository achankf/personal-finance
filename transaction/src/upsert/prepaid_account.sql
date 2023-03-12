INSERT INTO PrepaidAccount (account_type_id)
VALUES (
        (
            SELECT account_type_id
            FROM AccountType
            WHERE account_type = ?
        )
    ) ON CONFLICT (account_type_id) DO NOTHING