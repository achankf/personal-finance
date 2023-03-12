use std::collections::HashMap;

use db::SqlResult;

use crate::MyTransaction;

pub struct DistributionTaxBreakdownTypes {
    // ids for each constant
    pub foreign_id: i64,
    pub rate_id: i64,
    pub percent_id: i64,
}

impl DistributionTaxBreakdownTypes {
    // ensures the args is a valid value
    fn sanity_check(&self, distribution_tax_breakdown_id: i64) {
        assert!(
            self.foreign_id == distribution_tax_breakdown_id
                || self.rate_id == distribution_tax_breakdown_id
                || self.percent_id == distribution_tax_breakdown_id
        );
    }

    pub fn is_foreign(&self, distribution_tax_breakdown_id: i64) -> bool {
        self.sanity_check(distribution_tax_breakdown_id);
        self.foreign_id == distribution_tax_breakdown_id
    }

    pub fn is_rate(&self, distribution_tax_breakdown_id: i64) -> bool {
        self.sanity_check(distribution_tax_breakdown_id);
        self.rate_id == distribution_tax_breakdown_id
    }

    pub fn is_percent(&self, distribution_tax_breakdown_id: i64) -> bool {
        self.sanity_check(distribution_tax_breakdown_id);
        self.percent_id == distribution_tax_breakdown_id
    }
}

impl MyTransaction<'_> {
    pub async fn get_distribution_tax_breakdown_types(
        &mut self,
    ) -> SqlResult<DistributionTaxBreakdownTypes> {
        struct RawData {
            distribution_tax_breakdown_type_id: i64,
            distribution_tax_breakdown_type: String,
        }

        let mapping: HashMap<_, _> = sqlx::query_file_as!(
            RawData,
            "src/utils/get_distribution_tax_breakdown_types.sql"
        )
        .map(
            |RawData {
                 distribution_tax_breakdown_type_id,
                 distribution_tax_breakdown_type,
             }| {
                (
                    distribution_tax_breakdown_type,
                    distribution_tax_breakdown_type_id,
                )
            },
        )
        .fetch_all(&mut *self.0)
        .await?
        .into_iter()
        .collect();

        Ok(DistributionTaxBreakdownTypes {
            foreign_id: mapping.get("FOREIGN").expect("id for FOREIGN").clone(),
            rate_id: mapping.get("RATE").expect("id for RATE").clone(),
            percent_id: mapping.get("PERCENT").expect("id for PERCENT").clone(),
        })
    }
}
