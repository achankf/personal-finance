SELECT EXISTS (
        SELECT *
        FROM CashAccountProduct
            INNER JOIN AccountType USING (account_type_id)
        WHERE account_type = ?
    ) AS "is_account_exist!:bool"