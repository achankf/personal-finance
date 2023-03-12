INSERT INTO CreditCardProduct (
        account_type_id,
        institution_id,
        currency_id,
        annual_fee,
        credit_limit,
        account_name
    )
VALUES (
        (
            SELECT account_type_id
            FROM AccountType
            WHERE account_type = ?
        ),
        (
            SELECT institution_id
            FROM Institution
            WHERE institution_name = ?
        ),
        (
            SELECT currency_id
            FROM Currency
            WHERE currency = ?
        ),
        ?,
        ?,
        ?
    ) ON CONFLICT(account_type_id) DO
UPDATE
SET institution_id = excluded.institution_id,
    annual_fee = excluded.annual_fee,
    credit_limit = excluded.credit_limit