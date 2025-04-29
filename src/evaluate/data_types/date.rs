use std::cmp::Ordering;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Date {
    pub years: Option<u32>,
    pub months: Option<u32>,
    pub days: Option<u32>,
}

impl Date {
    pub fn to_string(&self) -> String {
        match (&self.days, &self.months, &self.years) {
            (Some(d), Some(m), Some(y)) => format!("{}-{:0>2}-{:0>2}", y, m, d),
            (None, Some(m), Some(y)) => format!("{}-{:0>2}", y, m),
            (None, None, Some(y)) => y.to_string(),
            _ => "".to_string(),
        }
    }

    pub fn to_date_string(&self) -> String {
        format!("@{}", self.to_string())
    }

    pub fn to_date(&self) -> Option<NaiveDate> {
        let year = i32::try_from(self.years?).ok()?;
        let month = self.months.unwrap_or(1);
        let day = self.days.unwrap_or(1);

        chrono::NaiveDate::from_ymd_opt(year, month, day)
    }
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
