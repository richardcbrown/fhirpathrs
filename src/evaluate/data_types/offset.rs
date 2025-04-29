use chrono::{FixedOffset, Local};

use crate::{error::FhirpathError, evaluate::CompileResult};

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
    pub fn from_components(
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

pub fn get_fixed_offset(offset: &Option<Offset>) -> CompileResult<FixedOffset> {
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
