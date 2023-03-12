use chrono::{DateTime, Local};
use futures::StreamExt;
use num_traits::Zero;
use serde::Deserialize;

use crate::{
    one_shot::get_trade_data::{AccUnitMap, AccUnitResult, TradeType},
    MyBigDecimal, MyTransaction,
};

use super::TradeData;

impl MyTransaction<'_> {
    pub async fn get_trade_data(&mut self) -> Result<TradeData, Box<dyn std::error::Error>> {
        let acc_unit_map = {
            #[derive(Clone, Deserialize, Debug)]
            struct RawRecord {
                pub person_id: i64,
                pub security_id: i64,
                pub date: DateTime<Local>,
                pub forex_rate: MyBigDecimal,
                pub unit: MyBigDecimal,
                pub debit: MyBigDecimal,
                pub credit: MyBigDecimal,
                pub is_debit_record: bool,
            }

            let mut stream =
                sqlx::query_file_as!(RawRecord, "src/one_shot/get_trade_data/get_trade_data.sql")
                    .fetch(&mut *self.0);

            let mut key = None;
            let mut acc_unit = MyBigDecimal::zero();
            let mut acc_unit_map: AccUnitMap = Default::default();

            while let Some(record) = stream.next().await {
                let RawRecord {
                    person_id,
                    security_id,
                    date,
                    forex_rate,
                    unit,
                    debit,
                    credit,
                    is_debit_record,
                } = record?;

                // adjust sign for a debit balance on the unit
                let (trade_type, price, unit_change) = if is_debit_record {
                    (TradeType::Buy, debit, unit.clone())
                } else {
                    (TradeType::Sell, credit, -unit.clone())
                };

                let next_key = Some((person_id, security_id));
                if key != next_key {
                    acc_unit = Zero::zero();
                    key = next_key;
                }

                acc_unit += unit_change.clone();

                acc_unit_map
                    .entry(person_id)
                    .or_default()
                    .entry(security_id)
                    .or_default()
                    .entry(date)
                    .or_default()
                    .push(AccUnitResult {
                        date,
                        unit,
                        forex_rate,
                        price,
                        acc_unit: acc_unit.clone(),
                        trade_type,
                    });
            }
            acc_unit_map
        };

        let distribution_ref_map = self.get_distribution_ref().await?;

        let result = TradeData {
            acc_unit_map,
            distribution_ref_map,
        };

        Ok(result)
    }
}
