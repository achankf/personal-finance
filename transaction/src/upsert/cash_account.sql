INSERT INTO CashAccountProduct (
        account_type_id,
        institution_id,
        tax_shelter_type_id,
        currency_id,
        min_balance_waiver,
        inactive_fee_months,
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
            SELECT tax_shelter_type_id
            FROM TaxShelterType
            WHERE tax_shelter_type = ?
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
    tax_shelter_type_id = excluded.tax_shelter_type_id,
    currency_id = excluded.currency_id,
    min_balance_waiver = excluded.min_balance_waiver,
    inactive_fee_months = excluded.inactive_fee_months,
    account_name = excluded.account_name
WHERE institution_id <> excluded.institution_id
    OR tax_shelter_type_id <> excluded.tax_shelter_type_id
    OR currency_id <> excluded.currency_id
    OR min_balance_waiver <> excluded.min_balance_waiver
    OR inactive_fee_months <> excluded.inactive_fee_months
    OR account_name <> excluded.account_name