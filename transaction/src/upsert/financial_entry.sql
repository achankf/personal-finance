INSERT INTO FinancialEntry (
        transaction_id,
        item_id,
        date,
        account_id,
        forex_rate,
        unit,
        debit,
        credit,
        description
    )
VALUES (
        ?,
        ?,
        ?,
        (
            SELECT account_id
            FROM Account
            WHERE account_key = ?
        ),
        ?,
        ?,
        ?,
        ?,
        ?
    ) ON CONFLICT (transaction_id, item_id) DO
UPDATE
SET date = excluded.date,
    account_id = excluded.account_id,
    forex_rate = excluded.forex_rate,
    unit = excluded.unit,
    debit = excluded.debit,
    credit = excluded.credit,
    description = excluded.description
WHERE date <> excluded.date
    OR account_id <> excluded.account_id
    OR forex_rate <> excluded.forex_rate
    OR unit <> excluded.unit
    OR debit <> excluded.debit
    OR credit <> excluded.credit
    OR description <> excluded.description;