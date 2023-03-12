use db::SqlResult;

use crate::Transaction;

pub struct DistributionCheckResult {
    pub transaction_id: i64,
    pub ticker: String,
}

impl Transaction<'_> {
    /// Ensures Financial distribution entries matches the reference data (and vice-versa).
    pub async fn check_distribution(&mut self) -> SqlResult<Vec<DistributionCheckResult>> {
        let result = sqlx::query_as!(
            DistributionCheckResult,
            r#"
SELECT
    transaction_id,
    ticker
FROM
    StockAccountEntry
    INNER JOIN AccountSubtype USING (account_subtype_id)
    INNER JOIN StockAccountHolder s USING (stock_account_holder_id)
    INNER JOIN SECURITY USING (security_id)
    INNER JOIN FinancialEntry f USING (account_id)
WHERE
    account_subtype = 'DISTRIBUTION'
    AND transaction_id NOT IN (
        SELECT
            transaction_id
        FROM
            StockAccountEntry
            INNER JOIN AccountSubtype USING (account_subtype_id)
            INNER JOIN StockAccountHolder s USING (stock_account_holder_id)
            INNER JOIN FinancialEntry f USING (account_id)
            INNER JOIN CadDistribution d ON d.security_id = s.security_id
            AND d.payment_date = f.date
            AND d.total_cash_distribution = f.credit
        WHERE
            account_subtype = 'DISTRIBUTION'
    )
    AND ticker NOT IN ('TDB2460', 'TDB888')
ORDER BY
    date
"#
        )
        .fetch_all(&mut *self.0)
        .await?;

        Ok(result)
    }
}
