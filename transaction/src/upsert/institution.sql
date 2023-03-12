INSERT INTO Institution (institution_name)
VALUES (?) ON CONFLICT (institution_name) DO NOTHING