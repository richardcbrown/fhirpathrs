use std::{cmp::Ordering, str::FromStr};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{error::FhirpathError, evaluate::CompileResult, parser::literal::QuantityLiteral};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quantity {
    pub value: Decimal,
    pub unit: Option<String>,
}

impl TryFrom<&QuantityLiteral> for Quantity {
    type Error = FhirpathError;

    fn try_from(value: &QuantityLiteral) -> Result<Self, Self::Error> {
        let quant_value =
            Decimal::from_str(value.text.as_str()).map_err(|err| FhirpathError::CompileError {
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
