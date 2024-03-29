use std::ops::Range;

use chrono::{DateTime, TimeZone};
use chrono::{Local, NaiveDateTime};

pub fn all_time_until_now() -> Range<DateTime<Local>> {
    let zero = NaiveDateTime::default();
    let unix_epoch_start = Local::from_local_datetime(&Local, &zero)
        .single()
        .expect("cannot create Datetime for unix timestamp 0");

    unix_epoch_start..Local::now()
}
