INSERT INTO Currency (
        currency,
        currency_name,
        currency_symbol,
        market_exchange_rate
    )
VALUES (?, ?, ?, ?) ON CONFLICT(currency) DO
UPDATE
SET currency_name = excluded.currency_name,
    currency_symbol = excluded.currency_symbol,
    market_exchange_rate = excluded.market_exchange_rate
WHERE currency_name <> excluded.currency_name
    OR currency_symbol <> excluded.currency_symbol
    OR market_exchange_rate <> excluded.market_exchange_rate