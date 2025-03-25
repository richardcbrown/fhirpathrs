use std::{cell::LazyCell, cmp::Ordering};

use chrono::{Timelike, Utc};
use regex::Regex;
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
 */
const TIME_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new("@T([0-9][0-9])(:([0-9][0-9])(:([0-9][0-9](\\.([0-9]+))?)?)?)?")
        .expect("Invalid Time Regex")
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Time {
    pub hours: Option<u32>,
    pub minutes: Option<u32>,
    pub seconds: Option<f64>,
    pub precision: TimePrecision,
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

    pub fn from_components(
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
pub enum TimePrecision {
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
