use db::SqlResult;
use serde::Serialize;

use crate::MyTransaction;

#[derive(Serialize, Debug)]
pub struct CreditCardPadInjection {
    pub name: String,
    pub account_name: String,
    pub min_injection: f64,
}

impl MyTransaction<'_> {
    pub async fn get_credit_card_pad_injection(
        &mut self,
    ) -> SqlResult<Vec<CreditCardPadInjection>> {
        let result = sqlx::query_file_as!(
            CreditCardPadInjection,
            "src/one_shot/get_credit_card_pad_injection.sql"
        )
        .fetch_all(&mut *self.0)
        .await?;

        Ok(result)
    }
}
