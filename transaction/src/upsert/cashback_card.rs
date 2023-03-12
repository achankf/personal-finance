use common::Id;
use serde::Deserialize;
use serde_trim::string_trim;

use db::Query;

#[derive(Deserialize, Debug)]
pub struct CashbackCard {
    #[serde(deserialize_with = "string_trim")]
    pub account_type: String,
}

impl Id for CashbackCard {
    type IdType = String;

    fn id(&self) -> Self::IdType {
        self.account_type.clone()
    }
}

impl Query for CashbackCard {
    fn query(&self) -> db::SqlQuery {
        sqlx::query!(
            r#"
INSERT
    OR IGNORE INTO CashbackCard (account_type_id)
VALUES
    (
        (
            SELECT
                account_type_id
            FROM
                AccountType
            WHERE
                account_type = ?
        )
    )
"#,
            self.account_type,
        )
    }
}
