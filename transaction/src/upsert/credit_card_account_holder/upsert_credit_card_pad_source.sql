INSERT INTO CreditCardPadSource (credit_card_holder_id, cash_account_holder_id)
VALUES (
        ?,
        (
            SELECT cash_account_holder_id
            FROM Account
                INNER JOIN CashAccountEntry USING (account_id)
            WHERE account_key = ?
        )
    ) ON CONFLICT (credit_card_holder_id) DO
UPDATE
SET cash_account_holder_id = excluded.cash_account_holder_id
WHERE cash_account_holder_id <> excluded.cash_account_holder_id