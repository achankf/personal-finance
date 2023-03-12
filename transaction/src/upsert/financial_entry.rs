use common::{excel_date_optional_time_format, is_numeric, is_optional_numeric, Id};
use serde::Deserialize;
use serde_trim::string_trim;

use db::Query;

#[derive(Debug, Deserialize)]
pub struct FinancialEntry {
    pub transaction_id: i64,
    pub item_id: i64,
    #[serde(deserialize_with = "excel_date_optional_time_format")]
    pub date: i64,
    #[serde(deserialize_with = "string_trim")]
    pub account_key: String,
    #[serde(deserialize_with = "is_numeric")]
    pub unit: String,
    #[serde(deserialize_with = "is_optional_numeric")]
    pub debit: Option<String>,
    #[serde(deserialize_with = "is_optional_numeric")]
    pub credit: Option<String>,
    #[serde(deserialize_with = "string_trim")]
    pub description: String,
}

impl Id for FinancialEntry {
    type IdType = (i64, i64);

    fn id(&self) -> (i64, i64) {
        (self.transaction_id, self.item_id)
    }
}

impl Query for FinancialEntry {
    fn query(&self) -> db::SqlQuery {
        sqlx::query!(
            r#"
INSERT INTO
    FinancialEntry (
        transaction_id,
        item_id,
        date,
        account_id,
        unit,
        debit,
        credit,
        description
    )
VALUES
    (
        ?,
        ?,
        ?,
        (
            SELECT
                account_id
            FROM
                Account
            WHERE
                account_key = ?
        ),
        ?,
        ?,
        ?,
        ?
    ) ON CONFLICT (transaction_id, item_id) DO
UPDATE
SET
    date = excluded.date,
    account_id = excluded.account_id,
    unit = excluded.unit,
    debit = excluded.debit,
    credit = excluded.credit,
    description = excluded.description
WHERE
    date <> excluded.date
    OR account_id <> excluded.account_id
    OR unit <> excluded.unit
    OR debit <> excluded.debit
    OR credit <> excluded.credit
    OR description <> excluded.description;
"#,
            self.transaction_id,
            self.item_id,
            self.date,
            self.account_key,
            self.unit,
            self.debit,
            self.credit,
            self.description
        )
    }
}
