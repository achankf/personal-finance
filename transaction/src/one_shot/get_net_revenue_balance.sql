SELECT first_name,
    last_name,
    currency AS "currency!:String",
    currency_symbol AS "currency_symbol!:String",
    ROUND(
        SUM(
            ROUND(
                unit * ROUND(
                    COALESCE(credit, 0) - COALESCE(debit, 0),
                    2
                ),
                2
            )
        ),
        2
    ) AS "balance!:f64"
FROM FinancialEntry
    INNER JOIN OwnedAccount USING (account_id)
    INNER JOIN AccountSubtype USING (account_subtype_id)
    INNER JOIN AccountKind USING (account_kind_id)
    INNER JOIN Person USING (person_id)
    INNER JOIN Currency USING (currency_id)
WHERE account_kind IN ('REVENUE', 'EXPENSE')
    AND FinancialEntry.date BETWEEN ? AND ?
GROUP BY person_id,
    currency_id
ORDER BY person_id,
    currency_id