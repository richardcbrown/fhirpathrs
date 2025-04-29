use std::{cell::LazyCell, cmp::Ordering};

use chrono::{NaiveTime, Timelike, Utc};
use regex::Regex;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{error::FhirpathError, evaluate::CompileResult};

use super::{
    date_time::DateTimePrecision,
    utils::{parse_optional_f64, parse_optional_u32},
};

/**
 * Capture groups:
 * 1. Hours
 * 3. Minutes
 * 5. Seconds + Millis
 * 6. Seconds
 * 8. Millis
 */
const TIME_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new("@T([0-9][0-9])(:([0-9][0-9])(:(([0-9][0-9])(\\.([0-9]+))?)?)?)?")
        .expect("Invalid Time Regex")
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Time {
    pub hours: Option<u32>,
    pub minutes: Option<u32>,
    pub seconds: Option<u32>,
    pub millis: Option<u32>,
    pub precision: TimePrecision,
}

impl Time {
    pub fn to_string(&self) -> String {
        match (&self.millis, &self.seconds, &self.minutes, &self.hours) {
            (Some(mil), Some(s), Some(m), Some(h)) => {
                format!("{:0>2}:{:0>2}:{:0>2}.{mil}", h, m, s)
            }
            (None, Some(s), Some(m), Some(h)) => {
                format!("{:0>2}:{:0>2}:{:0>2}", h, m, s)
            }
            (None, None, Some(m), Some(h)) => format!("{:0>2}:{:0>2}", h, m),
            (None, None, None, Some(h)) => h.to_string(),
            _ => "".to_string(),
        }
    }

    pub fn to_time_string(&self) -> String {
        format!("@T{}", self.to_string())
    }

    pub fn to_time(&self) -> Option<NaiveTime> {
        let sec: u32 = self.seconds.unwrap_or(0);

        let nano: u32 = self.millis.unwrap_or(0) * 1000 * 1000;

        let min = self.minutes.unwrap_or(0);

        let hour = self.hours.unwrap_or(0);

        NaiveTime::from_hms_nano_opt(hour, min, sec, nano)
    }
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
    pub fn from_datetime_precision(
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
                millis: None,
                precision: TimePrecision::Hours,
            }),
            DateTimePrecision::Minutes => Some(Time {
                hours: Some(dt.hour()),
                minutes: Some(dt.minute()),
                seconds: None,
                millis: None,
                precision: TimePrecision::Minutes,
            }),
            DateTimePrecision::Seconds => Some(Time {
                hours: Some(dt.hour()),
                minutes: Some(dt.minute()),
                seconds: Some(dt.second()),
                millis: Some(dt.nanosecond() / (1000 * 1000)),
                precision: TimePrecision::Seconds,
            }),
        }
    }

    pub fn from_components(
        millis: Option<u32>,
        seconds: Option<u32>,
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
                millis: None,
            }),
            DateTimePrecision::Minutes => Some(Time {
                precision: TimePrecision::Minutes,
                hours,
                minutes,
                seconds: None,
                millis: None,
            }),
            DateTimePrecision::Seconds => Some(Time {
                precision: TimePrecision::Seconds,
                hours,
                minutes,
                seconds,
                millis,
            }),
        }
    }
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
        let time_string = match value.starts_with("@") {
            true => value.clone(),
            false => format!("@T{}", value),
        };

        let captures = Regex::captures(&TIME_REGEX, &time_string).ok_or_else(|| {
            FhirpathError::CompileError {
                msg: format!("{} is not a Time", value),
            }
        })?;

        let hours = parse_optional_u32(captures.get(1))?;
        let minutes = parse_optional_u32(captures.get(3))?;
        let seconds = parse_optional_u32(captures.get(6))?;
        let millis = parse_optional_u32(captures.get(8))?;

        let precision = TimePrecision::from_components(millis, seconds, minutes, hours)?;

        Ok(Self {
            hours,
            minutes,
            seconds,
            millis,
            precision,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimePrecision {
    Hours,
    Minutes,
    Seconds,
}

impl TimePrecision {
    fn from_components(
        millis: Option<u32>,
        seconds: Option<u32>,
        minutes: Option<u32>,
        hours: Option<u32>,
    ) -> CompileResult<Self> {
        match (millis, seconds, minutes, hours) {
            (Some(_millis), Some(_sec), _, _) => Ok(TimePrecision::Seconds),
            (None, Some(_sec), _, _) => Ok(TimePrecision::Seconds),
            (None, None, Some(_min), _) => Ok(TimePrecision::Minutes),
            (None, None, None, Some(_hr)) => Ok(TimePrecision::Hours),
            _ => Err(FhirpathError::CompileError {
                msg: "Invalid Time precision".to_string(),
            }),
        }
    }
}
