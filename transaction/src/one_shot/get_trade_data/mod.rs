use std::collections::{BTreeMap, HashMap};

use bigdecimal::BigDecimal;
use chrono::{DateTime, Local};

use super::get_distribution_ref::DistributionRefMap;

mod acb_entry_type;
mod gl_breakdown;
mod trade_data;
mod transaction;

#[derive(Debug, PartialEq, Eq)]
pub enum TradeType {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AccUnitResult {
    date: DateTime<Local>,

    // how many unit of stocks are changed by the current instance
    unit: BigDecimal,

    forex_rate: BigDecimal,
    price: BigDecimal,

    // this field stores how many units of stock up to this instance
    acc_unit: BigDecimal,

    trade_type: TradeType,
}

// person_id -> security_id -> timestamp -> result
type AccUnitMap = HashMap<i64, HashMap<i64, BTreeMap<DateTime<Local>, Vec<AccUnitResult>>>>;

#[derive(PartialEq, Eq, Clone)]
pub enum AcbEntryType {
    Buy,
    Sell,
    ReturnOfCapital,
    NonCashDistribution,
}

#[derive(Clone)]
pub struct AcbEntry {
    pub date: DateTime<Local>,
    pub r#type: AcbEntryType,
    pub unit: BigDecimal,
    pub price: BigDecimal,
    pub amount: BigDecimal,

    pub acb_change: BigDecimal,
    pub capital_gl: BigDecimal,
    pub acb: BigDecimal,
}

pub struct TradeData {
    acc_unit_map: AccUnitMap,
    distribution_ref_map: DistributionRefMap,
}

#[derive(Default, Debug)]
pub struct GlBreakDown {
    pub disposition_capital_gl: BigDecimal,
    pub total_cash_distribution: BigDecimal,
    pub total_non_cash_distribution: BigDecimal,
    pub return_of_capital: BigDecimal,
    pub distribution_capital_gain: BigDecimal,
    pub eligible_dividend: BigDecimal,
    pub non_eligible_dividend: BigDecimal,
    pub foreign_business_income: BigDecimal,
    pub foreign_non_business_income: BigDecimal,
    pub other_income: BigDecimal,
    pub foreign_distribution: BigDecimal,
}
