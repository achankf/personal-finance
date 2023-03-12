use std::collections::BTreeSet;

use bigdecimal::{BigDecimal, Zero};
use chrono::{DateTime, Datelike, Local};

use crate::{one_shot::get_trade_data::TradeType, AcbEntry, AcbEntryType, GlBreakDown};

use super::TradeData;

impl TradeData {
    pub fn get_acc_unit(
        &self,
        person_id: i64,
        security_id: i64,
        date: &DateTime<Local>,
    ) -> Option<BigDecimal> {
        self.acc_unit_map
            .get(&person_id)
            .and_then(|security_map| security_map.get(&security_id))
            .and_then(|time_map| time_map.range(..date).last())
            .and_then(|(_, record_list)| record_list.last().map(|record| record.acc_unit.clone()))
    }

    /// get a set of security_ids owned by the given person, including ones that were previously owned
    pub fn get_owned_securities(&self, person_id: i64) -> BTreeSet<i64> {
        if let Some(security_map) = self.acc_unit_map.get(&person_id) {
            security_map
                .keys()
                .map(|security_id| *security_id)
                .collect()
        } else {
            Default::default()
        }
    }

    pub fn get_single_breakdown(
        &self,
        person_id: i64,
        security_id: i64,
        year: &Option<i32>,
    ) -> GlBreakDown {
        let filter_by_year = |date: &DateTime<Local>| -> bool {
            if let Some(year) = year {
                date.year() == *year
            } else {
                true
            }
        };

        let disposition_capital_gl = self
            .get_acb(person_id, security_id)
            .into_iter()
            .filter(|entry| entry.r#type == AcbEntryType::Sell && filter_by_year(&entry.date))
            .fold(BigDecimal::zero(), |acc, cur| acc + cur.capital_gl)
            .with_scale_round(2, bigdecimal::RoundingMode::HalfUp);

        let init_value = GlBreakDown {
            disposition_capital_gl,
            ..GlBreakDown::default()
        };

        self.distribution_ref_map
            .get(&security_id)
            .unwrap_or(&Default::default())
            .values()
            .filter(|x| filter_by_year(&x.record_date))
            .fold(init_value, |acc, entry| {
                let unit = self
                    .get_acc_unit(person_id, security_id, &entry.ex_dividend_date)
                    .unwrap_or_default();
                let balance = |x: &BigDecimal| x * unit.clone();

                acc + GlBreakDown {
                    return_of_capital: balance(&entry.return_of_capital),
                    total_cash_distribution: balance(&entry.total_cash_distribution),
                    total_non_cash_distribution: balance(&entry.total_non_cash_distribution),
                    distribution_capital_gain: balance(&entry.capital_gain),
                    eligible_dividend: balance(&entry.eligible_dividend),
                    non_eligible_dividend: balance(&entry.non_eligible_dividend),
                    foreign_business_income: balance(&entry.foreign_business_income),
                    foreign_non_business_income: balance(&entry.foreign_non_business_income),
                    other_income: balance(&entry.other_income),
                    foreign_distribution: balance(&entry.foreign_distribution),
                    disposition_capital_gl: Zero::zero(),
                }
            })
    }

    pub fn get_acb(&self, person_id: i64, security_id: i64) -> Vec<AcbEntry> {
        #[derive(PartialEq, Eq)]
        struct RawAcbEntry {
            date: DateTime<Local>,
            r#type: AcbEntryType,
            unit: BigDecimal,
            price: BigDecimal,
        }

        impl PartialOrd for RawAcbEntry {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(&other))
            }
        }

        impl Ord for RawAcbEntry {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.date.cmp(&other.date)
            }
        }

        let trades = self
            .acc_unit_map
            .get(&person_id)
            .and_then(|security_map| security_map.get(&security_id))
            .map(|records| {
                records.values().flat_map(|record_list| {
                    record_list.iter().map(|record| RawAcbEntry {
                        date: record.date,
                        r#type: if record.trade_type == TradeType::Buy {
                            AcbEntryType::Buy
                        } else {
                            AcbEntryType::Sell
                        },
                        unit: record.unit.clone(),
                        price: record.price.clone() * record.forex_rate.clone(),
                    })
                })
            })
            .into_iter()
            .flatten();

        let distributions = self
            .distribution_ref_map
            .get(&security_id)
            .map(|distributions| {
                distributions.values().flat_map(|record| {
                    let acc_unit = self
                        .get_acc_unit(person_id, security_id, &record.ex_dividend_date)
                        .unwrap_or_default();

                    let mut ret: Vec<RawAcbEntry> = Vec::with_capacity(2);

                    if !record.return_of_capital.is_zero() {
                        ret.push(RawAcbEntry {
                            date: record.record_date.clone(),
                            r#type: AcbEntryType::ReturnOfCapital,
                            unit: acc_unit.clone(),
                            price: record.return_of_capital.clone(),
                        });
                    }

                    if !record.total_non_cash_distribution.is_zero() {
                        ret.push(RawAcbEntry {
                            date: record.record_date.clone(),
                            r#type: AcbEntryType::NonCashDistribution,
                            unit: acc_unit.clone(),
                            price: record.total_non_cash_distribution.clone(),
                        });
                    }

                    ret
                })
            })
            .into_iter()
            .flatten();

        // sort all entries by appropriate date values
        let entries: BTreeSet<_> = trades.chain(distributions).collect();

        let mut ret: Vec<_> = Vec::with_capacity(entries.len());
        let mut acc_acb = BigDecimal::zero();
        let mut acc_unit = BigDecimal::zero();
        for RawAcbEntry {
            date,
            r#type,
            unit,
            price,
        } in entries
        {
            let amount = (unit.clone() * price.clone())
                .with_scale_round(2, bigdecimal::RoundingMode::HalfUp);

            let (acb_change, capital_gl): (BigDecimal, BigDecimal) = match r#type {
                AcbEntryType::Buy => (amount.clone(), 0.into()),
                AcbEntryType::Sell => {
                    let acb_change = -unit.clone() / acc_unit.clone() * acc_acb.clone();
                    let capital_gl = amount.clone() + acb_change.clone();
                    (acb_change, capital_gl)
                }
                AcbEntryType::NonCashDistribution => (amount.clone(), amount.clone()),
                AcbEntryType::ReturnOfCapital => (-amount.clone(), 0.into()),
            };

            let acb_change = acb_change.with_scale_round(2, bigdecimal::RoundingMode::HalfUp);

            let cur_acb = acc_acb.clone() + acb_change.clone();
            let unit_change = match r#type {
                AcbEntryType::Buy => unit.clone(),
                AcbEntryType::Sell => -unit.clone(),
                _ => BigDecimal::zero(),
            };

            let cur_unit = acc_unit.clone() + unit_change.clone();

            ret.push(AcbEntry {
                date,
                r#type,
                unit,
                price,
                amount,
                acb_change,
                capital_gl,
                acb: cur_acb.clone(),
            });

            acc_acb = cur_acb;
            acc_unit = cur_unit;
        }

        ret
    }
}
