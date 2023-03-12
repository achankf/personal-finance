use db::SqlResult;

use crate::MyTransaction;

pub struct AssetRebalance {
    pub first_name: String,
    pub last_name: String,
    pub asset_class_name: String,
    pub current_rebalance_amount: f64,
    pub potential_rebalance_amount: f64,
}

impl MyTransaction<'_> {
    pub async fn get_asset_rebalance(&mut self) -> SqlResult<Vec<AssetRebalance>> {
        let ret = sqlx::query_file_as!(AssetRebalance, "src/one_shot/get_asset_rebalance.sql",)
            .fetch_all(&mut *self.0)
            .await?;

        Ok(ret)
    }
}
