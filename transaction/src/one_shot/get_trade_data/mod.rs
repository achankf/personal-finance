use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Local};

use crate::MyBigDecimal;

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
    unit: MyBigDecimal,

    forex_rate: MyBigDecimal,
    price: MyBigDecimal,

    // this field stores how many units of stock up to this instance
    acc_unit: MyBigDecimal,

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
    pub unit: MyBigDecimal,
    pub price: MyBigDecimal,
    pub amount: MyBigDecimal,

    pub acb_change: MyBigDecimal,
    pub capital_gl: MyBigDecimal,
    pub acb: MyBigDecimal,
}

pub struct TradeData {
    acc_unit_map: AccUnitMap,
    distribution_ref_map: DistributionRefMap,
}

#[derive(Default, Debug)]
pub struct GlBreakDown {
    pub disposition_capital_gl: MyBigDecimal,
    pub total_cash_distribution: MyBigDecimal,
    pub total_non_cash_distribution: MyBigDecimal,
    pub return_of_capital: MyBigDecimal,
    pub distribution_capital_gain: MyBigDecimal,
    pub eligible_dividend: MyBigDecimal,
    pub non_eligible_dividend: MyBigDecimal,
    pub foreign_business_income: MyBigDecimal,
    pub foreign_non_business_income: MyBigDecimal,
    pub other_income: MyBigDecimal,
    pub foreign_business_income_tax_paid: MyBigDecimal,
    pub foreign_non_business_income_tax_paid: MyBigDecimal,
    pub foreign_distribution: MyBigDecimal,
}
