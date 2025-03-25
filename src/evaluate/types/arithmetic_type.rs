use std::str::FromStr;

use rust_decimal::Decimal;
use serde_json::Value;

use crate::error::FhirpathError;

use super::{date_time::DateTime, quantity::Quantity, time::Time};

#[derive(Clone)]
pub enum ArithmeticType {
    Number(Decimal),
    Quantity(Quantity),
    Time(Time),
    DateTime(DateTime),
    String(String),
}

impl TryFrom<&Value> for ArithmeticType {
    type Error = FhirpathError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(num) => Ok(ArithmeticType::Number(
                Decimal::from_str(num.as_str()).map_err(|_| FhirpathError::CompileError {
                    msg: "Failed to convert Number to Decimal".to_string(),
                })?,
            )),
            Value::String(string_val) => {
                let num = Decimal::from_str(&string_val);

                if let Ok(num_value) = num {
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
