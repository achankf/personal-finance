use num_traits::{Signed, Zero};
use owo_colors::OwoColorize;
use transaction::MyBigDecimal;

pub fn format_colored_cad(v: &MyBigDecimal) -> String {
    if v.is_zero() {
        "".into()
    } else {
        let v = v.round2();
        if v.is_positive() {
            format!("${v}").green().to_string()
        } else {
            format!("${v}").red().to_string()
        }
    }
}

pub fn format_cad(v: &MyBigDecimal) -> String {
    if v.is_zero() {
        "".into()
    } else {
        format!("${v}").to_string()
    }
}
