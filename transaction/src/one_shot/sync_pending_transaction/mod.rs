use std::path::PathBuf;

use common::deserialize_into_map;
use serde::Deserialize;

use crate::MyTransaction;

impl MyTransaction<'_> {
    pub async fn sync_pending_transactions(
        &mut self,
        path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Debug, Deserialize)]
        struct PendingTransaction {
            transaction_id: i64,
            note: String,
        }

        impl common::Id for PendingTransaction {
            type IdType = i64;

            fn id(&self) -> Self::IdType {
                self.transaction_id
            }
        }

        // parse the records from the source csv file
        let parsed_records = deserialize_into_map::<PendingTransaction>(&path)?;

        // remove all existing records
        sqlx::query_file!("src/one_shot/sync_pending_transaction/remove_pending_transaction.sql")
            .execute(&mut *self.0)
            .await?;

        // insert new records
        for PendingTransaction {
            transaction_id,
            note,
        } in parsed_records.values()
        {
            sqlx::query_file!(
                "src/one_shot/sync_pending_transaction/sync_pending_transaction.sql",
                transaction_id,
                note
            )
            .execute(&mut *self.0)
            .await?;
        }

        Ok(())
    }
}
