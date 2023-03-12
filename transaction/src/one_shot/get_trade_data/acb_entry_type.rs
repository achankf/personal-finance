use std::fmt;

use crate::AcbEntryType;

impl fmt::Display for AcbEntryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            AcbEntryType::Buy => "Buy",
            AcbEntryType::Sell => "Sell",
            AcbEntryType::ReturnOfCapital => "Return of Capital",
            AcbEntryType::NonCashDistribution => "Non-cash Distribution",
        };
        write!(f, "{value}")
    }
}
