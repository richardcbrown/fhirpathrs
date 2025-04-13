use std::{
    cmp::Ordering,
    ops::{Add, Mul, Sub},
    str::FromStr,
};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{error::FhirpathError, evaluate::CompileResult, parser::literal::QuantityLiteral};

pub enum TimeUnit {
    Years,
    Months,
    Weeks,
    Days,
    Hours,
    Minutes,
    Seconds,
    Millis,
}

impl TimeUnit {
    pub fn to_ucum(&self) -> &str {
        match &self {
            TimeUnit::Years => "a",
            TimeUnit::Months => "mo",
            TimeUnit::Weeks => "wk",
            TimeUnit::Days => "d",
            TimeUnit::Hours => "h",
            TimeUnit::Minutes => "min",
            TimeUnit::Seconds => "s",
            TimeUnit::Millis => "ms",
        }
    }

    // pub fn to_seconds(&self) -> Option<Decimal> {
    //     match self {
    //         TimeUnit::Years => Decimal::from_u32(356 * 60 * 60 * 60),
    //         TimeUnit::Months => Decimal::from_u32()
    //     }
    // }

    // pub fn get_conversion_factor(&self, other: &TimeUnit) -> Option<Decimal> {
    //     match self {
    //         TimeUnit::Years => match other {
    //             TimeUnit::Years => Decimal::from_i8(1),
    //             TimeUnit::Months => Decimal::from_i8(12),
    //             TimeUnit::Weeks => Decimal::from_i8(52),
    //             TimeUnit::Days => Decimal::from_i8(365),
    //         },
    //     }
    // }
}

impl TryFrom<&String> for TimeUnit {
    type Error = FhirpathError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "a" | "year" | "years" => Ok(TimeUnit::Years),
            "mo" | "month" | "months" => Ok(TimeUnit::Months),
            "wk" | "week" | "weeks" => Ok(TimeUnit::Weeks),
            "d" | "day" | "days" => Ok(TimeUnit::Days),
            "h" | "hour" | "hours" => Ok(TimeUnit::Hours),
            "min" | "minute" | "minutes" => Ok(TimeUnit::Minutes),
            "s" | "second" | "seconds" => Ok(TimeUnit::Seconds),
            "ms" | "millisecond" | "milliseconds" => Ok(TimeUnit::Millis),
            _ => Err(FhirpathError::CompileError {
                msg: format!("{} is not a Time Unit", value),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quantity {
    pub value: Decimal,
    pub unit: Option<String>,
}

impl TryFrom<&QuantityLiteral> for Quantity {
    type Error = FhirpathError;

    fn try_from(value: &QuantityLiteral) -> Result<Self, Self::Error> {
        let quant_value =
            Decimal::from_str(value.text.as_str()).map_err(|err| FhirpathError::CompileError {
                msg: format!("Failed to parse Quantity.value: {}", err.to_string()),
            })?;

        Ok(Self {
            value: quant_value,
            unit: value.unit.clone(),
        })
    }
}

impl Quantity {
    pub fn to_string(&self) -> String {
        match &self.unit {
            Some(u) => format!("{} {}", self.value, u),
            None => self.value.to_string(),
        }
    }

    pub fn try_convert_unit(&self, unit: &Option<String>) -> CompileResult<Quantity> {
        match (&self.unit, unit) {
            (None, None) => Ok(self.clone()),
            (Some(u1), Some(u2)) => {
                if u1.eq(u2) {
                    return Ok(self.clone());
                }

                // let t1 = TimeUnit::try_from(u1);
                // let t2 = TimeUnit::try_from(u2);

                // if let (Ok(time1), Ok(time2)) = (t1, t2) {
                //     let factor = time1.get_conversion_factor(time2);
                // }

                Err(FhirpathError::CompileError {
                    msg: format!("Converting between {u1} and {u2} not supported."),
                })
            }
            _ => Err(FhirpathError::CompileError {
                msg: format!("Cannot convert Quantities with mismatched units."),
            }),
        }
    }
}

impl Mul for Quantity {
    type Output = Option<Quantity>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.unit, rhs.unit) {
            (None, None) => Some(Quantity {
                value: self.value * rhs.value,
                unit: None,
            }),
            (Some(u1), None) => Some(Quantity {
                value: self.value * rhs.value,
                unit: Some(u1),
            }),
            (None, Some(u2)) => Some(Quantity {
                value: self.value * rhs.value,
                unit: Some(u2),
            }),
            // @todo needs proper unit support
            (Some(_), Some(_)) => None,
        }
    }
}

impl Add for Quantity {
    type Output = Option<Quantity>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.unit, rhs.unit) {
            (None, None) => Some(Quantity {
                value: self.value + rhs.value,
                unit: None,
            }),
            // @todo needs proper unit support
            (Some(_), Some(_)) => None,
            _ => None,
        }
    }
}

impl Sub for Quantity {
    type Output = Option<Quantity>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.unit, rhs.unit) {
            (None, None) => Some(Quantity {
                value: self.value - rhs.value,
                unit: None,
            }),
            // @todo needs proper unit support
            (Some(_), Some(_)) => None,
            _ => None,
        }
    }
}

impl PartialOrd for Quantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // attempt to convert self so units match with other
        let converted = self.try_convert_unit(&other.unit).ok()?;

        // units match so only need to compare values
        converted.value.partial_cmp(&other.value)
    }
}
