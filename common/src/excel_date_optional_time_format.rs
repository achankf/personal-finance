use core::fmt;

use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};

struct ExcelDateOptionalTimeVisitor;

impl<'de> serde::de::Visitor<'de> for ExcelDateOptionalTimeVisitor {
    type Value = i64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a formatted date string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if let Ok(datetime) = NaiveDateTime::parse_from_str(s, "%m/%d/%Y %H:%M:%S") {
            let datetime = datetime.and_local_timezone(Local).unwrap();
            return Ok(datetime.timestamp());
        }

        match NaiveDate::parse_from_str(s, "%m/%d/%Y") {
            Ok(date) => {
                let time = NaiveTime::from_num_seconds_from_midnight_opt(0, 0)
                    .expect("cannot to create 12:00AM constant");

                let datetime = date.and_time(time).and_local_timezone(Local).unwrap();
                Ok(datetime.timestamp())
            }
            Err(_) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(s),
                &self,
            )),
        }
    }
}

pub fn excel_date_optional_time_format<'de, D>(d: D) -> Result<i64, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    d.deserialize_str(ExcelDateOptionalTimeVisitor)
}
