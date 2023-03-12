WITH BaseData AS (
    SELECT person_id,
        currency_id,
        ROUND(
            SUM(
                ROUND(
                    unit * ROUND(
                        COALESCE(debit, 0) - COALESCE(credit, 0),
                        2
                    ),
                    2
                )
            ),
            2
        ) AS balance
    FROM FinancialEntry
        INNER JOIN OwnedAccount USING (account_id)
        INNER JOIN AccountSubtype USING (account_subtype_id)
        INNER JOIN AccountKind USING (account_kind_id)
    WHERE account_kind IN ('ASSET', 'LIABILITIES')
        AND account_subtype <> 'STOCK'
    GROUP BY person_id,
        currency_id
    UNION ALL
    SELECT person_id,
        currency_id,
        ROUND(
            ROUND(
                SUM(
                    CASE
                        WHEN debit IS NOT NULL THEN unit
                        ELSE - unit
                    END
                ),
                4
            ) * price,
            2
        ) AS balance
    FROM FinancialEntry
        INNER JOIN StockAccountEntry USING (account_id)
        INNER JOIN StockAccountHolder USING (stock_account_holder_id)
        INNER JOIN AccountSubtype USING (account_subtype_id)
        INNER JOIN SECURITY USING (security_id)
    WHERE account_subtype = 'STOCK'
    GROUP BY person_id,
        security_id,
        currency_id
    HAVING balance <> 0
),
Aggregation AS (
    SELECT person_id,
        currency_id,
        SUM(balance) AS balance
    FROM BaseData
    GROUP BY person_id,
        currency_id
)
SELECT first_name,
    last_name,
    currency AS "currency!:String",
    currency_symbol AS "currency_symbol!:String",
    balance AS "balance!:f64"
FROM Aggregation
    INNER JOIN Person USING (person_id)
    INNER JOIN Currency USING (currency_id)
WHERE balance <> 0