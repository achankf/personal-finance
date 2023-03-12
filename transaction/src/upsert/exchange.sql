INSERT INTO Exchange (
        exchange_key,
        exchange_name
    )
VALUES (?, ?) ON CONFLICT(exchange_key) DO
UPDATE
SET exchange_name = excluded.exchange_name
WHERE exchange_name <> excluded.exchange_name