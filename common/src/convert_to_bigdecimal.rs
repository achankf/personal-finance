use std::str::FromStr;

use bigdecimal::BigDecimal;

pub fn convert_to_bigdecimal(
    value: &Option<String>,
) -> Result<BigDecimal, Box<dyn std::error::Error>> {
    if let Some(value) = value {
        Ok(BigDecimal::from_str(value)?)
    } else {
        Ok(bigdecimal::Zero::zero())
    }
}
