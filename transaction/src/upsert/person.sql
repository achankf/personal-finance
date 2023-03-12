INSERT INTO Person (person_key, first_name, last_name)
VALUES (?, ?, ?) ON CONFLICT(person_key) DO
UPDATE
SET first_name = excluded.first_name,
    last_name = excluded.last_name
WHERE first_name <> excluded.first_name
    OR last_name <> excluded.last_name