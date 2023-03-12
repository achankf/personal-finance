INSERT INTO SECURITY (
        exchange_id,
        currency_id,
        ticker,
        security_name,
        price
    )
VALUES (
        (
            SELECT exchange_id
            FROM Exchange
            WHERE exchange_key = ?
        ),
        (
            SELECT currency_id
            FROM Currency
            WHERE currency = ?
        ),
        ?,
        ?,
        ?
    ) ON CONFLICT(ticker) DO
UPDATE
SET exchange_id = excluded.exchange_id,
    security_name = excluded.security_name,
    currency_id = excluded.currency_id,
    price = excluded.price
WHERE exchange_id <> excluded.exchange_id
    OR security_name <> excluded.security_name
    OR currency_id <> excluded.currency_id
    OR price <> excluded.price