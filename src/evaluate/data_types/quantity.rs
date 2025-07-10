use std::{
    cmp::Ordering,
    ops::{Add, Mul, Sub},
    str::FromStr,
};
use std::cell::LazyCell;
use regex::Regex;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{error::FhirpathError, evaluate::EvaluateResult, parser::literal::QuantityLiteral};

const QUANTITY_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"([0-9]+(\.[0-9]+)?)\s*(year|month|week|day|hour|minute|second|millisecond|years|months|weeks|days|hours|minutes|seconds|milliseconds|('[^']*'))").unwrap()
});

#[derive(PartialEq)]
pub enum CalendarUnit {
    Years,
    Months,
    Weeks,
    Days,
    Hours,
    Minutes,
    Seconds,
    Millis,
}

// define the static ordering of CalendarUnit
// so we know if a given CalendarUnit is
// greater than or less than another
static CALENDAR_UNIT_ORDER: [CalendarUnit; 8] = [
    CalendarUnit::Millis,
    CalendarUnit::Seconds,
    CalendarUnit::Minutes,
    CalendarUnit::Hours,
    CalendarUnit::Days,
    CalendarUnit::Weeks,
    CalendarUnit::Months,
    CalendarUnit::Years
];

#[derive(PartialEq)]
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

impl CalendarUnit {
    fn is_equivalent(&self, other: &TimeUnit) -> bool {
        match self {
            CalendarUnit::Years => other == &TimeUnit::Years,
            CalendarUnit::Months => other == &TimeUnit::Months,
            CalendarUnit::Weeks => other == &TimeUnit::Weeks,
            CalendarUnit::Days => other == &TimeUnit::Days,
            CalendarUnit::Hours => other == &TimeUnit::Hours,
            CalendarUnit::Minutes => other == &TimeUnit::Minutes,
            CalendarUnit::Seconds => other == &TimeUnit::Seconds,
            CalendarUnit::Millis => other == &TimeUnit::Millis,
        }
    }

    /// TimeUnit and CalendarUnit are considered equal
    /// for precision Second and below
    fn is_equal(&self, other: &TimeUnit) -> bool {
        match self {
            CalendarUnit::Seconds => other == &TimeUnit::Seconds,
            CalendarUnit::Millis => other == &TimeUnit::Millis,
            _ => false
        }
    }
}

impl TimeUnit {
    fn is_equivalent(&self, other: &CalendarUnit) -> bool {
        match self {
            TimeUnit::Years => other == &CalendarUnit::Years,
            TimeUnit::Months => other == &CalendarUnit::Months,
            TimeUnit::Weeks => other == &CalendarUnit::Weeks,
            TimeUnit::Days => other == &CalendarUnit::Days,
            TimeUnit::Hours => other == &CalendarUnit::Hours,
            TimeUnit::Minutes => other == &CalendarUnit::Minutes,
            TimeUnit::Seconds => other == &CalendarUnit::Seconds,
            TimeUnit::Millis => other == &CalendarUnit::Millis,
        }
    }

    /// TimeUnit and CalendarUnit are considered equal
    /// for precision Second and below
    fn is_equal(&self, other: &CalendarUnit) -> bool {
        match self {
            TimeUnit::Seconds => other == &CalendarUnit::Seconds,
            TimeUnit::Millis => other == &CalendarUnit::Millis,
            _ => false
        }
    }
}

impl TryFrom<&String> for CalendarUnit {
    type Error = FhirpathError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "year" | "years" => Ok(CalendarUnit::Years),
            "month" | "months" => Ok(CalendarUnit::Months),
            "week" | "weeks" => Ok(CalendarUnit::Weeks),
            "day" | "days" => Ok(CalendarUnit::Days),
            "hour" | "hours" => Ok(CalendarUnit::Hours),
            "minute" | "minutes" => Ok(CalendarUnit::Minutes),
            "second" | "seconds" => Ok(CalendarUnit::Seconds),
            "millisecond" | "milliseconds" => Ok(CalendarUnit::Millis),
            _ => Err(FhirpathError::EvaluateError {
                msg: format!("{} is not a Calendar Unit", value),
            }),
        }
    }
}

impl PartialOrd for CalendarUnit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_index = CALENDAR_UNIT_ORDER.iter().enumerate().find(|&(i, x)| x == self)?;
        let other_index = CALENDAR_UNIT_ORDER.iter().enumerate().find(|&(i, x)| x == other)?;

        self_index.partial_cmp(&other_index)
    }
}

impl TryFrom<&String> for TimeUnit {
    type Error = FhirpathError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "'a'" => Ok(TimeUnit::Years),
            "'mo'" => Ok(TimeUnit::Months),
            "'wk'" => Ok(TimeUnit::Weeks),
            "'d'" => Ok(TimeUnit::Days),
            "'h'" => Ok(TimeUnit::Hours),
            "'min'" => Ok(TimeUnit::Minutes),
            "'s'" => Ok(TimeUnit::Seconds),
            "'ms'" => Ok(TimeUnit::Millis),
            _ => Err(FhirpathError::EvaluateError {
                msg: format!("{} is not a Time Unit", value),
            }),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Quantity {
    pub value: Decimal,
    pub unit: Option<String>,
}

impl TryFrom<&QuantityLiteral> for Quantity {
    type Error = FhirpathError;

    fn try_from(value: &QuantityLiteral) -> Result<Self, Self::Error> {
        let quant_value =
            Decimal::from_str(value.text.as_str()).map_err(|err| FhirpathError::EvaluateError {
                msg: format!("Failed to parse Quantity.value: {}", err.to_string()),
            })?;

        Ok(Self {
            value: quant_value,
            unit: value.unit.clone(),
        })
    }
}

impl TryFrom<&Value> for Quantity {
    type Error = FhirpathError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Object(obj) => {
                let q_value = obj
                    .get("value")
                    .and_then(|val| match val {
                        Value::Number(num) => Decimal::from_f64(num.as_f64()?),
                        Value::String(string_num) => Decimal::from_str_exact(&string_num).ok(),
                        _ => None,
                    })
                    .ok_or(FhirpathError::EvaluateError {
                        msg: "Could not parse Quantity.value".to_string(),
                    })?;

                let unit_prop = obj.get("unit");

                match unit_prop {
                    None => {
                        return Ok(Quantity {
                            value: q_value,
                            unit: None,
                        })
                    }
                    Some(unit) => match unit {
                        Value::String(string_unit) => Ok(Quantity {
                            value: q_value,
                            unit: Some(string_unit.to_string()),
                        }),
                        _ => Err(FhirpathError::EvaluateError {
                            msg: "Invalid Qunatity.unit".to_string(),
                        }),
                    },
                }
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: "Cannot convert value to Quantity".to_string(),
            }),
        }
    }
}

impl TryFrom<&String> for Quantity {
    type Error = FhirpathError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let captures = Regex::captures(&QUANTITY_REGEX, value).ok_or(FhirpathError::EvaluateError { msg: format!("Failed to parse Quantity. {}", value) })?;

        let capture_text = captures[1].to_string();

        let capture_unit = captures[3].to_string();

        Ok(Quantity {
            value: Decimal::from_str_exact(&capture_text).map_err(|e| FhirpathError::EvaluateError {
                msg: format!("Could not convert value to Decimal: {}", e)
            })?,
            unit: Some(capture_unit),
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

    pub fn fhir_eq(&self, other: &Self) -> Option<bool> {
        if let Ok(conv) = self.try_convert(other) {
            return Some(conv.value.eq(&other.value) && conv.unit.eq(&other.unit));
        }

        // if Quantities are Calendar and Time unit based we can check for equality
        let calendar_unit_first = CalendarUnit::try_from(&self.unit.clone()?);
        let calendar_unit_second = CalendarUnit::try_from(&other.unit.clone()?);

        // check if either unit is a UCUM time unit
        let time_unit_first = TimeUnit::try_from(&self.unit.clone()?);
        let time_unit_second = TimeUnit::try_from(&other.unit.clone()?);

        // check for equality by Calendar and Time unit semantics
        if let (Ok(first), Ok(second)) = (calendar_unit_first, time_unit_second) {
            return Some(first.is_equal(&second));
        }

        if let (Ok(first), Ok(second)) = (time_unit_first, calendar_unit_second) {
            return Some(first.is_equal(&second));
        }

        None
    }


    pub fn try_convert(&self, other: &Quantity) -> EvaluateResult<Quantity> {
        match (&self.unit, &other.unit) {
            (None, None) => Ok(self.clone()),
            (Some(u1), Some(u2)) => {
                if u1.eq(u2) {
                    return Ok(self.clone());
                }

                // if they are both calendar units convert using
                // calendar conversion
                let calendar_unit_first = CalendarUnit::try_from(u1);
                let calendar_unit_second = CalendarUnit::try_from(u2);

                // @todo calendar unit conversion

                // check if either unit is a UCUM time unit
                let time_unit_first = TimeUnit::try_from(u1);
                let time_unit_second = TimeUnit::try_from(u2);

                // if the Calendar and Time unit are equal according to
                // fhirpath rules we can safely convert
                if let (Ok(first), Ok(second)) = (calendar_unit_first, time_unit_second) {
                    if first.is_equal(&second) {
                        return Ok(Quantity { value: self.value, unit: other.unit.clone() })
                    }
                }

                if let (Ok(first), Ok(second)) = (time_unit_first, calendar_unit_second) {
                    if first.is_equal(&second) {
                        return Ok(Quantity { value: self.value, unit: other.unit.clone() })
                    }
                }

                Err(FhirpathError::EvaluateError {
                    msg: format!("Converting between {u1} and {u2} not supported."),
                })
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: "Cannot convert Quantities with mismatched units.".to_string(),
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
        let converted = self.try_convert(&other).ok()?;

        // units match so only need to compare values
        converted.value.partial_cmp(&other.value)
    }
}
