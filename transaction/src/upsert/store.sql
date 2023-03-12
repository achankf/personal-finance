INSERT INTO Store (store_key, store_name)
VALUES (?, ?) ON CONFLICT(store_key) DO
UPDATE
SET store_name = excluded.store_name
WHERE store_name <> excluded.store_name