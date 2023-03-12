INSERT
    OR IGNORE INTO TaxShelterType (tax_shelter_type, tax_shelter_name)
VALUES (?, ?) ON CONFLICT(tax_shelter_type) DO
UPDATE
SET tax_shelter_name = excluded.tax_shelter_name
WHERE tax_shelter_name <> excluded.tax_shelter_name