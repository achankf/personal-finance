use common::{is_numeric, Id};
use serde::Deserialize;

use db::Query;

#[derive(Debug, Deserialize)]
pub struct TransactionForex {
    transaction_id: i64,
    #[serde(deserialize_with = "is_numeric")]
    exchange_rate: String,
}

impl Id for TransactionForex {
    type IdType = i64;

    fn id(&self) -> Self::IdType {
        self.transaction_id
    }
}

impl Query for TransactionForex {
    fn query(&self) -> db::SqlQuery {
        sqlx::query!(
            r#"
INSERT INTO
    TransactionForex (transaction_id, exchange_rate)
VALUES
    (?, ?) ON CONFLICT(transaction_id) DO
UPDATE
SET
    exchange_rate = excluded.exchange_rate
WHERE
    exchange_rate <> excluded.exchange_rate
"#,
            self.transaction_id,
            self.exchange_rate
        )
    }
}
