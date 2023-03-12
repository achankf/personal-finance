SELECT first_name,
    last_name,
    asset_class_name,
    current_rebalance_amount AS "current_rebalance_amount!:f64",
    potential_rebalance_amount AS "potential_rebalance_amount!:f64"
FROM AllocationView
    INNER JOIN Person USING (person_id)
    INNER JOIN AssetClass USING (asset_class_id)
    INNER JOIN AssetClassName USING (asset_class_name_id)
ORDER BY Person.person_id,
    current_rebalance_amount DESC