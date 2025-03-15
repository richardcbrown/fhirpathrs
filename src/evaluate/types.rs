use std::{cell::LazyCell, cmp::Ordering, fmt::format};

use chrono::{Datelike, FixedOffset, Local, NaiveDateTime, Timelike, Utc};
use regex::{Match, Regex};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use crate::{error::FhirpathError, parser::literal::QuantityLiteral};

use super::CompileResult;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum DateTimePrecision {
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quantity {
    value: f64,
    unit: Option<String>,
}

impl TryFrom<&QuantityLiteral> for Quantity {
    type Error = FhirpathError;

    fn try_from(value: &QuantityLiteral) -> Result<Self, Self::Error> {
        let quant_value = value
            .text
            .parse::<f64>()
            .map_err(|err| FhirpathError::CompileError {
                msg: format!("Failed to parse Quantity.value: {}", err.to_string()),
            })?;

        Ok(Self {
            value: quant_value,
            unit: value.unit.clone(),
        })
    }
}

impl Quantity {
    fn try_convert_unit(&self, unit: &Option<String>) -> CompileResult<Quantity> {
        match (&self.unit, unit) {
            (None, None) => Ok(self.clone()),
            (Some(u1), Some(u2)) => {
                if u1.eq(u2) {
                    return Ok(self.clone());
                }

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

impl PartialOrd for Quantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // attempt to convert self so units match with other
        let converted = self.try_convert_unit(&other.unit).ok()?;

        // units match so only need to compare values
        converted.value.partial_cmp(&other.value)
    }
}

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Time {
    hours: Option<u32>,
    minutes: Option<u32>,
    seconds: Option<f64>,
    precision: TimePrecision,
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.precision != other.precision {
            return None;
        }

        match (self.hours, other.hours) {
            (Some(self_hours), Some(other_hours)) => {
                if self_hours > other_hours {
                    return Some(Ordering::Greater);
                }

                if self_hours < other_hours {
                    return Some(Ordering::Less);
                }
            }
            (None, None) => {}
            _ => return None,
        }

        match (self.minutes, other.minutes) {
            (Some(self_minutes), Some(other_minutes)) => {
                if self_minutes > other_minutes {
                    return Some(Ordering::Greater);
                }

                if self_minutes < other_minutes {
                    return Some(Ordering::Less);
                }
            }
            (None, None) => {}
            _ => return None,
        }

        match (self.seconds, other.seconds) {
            (Some(self_seconds), Some(other_seconds)) => {
                if self_seconds > other_seconds {
                    return Some(Ordering::Greater);
                }

                if self_seconds < other_seconds {
                    return Some(Ordering::Less);
                }
            }
            (None, None) => {}
            _ => return None,
        }

        return Some(Ordering::Equal);
    }
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

    fn from_components(
        seconds: Option<f64>,
        minutes: Option<u32>,
        hours: Option<u32>,
        precision: &DateTimePrecision,
    ) -> Option<Time> {
        match precision {
            DateTimePrecision::Years => None,
            DateTimePrecision::Months => None,
            DateTimePrecision::Days => None,
            DateTimePrecision::Hours => Some(Time {
                precision: TimePrecision::Hours,
                hours,
                minutes: None,
                seconds: None,
            }),
            DateTimePrecision::Minutes => Some(Time {
                precision: TimePrecision::Minutes,
                hours,
                minutes,
                seconds: None,
            }),
            DateTimePrecision::Seconds => Some(Time {
                precision: TimePrecision::Seconds,
                hours,
                minutes,
                seconds,
            }),
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
            Value::String(val) => Time::try_from(val),
            _ => Err(FhirpathError::CompileError {
                msg: "Only parsing times from strings is supported".to_string(),
            }),
        }
    }
}

impl TryFrom<&String> for Time {
    type Error = FhirpathError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let captures =
            Regex::captures(&TIME_REGEX, value).ok_or_else(|| FhirpathError::CompileError {
                msg: format!("{} is not a Time", value),
            })?;

        let hours = parse_optional_u32(captures.get(1))?;
        let minutes = parse_optional_u32(captures.get(3))?;
        let seconds = parse_optional_f64(captures.get(5))?;

        let precision = TimePrecision::from_components(seconds, minutes, hours)?;

        Ok(Self {
            hours,
            minutes,
            seconds,
            precision,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Date {
    years: Option<u32>,
    months: Option<u32>,
    days: Option<u32>,
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.years, other.years) {
            (Some(self_years), Some(other_years)) => {
                if self_years > other_years {
                    return Some(Ordering::Greater);
                }

                if self_years < other_years {
                    return Some(Ordering::Less);
                }
            }
            (None, None) => {}
            _ => return None,
        }

        match (self.months, other.months) {
            (Some(self_months), Some(other_months)) => {
                if self_months > other_months {
                    return Some(Ordering::Greater);
                }

                if self_months < other_months {
                    return Some(Ordering::Less);
                }
            }
            (None, None) => {}
            _ => return None,
        }

        match (self.days, other.days) {
            (Some(self_days), Some(other_days)) => {
                if self_days > other_days {
                    return Some(Ordering::Greater);
                }

                if self_days < other_days {
                    return Some(Ordering::Less);
                }
            }
            (None, None) => {}
            _ => return None,
        }

        Some(Ordering::Equal)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateTime {
    precision: DateTimePrecision,
    date: Date,
    time: Option<Time>,
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

    fn from_components(
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

#[derive(Clone)]
pub enum ArithmeticType {
    Number(Number),
    Quantity(Quantity),
    Time(Time),
    DateTime(DateTime),
    String(String),
}

pub fn implicit_convert(
    first: ArithmeticType,
    second: ArithmeticType,
) -> (ArithmeticType, ArithmeticType) {
    match (&first, &second) {
        // can implicitly convert num to Quantity
        (ArithmeticType::Number(num), ArithmeticType::Quantity(_)) => match num.as_f64() {
            Some(parsed) => (
                ArithmeticType::Quantity(Quantity {
                    value: parsed,
                    unit: None,
                }),
                second.clone(),
            ),
            None => (first, second),
        },
        (ArithmeticType::Quantity(_), ArithmeticType::Number(num)) => match num.as_f64() {
            Some(parsed) => (
                first.clone(),
                ArithmeticType::Quantity(Quantity {
                    value: parsed,
                    unit: None,
                }),
            ),
            None => (first, second),
        },
        _ => (first, second),
    }
}

impl TryFrom<&Value> for ArithmeticType {
    type Error = FhirpathError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(num) => Ok(ArithmeticType::Number(num.clone())),
            Value::String(string_val) => {
                let num = string_val
                    .parse::<f64>()
                    .ok()
                    .and_then(|num| Number::from_f64(num));

                if let Some(num_value) = num {
                    return Ok(ArithmeticType::Number(num_value));
                }

                if let Some(datetime_value) = DateTime::try_from(string_val).ok() {
                    return Ok(ArithmeticType::DateTime(datetime_value));
                }

                Ok(ArithmeticType::String(string_val.to_string()))
            }
            Value::Object(_) => {
                let datetime: Option<DateTime> = serde_json::from_value(value.clone()).ok();

                if let Some(datetime_value) = datetime {
                    return Ok(ArithmeticType::DateTime(datetime_value));
                }

                let time: Option<Time> = serde_json::from_value(value.clone()).ok();

                if let Some(time_value) = time {
                    return Ok(ArithmeticType::Time(time_value));
                }

                let quantity: Option<Quantity> = serde_json::from_value(value.clone()).ok();

                if let Some(quantity_value) = quantity {
                    return Ok(ArithmeticType::Quantity(quantity_value));
                }

                Err(FhirpathError::CompileError {
                    msg: "Could not convert object to ArithmeticType".to_string(),
                })
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

    #[test]
    fn test_datetime_cmp_none() {
        let result =
            DateTime::try_from(&Value::String("@2025-01-01T10:11:12.123+02:00".to_string()))
                .unwrap()
                .partial_cmp(
                    &DateTime::try_from(&Value::String("@2025-01-01T10:11".to_string())).unwrap(),
                );

        assert_eq!(result, None)
    }

    #[test]
    fn test_datetime_cmp_greater() {
        let result =
            DateTime::try_from(&Value::String("@2025-01-01T10:11:12.123+01:00".to_string()))
                .unwrap()
                .partial_cmp(
                    &DateTime::try_from(&Value::String(
                        "@2025-01-01T10:11:12.123+02:00".to_string(),
                    ))
                    .unwrap(),
                );

        assert_eq!(result, Some(std::cmp::Ordering::Greater))
    }

    #[test]
    fn test_datetime_cmp_lesser() {
        let result =
            DateTime::try_from(&Value::String("@2025-01-01T10:11:12.123+02:00".to_string()))
                .unwrap()
                .partial_cmp(
                    &DateTime::try_from(&Value::String(
                        "@2025-01-01T10:11:12.123+01:00".to_string(),
                    ))
                    .unwrap(),
                );

        assert_eq!(result, Some(std::cmp::Ordering::Less))
    }

    #[test]
    fn test_datetime_cmp_equal() {
        let result = DateTime::try_from(&Value::String("@2025-01-01".to_string()))
            .unwrap()
            .partial_cmp(&DateTime::try_from(&Value::String("@2025-01-01".to_string())).unwrap());

        assert_eq!(result, Some(std::cmp::Ordering::Equal))
    }
}
