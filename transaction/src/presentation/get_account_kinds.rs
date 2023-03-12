use std::collections::HashMap;

use db::SqlResult;

use crate::{Account, Transaction};

pub struct AccountKindQuery {
    data: HashMap<i64, String>,
    asset_account_kind_id: i64,
    liabilities_account_kind_id: i64,
    equity_account_kind_id: i64,
    expense_account_kind_id: i64,
    revenue_account_kind_id: i64,
}

impl AccountKindQuery {
    pub fn new(data: HashMap<i64, String>) -> Self {
        let reverse_data: HashMap<String, i64> = data
            .iter()
            .map(|(value, key)| (key.clone(), *value))
            .collect();

        let asset_account_kind_id = *reverse_data.get("ASSET").expect("to get asset kind id");
        let liabilities_account_kind_id = *reverse_data
            .get("LIABILITIES")
            .expect("to get liabilities kind id");
        let equity_account_kind_id = *reverse_data.get("EQUITY").expect("to get equity kind id");
        let expense_account_kind_id = *reverse_data.get("EXPENSE").expect("to get expense kind id");
        let revenue_account_kind_id = *reverse_data.get("REVENUE").expect("to get revenue kind id");

        Self {
            data,
            asset_account_kind_id,
            liabilities_account_kind_id,
            equity_account_kind_id,
            expense_account_kind_id,
            revenue_account_kind_id,
        }
    }

    pub fn is_using_debit_balance(&self, account_kind_id: i64) -> bool {
        account_kind_id == self.asset_account_kind_id
            || account_kind_id == self.expense_account_kind_id
    }

    pub fn is_using_credit_balance(&self, account_kind_id: i64) -> bool {
        !self.is_using_debit_balance(account_kind_id)
    }

    pub fn is_asset_kind_id(&self, account_kind_id: i64) -> bool {
        self.asset_account_kind_id == account_kind_id
    }

    pub fn is_liabilities_kind_id(&self, account_kind_id: i64) -> bool {
        self.liabilities_account_kind_id == account_kind_id
    }

    /// must get the value or panic
    pub fn must_get_str(&self, account_kind_id: i64) -> String {
        self.data
            .get(&account_kind_id)
            .expect("to get the account kind")
            .clone()
    }
}

impl Transaction<'_> {
    pub async fn get_account_kinds(&mut self) -> SqlResult<HashMap<i64, String>> {
        struct RawData {
            account_kind_id: i64,
            account_kind: String,
        }

        Ok(sqlx::query_as!(
            RawData,
            r#"
SELECT
    account_kind_id,
    account_kind
FROM
    AccountKind
"#,
        )
        .fetch_all(&mut *self.0)
        .await?
        .into_iter()
        .map(|record| (record.account_kind_id, record.account_kind))
        .collect())
    }

    pub async fn get_account_kinds_reverse(&mut self) -> SqlResult<AccountKindQuery> {
        struct RawData {
            account_kind_id: i64,
            account_kind: String,
        }

        Ok(AccountKindQuery::new(
            sqlx::query_as!(
                RawData,
                r#"
SELECT
    account_kind_id,
    account_kind
FROM
    AccountKind
"#,
            )
            .fetch_all(&mut *self.0)
            .await?
            .into_iter()
            .map(|record| (record.account_kind_id, record.account_kind))
            .collect(),
        ))
    }
}
