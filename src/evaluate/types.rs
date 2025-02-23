use std::cell::LazyCell;

use chrono::{Datelike, FixedOffset, Local, NaiveDateTime, Timelike, Utc};
use regex::{Match, Regex};
use serde_json::{Number, Value};

use crate::error::FhirpathError;

use super::CompileResult;

#[derive(Debug, Clone, PartialEq)]
enum DateTimePrecision {
    Years,
    Months,
    Days,
    Hours,
    Minutes,
    Seconds,
}

impl DateTimePrecision {
    fn get_format(&self) -> &str {
        match self {
            DateTimePrecision::Years => "@%Y",
            DateTimePrecision::Months => "@%Y-%m",
            DateTimePrecision::Days => "@%Y-%m-%d",
            DateTimePrecision::Hours => "@%Y-%m-%dT%H",
            DateTimePrecision::Minutes => "@%Y-%m-%dT%H:%M",
            DateTimePrecision::Seconds => "@%Y-%m-%dT%H:%M:%S",
        }
    }

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

#[derive(Debug, Clone, PartialEq)]
enum TimePrecision {
    Hours,
    Minutes,
    Seconds,
}

impl TimePrecision {
    fn from_components(
        seconds: Option<f64>,
        minutes: Option<u32>,
        hours: Option<u32>,
    ) -> CompileResult<Self> {
        match (seconds, minutes, hours) {
            (Some(_sec), _, _) => Ok(TimePrecision::Seconds),
            (None, Some(_min), _) => Ok(TimePrecision::Minutes),
            (None, None, Some(_hr)) => Ok(TimePrecision::Hours),
            (None, None, None) => Err(FhirpathError::CompileError {
                msg: "Invalid Time precision".to_string(),
            }),
        }
    }
}

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

/**
 * Capture groups:
 * 1. Hours
 * 3. Minutes
 * 5. Seconds + Millis
 */
const TIME_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new("@T([0-9][0-9])(:([0-9][0-9])(:([0-9][0-9](\\.([0-9]+))?)?)?)?")
        .expect("Invalid Time Regex")
});

#[derive(Debug, Clone, PartialEq)]
pub struct Quantity {}

#[derive(Debug, Clone, PartialEq)]
pub enum OffsetDirection {
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Offset {
    hours: u32,
    minutes: u32,
    direction: OffsetDirection,
}

impl Offset {
    fn from_components(
        tz: Option<&str>,
        tz_dir: Option<&str>,
        tz_hours: Option<u32>,
        tz_minutes: Option<u32>,
    ) -> Option<Self> {
        match tz {
            Some(val) => {
                if val == "Z" {
                    Some(Self {
                        direction: OffsetDirection::Plus,
                        hours: 0,
                        minutes: 0,
                    })
                } else {
                    let direction = match tz_dir {
                        Some("+") => Some(OffsetDirection::Plus),
                        Some("-") => Some(OffsetDirection::Minus),
                        _ => None,
                    }?;

                    let hours = tz_hours?;
                    let minutes = tz_minutes?;

                    Some(Self {
                        hours,
                        minutes,
                        direction,
                    })
                }
            }
            None => None,
        }
    }
}

fn get_fixed_offset(offset: &Option<Offset>) -> CompileResult<FixedOffset> {
    match offset {
        None => Ok(Local::now().offset().clone()),
        Some(tz) => {
            let seconds = ((tz.hours * 60) + tz.minutes) * 60;

            match tz.direction {
                OffsetDirection::Plus => FixedOffset::east_opt(seconds as i32).ok_or_else(|| {
                    FhirpathError::CompileError {
                        msg: "Could not create FixedOffset".to_string(),
                    }
                }),
                OffsetDirection::Minus => FixedOffset::west_opt(seconds as i32).ok_or_else(|| {
                    FhirpathError::CompileError {
                        msg: "Could not create FixedOffset".to_string(),
                    }
                }),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Time {
    hours: Option<u32>,
    minutes: Option<u32>,
    seconds: Option<f64>,
    precision: TimePrecision,
}

impl Time {
    fn from_datetime_precision(
        dt: chrono::DateTime<Utc>,
        precision: &DateTimePrecision,
    ) -> Option<Time> {
        match precision {
            DateTimePrecision::Years => None,
            DateTimePrecision::Months => None,
            DateTimePrecision::Days => None,
            DateTimePrecision::Hours => Some(Time {
                hours: Some(dt.hour()),
                minutes: None,
                seconds: None,
                precision: TimePrecision::Hours,
            }),
            DateTimePrecision::Minutes => Some(Time {
                hours: Some(dt.hour()),
                minutes: Some(dt.minute()),
                seconds: None,
                precision: TimePrecision::Minutes,
            }),
            DateTimePrecision::Seconds => {
                let secs = format!("{}.{}", dt.second(), dt.nanosecond())
                    .parse::<f64>()
                    .ok()?;

                Some(Time {
                    hours: Some(dt.hour()),
                    minutes: Some(dt.minute()),
                    seconds: Some(secs),
                    precision: TimePrecision::Seconds,
                })
            }
        }
    }
}

fn parse_optional_u32(capture: Option<Match>) -> CompileResult<Option<u32>> {
    Ok(capture.and_then(|val| Some(val.as_str()))).and_then(|hour| match hour {
        Some(h) => {
            let parse_result = h.parse::<u32>().map_err(|_| FhirpathError::CompileError {
                msg: "Could not parse value to number".to_string(),
            });

            match parse_result {
                Ok(res) => Ok(Some(res)),
                Err(err) => Err(err),
            }
        }
        None => Ok(None),
    })
}

fn parse_optional_f64(capture: Option<Match>) -> CompileResult<Option<f64>> {
    Ok(capture.and_then(|val| Some(val.as_str()))).and_then(|hour| match hour {
        Some(h) => {
            let parse_result = h.parse::<f64>().map_err(|_| FhirpathError::CompileError {
                msg: "Could not parse value to number".to_string(),
            });

            match parse_result {
                Ok(res) => Ok(Some(res)),
                Err(err) => Err(err),
            }
        }
        None => Ok(None),
    })
}

impl TryFrom<&Value> for Time {
    type Error = FhirpathError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(val) => {
                let captures = Regex::captures(&TIME_REGEX, val).ok_or_else(|| {
                    FhirpathError::CompileError {
                        msg: format!("{} is not a Time", val),
                    }
                })?;

                let hours = parse_optional_u32(captures.get(1))?;
                let minutes = parse_optional_u32(captures.get(3))?;
                let seconds = parse_optional_f64(captures.get(5))?;

                let precision = match (seconds, minutes, hours) {
                    (Some(_sec), _, _) => Ok(TimePrecision::Seconds),
                    (None, Some(_min), _) => Ok(TimePrecision::Minutes),
                    (None, None, Some(_hr)) => Ok(TimePrecision::Hours),
                    (None, None, None) => Err(FhirpathError::CompileError {
                        msg: "No matching time precision".to_string(),
                    }),
                }?;

                Ok(Self {
                    hours,
                    minutes,
                    seconds,
                    precision,
                })
            }
            _ => Err(FhirpathError::CompileError {
                msg: "Only parsing times from strings is supported".to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Date {
    years: Option<u32>,
    months: Option<u32>,
    days: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DateTime {
    precision: DateTimePrecision,
    date: Date,
    time: Option<Time>,
}

impl DateTime {
    fn from_datetime_precision(dt: chrono::DateTime<Utc>, precision: &DateTimePrecision) -> Self {
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
}

impl TryFrom<&Value> for DateTime {
    type Error = FhirpathError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(val) => {
                let captures = Regex::captures(&DATETIME_REGEX, val).ok_or_else(|| {
                    FhirpathError::CompileError {
                        msg: format!("{} is not a DateTime", val),
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

                let precision = match (seconds, minutes, hours, days, months, years) {
                    (Some(_sec), _, _, _, _, _) => Ok(DateTimePrecision::Seconds),
                    (None, Some(_min), _, _, _, _) => Ok(DateTimePrecision::Minutes),
                    (None, None, Some(_h), _, _, _) => Ok(DateTimePrecision::Hours),
                    (None, None, None, Some(_day), _, _) => Ok(DateTimePrecision::Days),
                    (None, None, None, None, Some(_mon), _) => Ok(DateTimePrecision::Months),
                    (None, None, None, None, None, Some(_yr)) => Ok(DateTimePrecision::Years),
                    (None, None, None, None, None, None) => Err(FhirpathError::CompileError {
                        msg: "Invalid DateTime precision".to_string(),
                    }),
                }?;

                let mut date_format = precision.get_format().to_string();

                if fractional_secs {
                    date_format.extend("%.f".chars());
                }

                if tz.is_some() {
                    date_format.extend("%z".chars());
                }

                let offset = Offset::from_components(tz, tz_dir, tz_hours, tz_minutes);

                let adjusted_datetime = NaiveDateTime::parse_from_str(val, &date_format)
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
            }
            _ => Err(FhirpathError::CompileError {
                msg: "Only parsing times from strings is supported".to_string(),
            }),
        }
    }
}

pub enum ArithmeticType {
    Number(Number),
    Quantity(Quantity),
    Time(Time),
    DateTime(DateTime),
}

impl TryFrom<&Value> for ArithmeticType {
    type Error = FhirpathError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(num) => Ok(ArithmeticType::Number(num.clone())),
            Value::String(string_num) => {
                let num = string_num
                    .parse::<f64>()
                    .map_err(|_| FhirpathError::CompileError {
                        msg: "String was not a number".to_string(),
                    })?;

                let num_value =
                    Number::from_f64(num).ok_or_else(|| FhirpathError::CompileError {
                        msg: "Failed to convert f64 to Number".to_string(),
                    })?;

                Ok(ArithmeticType::Number(num_value))
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::Value;

    use crate::evaluate::types::{Date, DateTime, DateTimePrecision, TimePrecision};

    use super::Time;

    #[test]
    fn test_parse_basic_time() {
        let time = Time::try_from(&Value::String("@T10:11:12".to_string())).unwrap();

        assert_eq!(
            time,
            Time {
                hours: Some(10),
                minutes: Some(11),
                seconds: Some(12.0),
                precision: TimePrecision::Seconds
            }
        )
    }

    #[test]
    fn test_parse_basic_datetime() {
        let datetime =
            DateTime::try_from(&Value::String("@2025-01-01T10:11:12.123+02:00".to_string()))
                .unwrap();

        assert_eq!(
            datetime,
            DateTime {
                date: Date {
                    years: Some(2025),
                    months: Some(01),
                    days: Some(01)
                },
                time: Some(Time {
                    hours: Some(8),
                    minutes: Some(11),
                    seconds: Some(12.123),
                    precision: TimePrecision::Seconds
                }),
                precision: DateTimePrecision::Seconds
            }
        )
    }
}
