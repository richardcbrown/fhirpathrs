use std::{cell::LazyCell, cmp::Ordering};

use chrono::{Datelike, NaiveDateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{error::FhirpathError, evaluate::CompileResult};

use super::{
    date::Date,
    offset::{get_fixed_offset, Offset},
    time::Time,
    utils::{parse_optional_f64, parse_optional_u32},
};

/**
 * Capture groups:
 * 1. Years
 * 3. Months
 * 5. Days
 * 7. Hours
 * 9. Minutes
 * 11. Seconds + Millis
 * 13. Timezone
 * 14. Timezone direction
 * 15. Timezone hours
 * 16. Timezone minutes
 */
const DATETIME_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new("@([0-9][0-9][0-9][0-9])(\\-([0-9][0-9])(\\-([0-9][0-9])(T([0-9][0-9])(:([0-9][0-9])(:([0-9][0-9](\\.[0-9]+)?))?)?(Z|(\\+|\\-)([0-9][0-9]):([0-9][0-9]))?)?)?)?").expect("Invalid DateTime Regex")
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateTime {
    pub precision: DateTimePrecision,
    pub date: Date,
    pub time: Option<Time>,
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.precision != other.precision {
            return None;
        }

        let cmp = self.date.partial_cmp(&other.date)?;

        match cmp {
            Ordering::Equal => match (&self.time, &other.time) {
                (None, None) => Some(Ordering::Equal),
                (Some(t1), Some(t2)) => t1.partial_cmp(t2),
                _ => None,
            },
            _ => return Some(cmp),
        }
    }
}

impl DateTime {
    pub fn from_datetime_precision(
        dt: chrono::DateTime<Utc>,
        precision: &DateTimePrecision,
    ) -> Self {
        match precision {
            DateTimePrecision::Years => Self {
                date: Date {
                    years: Some(dt.year() as u32),
                    months: None,
                    days: None,
                },
                time: Time::from_datetime_precision(dt, precision),
                precision: precision.clone(),
            },
            DateTimePrecision::Months => Self {
                date: Date {
                    years: Some(dt.year() as u32),
                    months: Some(dt.month()),
                    days: None,
                },
                time: Time::from_datetime_precision(dt, precision),
                precision: precision.clone(),
            },
            _ => Self {
                date: Date {
                    years: Some(dt.year() as u32),
                    months: Some(dt.month()),
                    days: Some(dt.day()),
                },
                time: Time::from_datetime_precision(dt, precision),
                precision: precision.clone(),
            },
        }
    }

    pub fn from_components(
        seconds: Option<f64>,
        minutes: Option<u32>,
        hours: Option<u32>,
        days: Option<u32>,
        months: Option<u32>,
        years: Option<u32>,
        precision: DateTimePrecision,
    ) -> Self {
        Self {
            precision: precision.clone(),
            date: Date {
                years,
                months,
                days,
            },
            time: Time::from_components(seconds, minutes, hours, &precision),
        }
    }
}

impl TryFrom<&Value> for DateTime {
    type Error = FhirpathError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(val) => DateTime::try_from(val),
            _ => Err(FhirpathError::CompileError {
                msg: "Only parsing times from strings is supported".to_string(),
            }),
        }
    }
}

impl TryFrom<&String> for DateTime {
    type Error = FhirpathError;

    fn try_from(string_value: &String) -> Result<Self, Self::Error> {
        let value = match string_value.starts_with("@") {
            true => string_value.clone(),
            false => format!("@{}", string_value),
        };

        let captures = Regex::captures(&DATETIME_REGEX, &value).ok_or_else(|| {
            FhirpathError::CompileError {
                msg: format!("{} is not a DateTime", value),
            }
        })?;

        let years = parse_optional_u32(captures.get(1))?;
        let months = parse_optional_u32(captures.get(3))?;
        let days = parse_optional_u32(captures.get(5))?;

        let hours = parse_optional_u32(captures.get(7))?;
        let minutes = parse_optional_u32(captures.get(9))?;
        let seconds = parse_optional_f64(captures.get(11))?;

        let fractional_secs = captures.get(12).is_some();

        let tz = captures.get(13).and_then(|val| Some(val.as_str()));
        let tz_dir = captures.get(14).and_then(|val| Some(val.as_str()));
        let tz_hours = parse_optional_u32(captures.get(15))?;
        let tz_minutes = parse_optional_u32(captures.get(16))?;

        let precision =
            DateTimePrecision::from_components(seconds, minutes, hours, days, months, years)?;

        if tz.is_some() {
            let mut date_format = "@%Y-%m-%dT%H:%M:%S".to_string();

            if fractional_secs {
                date_format.extend("%.f".chars());
            }

            date_format.extend("%z".chars());

            let offset = Offset::from_components(tz, tz_dir, tz_hours, tz_minutes);

            let adjusted_datetime = NaiveDateTime::parse_from_str(&value, &date_format)
                .map_err(|err| FhirpathError::CompileError {
                    msg: format!("Failed to parse DateTime: {}", err.to_string()),
                })?
                .and_local_timezone(get_fixed_offset(&offset)?)
                .single()
                .ok_or_else(|| FhirpathError::CompileError {
                    msg: "Failed to map DateTime".to_string(),
                })?
                .to_utc();

            Ok(Self::from_datetime_precision(adjusted_datetime, &precision))
        } else {
            Ok(Self::from_components(
                seconds, minutes, hours, days, months, years, precision,
            ))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DateTimePrecision {
    Years,
    Months,
    Days,
    Hours,
    Minutes,
    Seconds,
}

impl DateTimePrecision {
    fn from_components(
        seconds: Option<f64>,
        minutes: Option<u32>,
        hours: Option<u32>,
        days: Option<u32>,
        months: Option<u32>,
        years: Option<u32>,
    ) -> CompileResult<DateTimePrecision> {
        match (seconds, minutes, hours, days, months, years) {
            (Some(_sec), _, _, _, _, _) => Ok(DateTimePrecision::Seconds),
            (None, Some(_min), _, _, _, _) => Ok(DateTimePrecision::Minutes),
            (None, None, Some(_h), _, _, _) => Ok(DateTimePrecision::Hours),
            (None, None, None, Some(_day), _, _) => Ok(DateTimePrecision::Days),
            (None, None, None, None, Some(_mon), _) => Ok(DateTimePrecision::Months),
            (None, None, None, None, None, Some(_yr)) => Ok(DateTimePrecision::Years),
            (None, None, None, None, None, None) => Err(FhirpathError::CompileError {
                msg: "Invalid DateTime precision".to_string(),
            }),
        }
    }
}
