use core::fmt;
use std::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign, Div, Mul, Neg, Rem, Sub},
    str::FromStr,
};

use bigdecimal::{BigDecimal, ParseBigDecimalError};
use num_traits::{Num, One, Signed, Zero};
use serde::Deserialize;

/// Wrapper for BigDecimal
/// Features:
///    - provide From<String> implementation based from BigDecimal's FromStr, so that I can use this type in sqlx's FromRow derive macro.
///    - empty strings are treated as 0's (used for converting spreadsheets)
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct MyBigDecimal(pub BigDecimal);

impl MyBigDecimal {
    /// Round number with bigdecimal::RoundingMode::HalfUp
    pub fn round(&self, digits: i64) -> Self {
        Self(
            self.0
                .with_scale_round(digits, bigdecimal::RoundingMode::HalfUp),
        )
    }

    /// Round number with bigdecimal::RoundingMode::HalfUp, into 2 decimal places.
    pub fn round2(&self) -> Self {
        self.round(2)
    }

    /// Round number with bigdecimal::RoundingMode::HalfUp, into 4 decimal places.
    pub fn round4(&self) -> Self {
        self.round(4)
    }

    /// For calculations related to fractional shares; rounding to 9 decimal places.
    pub fn round9(&self) -> Self {
        self.round(9)
    }
}

impl<'de> Deserialize<'de> for MyBigDecimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(BigDecimal::deserialize(deserializer)?))
    }
}

impl Display for MyBigDecimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<String> for MyBigDecimal {
    fn from(value: String) -> Self {
        if value.trim().is_empty() {
            // special case for treating empty strings as zeroes
            MyBigDecimal(BigDecimal::zero())
        } else {
            MyBigDecimal(
                BigDecimal::from_str(&value).expect("to parse the string into a BigDecimal"),
            )
        }
    }
}

impl From<i64> for MyBigDecimal {
    fn from(value: i64) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl From<i32> for MyBigDecimal {
    fn from(value: i32) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl Add for MyBigDecimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for MyBigDecimal {
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(rhs.0);
    }
}

impl Sub for MyBigDecimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Rem for MyBigDecimal {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl Mul for MyBigDecimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul<i32> for MyBigDecimal {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<i32> for &MyBigDecimal {
    type Output = MyBigDecimal;

    fn mul(self, rhs: i32) -> Self::Output {
        MyBigDecimal(self.0.clone() * rhs)
    }
}

impl Div for MyBigDecimal {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Div<i32> for MyBigDecimal {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Neg for MyBigDecimal {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Num for MyBigDecimal {
    type FromStrRadixErr = ParseBigDecimalError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Self(BigDecimal::from_str_radix(str, radix)?))
    }
}

impl Signed for MyBigDecimal {
    fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    fn abs_sub(&self, other: &Self) -> Self {
        Self(self.0.abs_sub(&other.0))
    }

    fn signum(&self) -> Self {
        Self(self.0.signum())
    }

    fn is_positive(&self) -> bool {
        self.0.is_positive()
    }

    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
}

impl Zero for MyBigDecimal {
    fn zero() -> Self {
        Self(BigDecimal::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl One for MyBigDecimal {
    fn one() -> Self {
        Self(BigDecimal::one())
    }

    fn set_one(&mut self) {
        *self = One::one();
    }

    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}
