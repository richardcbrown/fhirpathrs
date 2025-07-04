use std::{cell::LazyCell, cmp::Ordering};

use chrono::{Datelike, Months, NaiveDateTime, TimeDelta, Utc};
use regex::Regex;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{error::FhirpathError, evaluate::EvaluateResult};
use crate::evaluate::data_types::time::TimePrecision;
use super::{
    date::Date,
    offset::{get_fixed_offset, Offset},
    quantity::{Quantity, CalendarUnit},
    time::Time,
    utils::parse_optional_u32,
};

/**
 * Capture groups:
 * 1. Years
 * 3. Months
 * 5. Days
 * 7. Hours
 * 9. Minutes
 * 11. Seconds + Millis
 * 12. Seconds
 * 14. Millis
 * 16. Timezone direction
 * 17. Timezone hours
 * 18. Timezone minutes
 */
const DATETIME_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new("@([0-9][0-9][0-9][0-9])(\\-([0-9][0-9])(\\-([0-9][0-9])(T([0-9][0-9])(:([0-9][0-9])(:(([0-9][0-9])(\\.([0-9]+))?))?)?(Z|(\\+|\\-)([0-9][0-9]):([0-9][0-9]))?)?)?)?").expect("Invalid DateTime Regex")
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateTime {
    pub precision: DateTimePrecision,
    pub date: Date,
    pub time: Option<Time>,
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // we can compare second and millisecond precision
        let is_sec_milli_comparison = (self.precision == DateTimePrecision::Seconds && other.precision == DateTimePrecision::Millis)
            || (self.precision == DateTimePrecision::Millis &&  other.precision == DateTimePrecision::Seconds);

        if self.precision != other.precision {
            if !is_sec_milli_comparison {
                return None;
            }
        }

        let cmp = self.date.partial_cmp(&other.date)?;

        match cmp {
            Ordering::Equal => match (&self.time, &other.time) {
                (None, None) => Some(Ordering::Equal),
                (Some(t1), Some(t2)) => {
                    let mut first_time = t1.clone();
                    let mut second_time = t2.clone();

                    // convert the second precision time
                    // to milli precision time and compare
                    if is_sec_milli_comparison {
                        if first_time.precision == TimePrecision::Seconds {
                            first_time.precision = TimePrecision::Millis;
                            first_time.millis = Some(0);
                        } else {
                            second_time.precision = TimePrecision::Millis;
                            second_time.millis = Some(0);
                        }
                    }
                    
                    first_time.partial_cmp(&second_time)
                },
                _ => None,
            },
            _ => return Some(cmp),
        }
    }
}

impl DateTime {
    pub fn to_datetime(&self) -> Option<chrono::DateTime<Utc>> {
        let date = self.date.to_date()?;
        let time = self
            .time
            .as_ref()
            .and_then(|t| t.to_time())
            .or(chrono::NaiveTime::from_hms_opt(0, 0, 0))?;

        Some(chrono::NaiveDateTime::new(date, time).and_utc())
    }

    pub fn try_add(&self, q: &Quantity) -> EvaluateResult<DateTime> {
        let datetime = self.to_datetime().ok_or(FhirpathError::EvaluateError {
            msg: "Invalid DateTime".to_string(),
        })?;

        let unit = q.unit.as_ref().ok_or(FhirpathError::EvaluateError {
            msg: "Cannot add unitless Quantity to DateTime".to_string(),
        })?;

        let time_unit = CalendarUnit::try_from(unit)?;

        let updated_dt = match time_unit {
            CalendarUnit::Years => {
                let int_year =
                    i32::try_from(q.value.trunc()).map_err(|err| FhirpathError::EvaluateError {
                        msg: format!("Could not parse as Integer: {}", err.to_string()),
                    })?;

                let new_year =
                    datetime
                        .year()
                        .checked_add(int_year)
                        .ok_or(FhirpathError::EvaluateError {
                            msg: "Could not add years".to_string(),
                        })?;

                datetime
                    .with_year(new_year)
                    .ok_or(FhirpathError::EvaluateError {
                        msg: format!("Could not add {} years to {}", int_year, datetime),
                    })
            }
            CalendarUnit::Months => {
                let months =
                    i32::try_from(q.value.trunc()).map_err(|err| FhirpathError::EvaluateError {
                        msg: format!("Could not parse as Integer: {}", err.to_string()),
                    })?;

                let abs_months: u32 = months.unsigned_abs();

                match months > 0 {
                    true => datetime.checked_add_months(Months::new(abs_months)),
                    false => datetime.checked_sub_months(Months::new(abs_months)),
                }
                .ok_or(FhirpathError::EvaluateError {
                    msg: format!("Could not add {} months to {}", months, datetime),
                })
            }
            CalendarUnit::Weeks => {
                let weeks =
                    i64::try_from(q.value.trunc()).map_err(|err| FhirpathError::EvaluateError {
                        msg: format!("Could not parse as Integer: {}", err.to_string()),
                    })?;

                let delta = TimeDelta::try_weeks(weeks).ok_or(FhirpathError::EvaluateError {
                    msg: format!("could not create TimeDelta from {} weeks", weeks),
                })?;

                datetime
                    .checked_add_signed(delta)
                    .ok_or(FhirpathError::EvaluateError {
                        msg: format!("Could not add {} weeks to {}", weeks, datetime),
                    })
            }
            CalendarUnit::Days => {
                let days =
                    i32::try_from(q.value.trunc()).map_err(|err| FhirpathError::EvaluateError {
                        msg: format!("Could not parse as Integer: {}", err.to_string()),
                    })?;

                let abs_days: u32 = days.unsigned_abs();

                match days > 0 {
                    true => datetime.checked_add_months(Months::new(abs_days)),
                    false => datetime.checked_sub_months(Months::new(abs_days)),
                }
                .ok_or(FhirpathError::EvaluateError {
                    msg: format!("Could not add {} days to {}", days, datetime),
                })
            }
            CalendarUnit::Hours => {
                let hours =
                    i64::try_from(q.value.trunc()).map_err(|err| FhirpathError::EvaluateError {
                        msg: format!("Could not parse as Integer: {}", err.to_string()),
                    })?;

                let delta = TimeDelta::try_hours(hours).ok_or(FhirpathError::EvaluateError {
                    msg: format!("could not create TimeDelta from {} hours", hours),
                })?;

                datetime
                    .checked_add_signed(delta)
                    .ok_or(FhirpathError::EvaluateError {
                        msg: format!("Could not add {} hours to {}", hours, datetime),
                    })
            }
            CalendarUnit::Minutes => {
                let minutes =
                    i64::try_from(q.value.trunc()).map_err(|err| FhirpathError::EvaluateError {
                        msg: format!("Could not parse as Integer: {}", err.to_string()),
                    })?;

                let delta = TimeDelta::try_minutes(minutes).ok_or(FhirpathError::EvaluateError {
                    msg: format!("could not create TimeDelta from {} minutes", minutes),
                })?;

                datetime
                    .checked_add_signed(delta)
                    .ok_or(FhirpathError::EvaluateError {
                        msg: format!("Could not add {} minutes to {}", minutes, datetime),
                    })
            }
            CalendarUnit::Seconds => {
                let frac_seconds =
                    Decimal::try_from(q.value).map_err(|err| FhirpathError::EvaluateError {
                        msg: format!("Could not parse as Integer: {}", err.to_string()),
                    })?;

                let seconds: i64 = i64::try_from(frac_seconds.trunc()).map_err(|err| {
                    FhirpathError::EvaluateError {
                        msg: format!("Could not convert Decimal to i64: {}", err.to_string()),
                    }
                })?;

                let millis: i64 =
                    i64::try_from(frac_seconds.round_dp(3).fract()).map_err(|err| {
                        FhirpathError::EvaluateError {
                            msg: format!("Could not convert Decimal to i64: {}", err.to_string()),
                        }
                    })?;

                let sec_delta =
                    TimeDelta::try_seconds(seconds).ok_or(FhirpathError::EvaluateError {
                        msg: format!("could not create TimeDelta from {} seconds", seconds),
                    })?;

                let millis_delta =
                    TimeDelta::try_milliseconds(millis).ok_or(FhirpathError::EvaluateError {
                        msg: format!("could not create TimeDelta from {} milliseconds", millis),
                    })?;

                datetime
                    .checked_add_signed(sec_delta)
                    .and_then(|dt| dt.checked_add_signed(millis_delta))
                    .ok_or(FhirpathError::EvaluateError {
                        msg: format!("Could not add {} seconds to {}", seconds, datetime),
                    })
            }
            CalendarUnit::Millis => {
                let millis =
                    i64::try_from(q.value.round()).map_err(|err| FhirpathError::EvaluateError {
                        msg: format!("Could not parse as Integer: {}", err.to_string()),
                    })?;

                let delta = TimeDelta::try_seconds(millis).ok_or(FhirpathError::EvaluateError {
                    msg: format!("could not create TimeDelta from {} seconds", millis),
                })?;

                datetime
                    .checked_add_signed(delta)
                    .ok_or(FhirpathError::EvaluateError {
                        msg: format!("Could not add {} seconds to {}", millis, datetime),
                    })
            }
        }?;

        Ok(DateTime::from_datetime_precision(
            updated_dt,
            &self.precision,
        ))
    }

    pub fn try_sub(&self, q: &Quantity) -> EvaluateResult<DateTime> {
        let mut negative_q = q.clone();

        negative_q.value = -negative_q.value;

        self.try_add(&negative_q)
    }

    pub fn to_string(&self) -> String {
        match &self.time {
            Some(time) => format!("{}T{}", self.date.to_string(), time.to_string()),
            None => format!("{}", self.date.to_string()),
        }
    }

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
        millis: Option<u32>,
        seconds: Option<u32>,
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
            time: Time::from_components(millis, seconds, minutes, hours, &precision),
        }
    }
}

impl TryFrom<&Value> for DateTime {
    type Error = FhirpathError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(val) => DateTime::try_from(val),
            _ => Err(FhirpathError::EvaluateError {
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
            FhirpathError::EvaluateError {
                msg: format!("{} is not a DateTime", value),
            }
        })?;

        let years = parse_optional_u32(captures.get(1))?;
        let months = parse_optional_u32(captures.get(3))?;
        let days = parse_optional_u32(captures.get(5))?;

        let hours = parse_optional_u32(captures.get(7))?;
        let minutes = parse_optional_u32(captures.get(9))?;

        let seconds = parse_optional_u32(captures.get(12))?;
        let millis = parse_optional_u32(captures.get(14))?;

        let tz = captures.get(15).and_then(|val| Some(val.as_str()));
        let tz_dir = captures.get(16).and_then(|val| Some(val.as_str()));
        let tz_hours = parse_optional_u32(captures.get(17))?;
        let tz_minutes = parse_optional_u32(captures.get(18))?;

        let precision = DateTimePrecision::from_components(
            millis, seconds, minutes, hours, days, months, years,
        )?;

        if tz.is_some() {
            let mut date_format = "@%Y-%m-%dT%H:%M:%S".to_string();

            if millis.is_some() {
                date_format.extend("%.f".chars());
            }

            date_format.extend("%z".chars());

            let offset = Offset::from_components(tz, tz_dir, tz_hours, tz_minutes);

            let adjusted_datetime = NaiveDateTime::parse_from_str(&value, &date_format)
                .map_err(|err| FhirpathError::EvaluateError {
                    msg: format!("Failed to parse DateTime: {}", err.to_string()),
                })?
                .and_local_timezone(get_fixed_offset(&offset)?)
                .single()
                .ok_or_else(|| FhirpathError::EvaluateError {
                    msg: "Failed to map DateTime".to_string(),
                })?
                .to_utc();

            Ok(Self::from_datetime_precision(adjusted_datetime, &precision))
        } else {
            Ok(Self::from_components(
                millis, seconds, minutes, hours, days, months, years, precision,
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
    Millis,
}

impl DateTimePrecision {
    fn from_components(
        millis: Option<u32>,
        seconds: Option<u32>,
        minutes: Option<u32>,
        hours: Option<u32>,
        days: Option<u32>,
        months: Option<u32>,
        years: Option<u32>,
    ) -> EvaluateResult<DateTimePrecision> {
        match (millis, seconds, minutes, hours, days, months, years) {
            (Some(_millis), _, _, _, _, _, _) => Ok(DateTimePrecision::Millis),
            (None, Some(_sec), _, _, _, _, _) => Ok(DateTimePrecision::Seconds),
            (None, None, Some(_min), _, _, _, _) => Ok(DateTimePrecision::Minutes),
            (None, None, None, Some(_h), _, _, _) => Ok(DateTimePrecision::Hours),
            (None, None, None, None, Some(_day), _, _) => Ok(DateTimePrecision::Days),
            (None, None, None, None, None, Some(_mon), _) => Ok(DateTimePrecision::Months),
            (None, None, None, None, None, None, Some(_yr)) => Ok(DateTimePrecision::Years),
            _ => Err(FhirpathError::EvaluateError {
                msg: "Invalid DateTime precision".to_string(),
            }),
        }
    }
}
