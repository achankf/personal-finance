INSERT INTO StoreCashbackMapping (
        store_id,
        account_type_id,
        cashback_category_name_id
    )
VALUES (
        (
            SELECT store_id
            FROM Store
            WHERE store_key = ?
        ),
        (
            SELECT account_type_id
            FROM AccountType
            WHERE account_type = ?
        ),
        (
            SELECT cashback_category_name_id
            FROM CashbackCategoryName
            WHERE cashback_category_name = ?
        )
    ) ON CONFLICT(store_id, account_type_id) DO
UPDATE
SET cashback_category_name_id = excluded.cashback_category_name_id
WHERE cashback_category_name_id <> excluded.cashback_category_name_id