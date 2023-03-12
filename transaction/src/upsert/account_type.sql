INSERT INTO AccountType (account_type)
VALUES (?) ON CONFLICT(account_type) DO NOTHING