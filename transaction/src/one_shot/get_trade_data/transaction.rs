use std::str::FromStr;

use bigdecimal::{BigDecimal, One, Zero};
use chrono::{DateTime, Local};
use futures::StreamExt;
use serde::Deserialize;

use crate::{
    one_shot::get_trade_data::{AccUnitMap, AccUnitResult, TradeType},
    Transaction,
};

use super::TradeData;

impl Transaction<'_> {
    pub async fn get_trade_data(&mut self) -> Result<TradeData, Box<dyn std::error::Error>> {
        let acc_unit_map = {
            #[derive(Clone, Deserialize, Debug)]
            struct RawRecord {
                pub person_id: i64,
                pub security_id: i64,
                pub date: DateTime<Local>,
                pub forex_rate: Option<String>,
                pub unit: String,
                pub debit: Option<String>,
                pub credit: Option<String>,
            }

            let mut stream =
                sqlx::query_file_as!(RawRecord, "src/one_shot/get_trade_data/get_trade_data.sql")
                    .fetch(&mut *self.0);

            let mut key = None;
            let mut acc_unit = BigDecimal::zero();
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
                } = record?;

                assert!(debit.is_some() ^ credit.is_some());

                let unit: BigDecimal = BigDecimal::from_str(&unit)?; // adjust sign for a debit balance on the unit
                let (trade_type, price, unit_change) = if let Some(debit) = debit {
                    (TradeType::Buy, BigDecimal::from_str(&debit)?, unit.clone())
                } else if let Some(credit) = credit {
                    (
                        TradeType::Sell,
                        BigDecimal::from_str(&credit)?,
                        -unit.clone(),
                    )
                } else {
                    unreachable!()
                };
                let forex_rate = {
                    if let Some(value) = forex_rate {
                        BigDecimal::from_str(&value)?
                    } else {
                        BigDecimal::one()
                    }
                };

                let next_key = Some((person_id, security_id));
                if key != next_key {
                    acc_unit = BigDecimal::zero();
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
