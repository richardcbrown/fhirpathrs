use regex::Match;

use crate::{error::FhirpathError, evaluate::EvaluateResult};

use super::{arithmetic_type::ArithmeticType, quantity::Quantity};

pub fn parse_optional_u32(capture: Option<Match>) -> EvaluateResult<Option<u32>> {
    Ok(capture.and_then(|val| Some(val.as_str()))).and_then(|hour| match hour {
        Some(h) => {
            let parse_result = h.parse::<u32>().map_err(|_| FhirpathError::EvaluateError {
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

pub fn implicit_convert(
    first: &ArithmeticType,
    second: &ArithmeticType,
) -> (ArithmeticType, ArithmeticType) {
    match (first, second) {
        // can implicitly convert num to Quantity
        (ArithmeticType::Number(num), ArithmeticType::Quantity(_)) => (
            ArithmeticType::Quantity(Quantity {
                value: num.clone(),
                unit: None,
            }),
            second.clone(),
        ),
        (ArithmeticType::Quantity(_), ArithmeticType::Number(num)) => (
            first.clone(),
            ArithmeticType::Quantity(Quantity {
                value: num.clone(),
                unit: None,
            }),
        ),
        _ => (first.clone(), second.clone()),
    }
}
