INSERT INTO AssetAllocation (
        asset_class_name_id,
        security_id,
        weight
    )
VALUES (
        (
            SELECT asset_class_name_id
            FROM AssetClassName
            WHERE asset_class_name = ?
        ),
        (
            SELECT security_id
            FROM SECURITY
            WHERE ticker = ?
        ),
        ?
    ) ON CONFLICT(
        asset_class_name_id,
        security_id
    ) DO
UPDATE
SET weight = excluded.weight
WHERE weight <> excluded.weight