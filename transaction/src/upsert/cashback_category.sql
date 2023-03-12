INSERT INTO CashbackCategory (
        account_type_id,
        cashback_category_name_id,
        cashback_rate
    )
VALUES (
        (
            SELECT account_type_id
            FROM AccountType
            WHERE account_type = ?
        ),
        (
            SELECT cashback_category_name_id
            FROM CashbackCategoryName
            WHERE cashback_category_name = ?
        ),
        ?
    ) ON CONFLICT(account_type_id, cashback_category_name_id) DO
UPDATE
SET cashback_rate = excluded.cashback_rate
WHERE cashback_rate <> excluded.cashback_rate