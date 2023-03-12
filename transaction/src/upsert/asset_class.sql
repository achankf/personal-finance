INSERT INTO AssetClass (
        person_id,
        parent_id,
        asset_class_name_id,
        weight
    )
VALUES (
        (
            SELECT person_id
            FROM Person
            WHERE person_key = ?
        ),
        (
            SELECT asset_class_id
            FROM AssetClass
            WHERE asset_class_name_id = (
                    SELECT asset_class_name_id
                    FROM AssetClassName
                    WHERE asset_class_name = ?
                )
                AND person_id = (
                    SELECT person_id
                    FROM Person
                    WHERE person_key = ?
                )
        ),
        (
            SELECT asset_class_name_id
            FROM AssetClassName
            WHERE asset_class_name = ?
        ),
        ?
    ) ON CONFLICT(person_id, asset_class_name_id) DO
UPDATE
SET weight = excluded.weight,
    parent_id = excluded.parent_id
WHERE weight <> excluded.weight
    OR parent_id <> excluded.parent_id