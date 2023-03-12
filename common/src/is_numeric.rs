use std::str::FromStr;

use bigdecimal::BigDecimal;
use serde::de::Error;
use serde::{de, Deserialize};

pub fn is_numeric<'de, D>(d: D) -> Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    let de_string = String::deserialize(d)?;
    let de_string = de_string.trim();

    match BigDecimal::from_str(&de_string) {
        Ok(_) => Ok(de_string.to_owned()),
        Err(e) => Err(D::Error::custom(e)),
    }
}
