use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal, MathematicalOps,
};
use rust_decimal::prelude::Signed;
use serde_json::{json, Value};

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    data_types::{arithmetic_type::ArithmeticType, utils::implicit_convert},
    utils::{from_decimal, get_f64_from_expression, get_usize_from_expression},
    EvaluateResult, Evaluate, ResourceNode,
};

impl ArithmeticType {
    pub fn to_string(&self) -> String {
        match self {
            ArithmeticType::DateTime(dt) => dt.to_string(),
            ArithmeticType::Number(num) => num.to_string(),
            ArithmeticType::Quantity(q) => q.to_string(),
            ArithmeticType::String(s) => s.to_string(),
            ArithmeticType::Time(t) => t.to_string(),
        }
    }

    pub fn mul(&self, other: &ArithmeticType) -> EvaluateResult<Value> {
        let (first, second) = implicit_convert(self, other);

        match (first, second) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => Ok(json!(num1 * num2)),
            (ArithmeticType::Quantity(q1), ArithmeticType::Quantity(q2)) => {
                let result = (q1 * q2)
                    .and_then(|res| serde_json::to_value(res).ok())
                    .unwrap_or(Value::Array(vec![]));

                Ok(result)
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: format!(
                    "Cannot multiply {} by {}",
                    self.to_string(),
                    other.to_string()
                ),
            }),
        }
    }

    pub fn add(&self, other: &ArithmeticType) -> EvaluateResult<Value> {
        let (first, second) = implicit_convert(self, other);

        match (first, second) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => Ok(json!(num1 + num2)),
            (ArithmeticType::Quantity(q1), ArithmeticType::Quantity(q2)) => {
                let result = (q1 + q2)
                    .and_then(|res| serde_json::to_value(res).ok())
                    .unwrap_or(Value::Array(vec![]));

                Ok(result)
            }
            (ArithmeticType::DateTime(dt), ArithmeticType::Quantity(q)) => {
                let result = dt.try_add(&q)?;

                Ok(Value::String(result.to_string()))
            }
            (ArithmeticType::String(s1), ArithmeticType::String(s2)) => {
                Ok(Value::String(format!("{}{}", s1, s2)))
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: format!("Cannot add {} to {}", self.to_string(), other.to_string()),
            }),
        }
    }

    pub fn sub(&self, other: &ArithmeticType) -> EvaluateResult<Value> {
        let (first, second) = implicit_convert(self, other);

        match (first, second) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => Ok(json!(num1 - num2)),
            (ArithmeticType::Quantity(q1), ArithmeticType::Quantity(q2)) => {
                let result = (q1 - q2)
                    .and_then(|res| serde_json::to_value(res).ok())
                    .unwrap_or(Value::Array(vec![]));

                Ok(result)
            }
            (ArithmeticType::DateTime(dt), ArithmeticType::Quantity(q)) => {
                let result = dt.try_sub(&q)?;

                Ok(Value::String(result.to_string()))
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: format!(
                    "Cannot subtract {} to {}",
                    self.to_string(),
                    other.to_string()
                ),
            }),
        }
    }

    pub fn div(&self, other: &ArithmeticType) -> EvaluateResult<Value> {
        match (self, other) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => {
                if num2.is_zero() {
                    return Ok(Value::Array(vec![]));
                }

                let f1: f64 = from_decimal(num1.to_owned())?;
                let f2: f64 = from_decimal(num2.to_owned())?;

                let result = Decimal::from_f64(f1.div_euclid(f2)).ok_or_else(|| {
                    FhirpathError::EvaluateError {
                        msg: "Failed to convert to Decimal".to_string(),
                    }
                })?;

                Ok(
                    serde_json::to_value(result).map_err(|err| FhirpathError::EvaluateError {
                        msg: format!("Could not convert from Decimal: {}", err.to_string()),
                    })?,
                )
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: "div operation not supported for type".to_string(),
            })
        }
    }

    pub fn rem(&self, other: &ArithmeticType) -> EvaluateResult<Value> {
        match (self, other) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => {
                let f1: f64 = from_decimal(num1.to_owned())?;
                let f2: f64 = from_decimal(num2.to_owned())?;

                let result = Decimal::from_f64(f1.rem_euclid(f2)).ok_or_else(|| {
                    FhirpathError::EvaluateError {
                        msg: "Failed to convert to Decimal".to_string(),
                    }
                })?;

                Ok(
                    serde_json::to_value(result).map_err(|err| FhirpathError::EvaluateError {
                        msg: format!("Could not convert from Decimal: {}", err.to_string()),
                    })?,
                )
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: "rem operation not supported for type".to_string(),
            })
        }
    }

    pub fn divide(&self, other: &ArithmeticType) -> EvaluateResult<Value> {
        match (self, other) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => {
                if num2.is_zero() {
                    return Ok(Value::Array(vec![]));
                }

                Ok(serde_json::to_value(num1 / num2).map_err(|err| {
                    FhirpathError::EvaluateError {
                        msg: format!("Could not convert from Decimal: {}", err.to_string()),
                    }
                })?)
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: "divide operation not supported for type".to_string(),
            })
        }
    }

    pub fn exp(&self) -> EvaluateResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                let f1 = from_decimal(num.to_owned())?;

                let result =
                    Decimal::from_f64(f1.exp()).ok_or_else(|| FhirpathError::EvaluateError {
                        msg: format!("Could not convert from Decimal"),
                    })?;

                Ok(
                    serde_json::to_value(result).map_err(|err| FhirpathError::EvaluateError {
                        msg: format!("Could not convert from Decimal: {}", err.to_string()),
                    })?,
                )
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: "exp operation not supported for type".to_string(),
            })
        }
    }

    pub fn abs(&self) -> EvaluateResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                Ok(
                    serde_json::to_value(num.abs()).map_err(|err| FhirpathError::EvaluateError {
                        msg: format!("Could not convert from Decimal: {}", err.to_string()),
                    })?,
                )
            },
            ArithmeticType::Quantity(quantity) => {
                let mut abs_quantity = quantity.clone();

                abs_quantity.value = abs_quantity.value.abs();

                Ok(Value::String(abs_quantity.to_string()))
            },
            _ => Err(FhirpathError::EvaluateError {
                msg: "abs operation not supported for type".to_string(),
            })
        }
    }

    pub fn ceiling(&self) -> EvaluateResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                Ok(
                    serde_json::to_value(num.ceil()).map_err(|err| {
                        FhirpathError::EvaluateError {
                            msg: format!("Could not convert from Decimal: {}", err.to_string()),
                        }
                    })?,
                )
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: "ceiling operation not supported for type".to_string(),
            })
        }
    }

    pub fn floor(&self) -> EvaluateResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                Ok(serde_json::to_value(num.floor()).map_err(|err| {
                    FhirpathError::EvaluateError {
                        msg: format!("Could not convert from Decimal: {}", err.to_string()),
                    }
                })?)
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: "floor operation not supported for type".to_string(),
            })
        }
    }

    pub fn ln(&self) -> EvaluateResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                Ok(
                    serde_json::to_value(num.ln()).map_err(|err| FhirpathError::EvaluateError {
                        msg: format!("Could not convert from Decimal: {}", err.to_string()),
                    })?,
                )
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: "ln operation not supported for type".to_string(),
            })
        }
    }

    pub fn log(&self, base: f64) -> EvaluateResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                let f1 = from_decimal(num.to_owned())?;

                let result =
                    Decimal::from_f64(f1.log(base)).ok_or_else(|| FhirpathError::EvaluateError {
                        msg: "Failed to convert to Decimal".to_string(),
                    })?;

                Ok(
                    serde_json::to_value(result).map_err(|err| FhirpathError::EvaluateError {
                        msg: format!("Could not convert from Decimal: {}", err.to_string()),
                    })?,
                )
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: "log operation not supported for type".to_string(),
            })
        }
    }

    pub fn power(&self, exponent: f64) -> EvaluateResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                // slightly naive approach to avoid complex numbers
                if num.is_sign_negative() && (exponent != 0.0 && exponent < 1.0) {
                    return Ok(Value::Array(vec![]));
                }

                let result = num.checked_powf(exponent);

                let value = match result {
                    Some(result) => serde_json::to_value(result).map_err(|err| {
                        FhirpathError::EvaluateError {
                            msg: format!("Could not convert from Decimal: {}", err.to_string()),
                        }
                    })?,
                    None => Value::Array(vec![]),
                };

                Ok(value)
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: "pow operation not supported for type".to_string(),
            })
        }
    }

    pub fn round(&self, precision: usize) -> EvaluateResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                let rounded = match precision {
                    0 => Ok(serde_json::to_value(num.round()).map_err(|err| {
                        FhirpathError::EvaluateError {
                            msg: format!("Could not convert from Decimal: {}", err.to_string()),
                        }
                    })?),
                    _ => {
                        let dp = precision
                            .to_u32()
                            .ok_or_else(|| FhirpathError::EvaluateError {
                                msg: format!("Could not convert to u32"),
                            })?;

                        let val = serde_json::to_value(num.round_dp(dp)).map_err(|err| {
                            FhirpathError::EvaluateError {
                                msg: format!("Could not convert from Decimal: {}", err.to_string()),
                            }
                        })?;

                        Ok(val)
                    }
                }?;

                Ok(rounded)
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: "round operation not supported for type".to_string(),
            })
        }
    }

    pub fn sqrt(&self) -> EvaluateResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                let root = num.sqrt();

                if let Some(root_val) = root {
                    return serde_json::to_value(root_val.normalize()).map_err(|err| {
                        FhirpathError::EvaluateError {
                            msg: format!("Could not convert from Decimal: {}", err.to_string()),
                        }
                    });
                }

                Ok(Value::Array(vec![]))
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: "sqrt operation not supported for type".to_string(),
            })
        }
    }

    pub fn truncate(&self) -> EvaluateResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                Ok(serde_json::to_value(num.trunc()).map_err(|err| {
                    FhirpathError::EvaluateError {
                        msg: format!("Could not convert from Decimal: {}", err.to_string()),
                    }
                })?)
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: "truncate operation not supported for type".to_string(),
            })
        }
    }
}

pub fn add<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::EvaluateError {
            msg: "add expects exactly two expressions".to_string(),
        });
    }

    let first = expressions[0].evaluate(input)?.get_single()?;
    let second = expressions[1].evaluate(input)?.get_single()?;

    if let Ok(result) = ArithmeticType::try_from(&first)?.add(&ArithmeticType::try_from(&second)?) {
        return Ok(ResourceNode::from_node(input, result));
    }

    if let (Value::String(mut first_string), Value::String(second_string)) = (first, second) {
        first_string.push_str(second_string.as_str());

        return Ok(ResourceNode::from_node(input, json!(first_string)));
    }

    Err(FhirpathError::EvaluateError {
        msg: "add operator not supported for types".to_string(),
    })
}

pub fn mul<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::EvaluateError {
            msg: "mul expects exactly two expressions".to_string(),
        });
    }

    let first = ArithmeticType::try_from(&expressions[0].evaluate(input)?.get_single()?)?;

    let second = ArithmeticType::try_from(&expressions[1].evaluate(input)?.get_single()?)?;

    Ok(ResourceNode::from_node(input, first.mul(&second)?))
}

pub fn sub<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::EvaluateError {
            msg: "sub expects exactly two expressions".to_string(),
        });
    }

    let first = ArithmeticType::try_from(&expressions[0].evaluate(input)?.get_single()?)?;

    let second = ArithmeticType::try_from(&expressions[1].evaluate(input)?.get_single()?)?;

    Ok(ResourceNode::from_node(input, first.sub(&second)?))
}

pub fn div<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::EvaluateError {
            msg: "sub expects exactly two expressions".to_string(),
        });
    }

    let first = ArithmeticType::try_from(&expressions[0].evaluate(input)?.get_single()?)?;

    let second = ArithmeticType::try_from(&expressions[1].evaluate(input)?.get_single()?)?;

    Ok(ResourceNode::from_node(input, first.div(&second)?))
}

pub fn rem<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::EvaluateError {
            msg: "sub expects exactly two expressions".to_string(),
        });
    }

    let first = ArithmeticType::try_from(&expressions[0].evaluate(input)?.get_single()?)?;

    let second = ArithmeticType::try_from(&expressions[1].evaluate(input)?.get_single()?)?;

    Ok(ResourceNode::from_node(input, first.rem(&second)?))
}

pub fn divide<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::EvaluateError {
            msg: "sub expects exactly two expressions".to_string(),
        });
    }

    let first = ArithmeticType::try_from(&expressions[0].evaluate(input)?.get_single()?)?;

    let second = ArithmeticType::try_from(&expressions[1].evaluate(input)?.get_single()?)?;

    Ok(ResourceNode::from_node(input, first.divide(&second)?))
}

pub fn amp<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::EvaluateError {
            msg: "sub expects exactly two expressions".to_string(),
        });
    }

    let first = expressions[0]
        .evaluate(input)?
        .get_single_or_empty()?
        .unwrap_or(Value::String("".to_string()));

    let second = expressions[1]
        .evaluate(input)?
        .get_single_or_empty()?
        .unwrap_or(Value::String("".to_string()));

    if let (Value::String(mut first_string), Value::String(second_string)) = (first, second) {
        first_string.push_str(second_string.as_str());

        return Ok(ResourceNode::from_node(input, json!(first_string)));
    }

    Err(FhirpathError::EvaluateError {
        msg: "add operator not supported for types".to_string(),
    })
}

pub fn abs<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.abs(),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

pub fn ceiling<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.ceiling(),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

pub fn exp<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.exp(),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

pub fn floor<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.floor(),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

pub fn ln<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.ln(),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

pub fn log<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if expressions.len() != 1 {
        return Err(FhirpathError::EvaluateError {
            msg: "log expects exactly one expression".to_string(),
        });
    }

    let first = get_f64_from_expression(input, &expressions[0])?;

    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.log(first),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

pub fn power<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if expressions.len() != 1 {
        return Err(FhirpathError::EvaluateError {
            msg: "log expects exactly one expression".to_string(),
        });
    }

    let first = get_f64_from_expression(input, &expressions[0])?;

    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.power(first),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

pub fn round<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if expressions.len() > 1 {
        return Err(FhirpathError::EvaluateError {
            msg: "log expects zero or one expression".to_string(),
        });
    }

    let first = match expressions.first() {
        Some(exp) => get_usize_from_expression(input, &exp),
        None => Ok(0),
    }?;

    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.round(first),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

pub fn sqrt<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.sqrt(),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

pub fn truncate<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.truncate(),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

#[cfg(test)]
mod test {
    use assert_json_diff::assert_json_eq;
    use rust_decimal_macros::dec;
    use serde_json::json;
    use crate::error::FhirpathError;
    use crate::evaluate::{
        compile,
        test::test::{run_tests, TestCase},
    };
    use crate::evaluate::test::test::Expected;

    #[test]
    fn evaluate_add_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": 2,
            "b": 6
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a + Patient.b".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([8])),
                options: None,
            },
            TestCase {
                path: "Patient.a + 1 year".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016"
                }),
                expected: Expected::Value(json!(["2017"])),
                options: None,
            },
            TestCase {
                path: "Patient.a + 1 month".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016"
                }),
                expected: Expected::Value(json!(["2016"])),
                options: None,
            },
            TestCase {
                path: "Patient.a + 12 month".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016"
                }),
                expected: Expected::Value(json!(["2017"])),
                options: None,
            },
            TestCase {
                path: "Patient.a + 2 month".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016-11"
                }),
                expected: Expected::Value(json!(["2017-01"])),
                options: None,
            },
            TestCase {
                path: "Patient.a + 1 week".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016"
                }),
                expected: Expected::Value(json!(["2016"])),
                options: None,
            },
            TestCase {
                path: "Patient.a + 53 week".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "@2016"
                }),
                expected: Expected::Value(json!(["2017"])),
                options: None,
            },
            TestCase {
                path: "Patient.a + 1 week".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016-12-30"
                }),
                expected: Expected::Value(json!(["2017-01-06"])),
                options: None,
            },
            TestCase {
                path: "Patient.a + 1 hour".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016"
                }),
                expected: Expected::Value(json!(["2016"])),
                options: None,
            },
            TestCase {
                path: "Patient.a + 8800 hour".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016"
                }),
                expected: Expected::Value(json!(["2017"])),
                options: None,
            },
            TestCase {
                path: "Patient.a + 24 hour".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016-12-31"
                }),
                expected: Expected::Value(json!(["2017-01-01"])),
                options: None,
            },
            TestCase {
                path: "Patient.a + 2 hour".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016-12-31T12"
                }),
                expected: Expected::Value(json!(["2016-12-31T14"])),
                options: None,
            },
            TestCase {
                path: "Patient.a + 2 hour".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016-12-31T22:00:00"
                }),
                expected: Expected::Value(json!(["2017-01-01T00:00:00"])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn evaluate_sub_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": 2,
            "b": 6
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a - Patient.b".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([-4])),
                options: None,
            },
            TestCase {
                path: "Patient.a - 1 year".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016"
                }),
                expected: Expected::Value(json!(["2015"])),
                options: None,
            },
            TestCase {
                path: "Patient.a - 1 month".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016"
                }),
                expected: Expected::Value(json!(["2015"])),
                options: None,
            },
            TestCase {
                path: "Patient.a - 12 month".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016"
                }),
                expected: Expected::Value(json!(["2015"])),
                options: None,
            },
            TestCase {
                path: "Patient.a - 2 month".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016-01"
                }),
                expected: Expected::Value(json!(["2015-11"])),
                options: None,
            },
            TestCase {
                path: "Patient.a - 1 week".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016"
                }),
                expected: Expected::Value(json!(["2015"])),
                options: None,
            },
            TestCase {
                path: "Patient.a - 53 week".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016"
                }),
                expected: Expected::Value(json!(["2014"])),
                options: None,
            },
            TestCase {
                path: "Patient.a - 1 week".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016-01-06"
                }),
                expected: Expected::Value(json!(["2015-12-30"])),
                options: None,
            },
            TestCase {
                path: "Patient.a - 1 hour".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016"
                }),
                expected: Expected::Value(json!(["2015"])),
                options: None,
            },
            TestCase {
                path: "Patient.a - 8800 hour".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016"
                }),
                expected: Expected::Value(json!(["2014"])),
                options: None,
            },
            TestCase {
                path: "Patient.a - 24 hour".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016-01-01"
                }),
                expected: Expected::Value(json!(["2015-12-31"])),
                options: None,
            },
            TestCase {
                path: "Patient.a + 2 hour".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016-12-31T12"
                }),
                expected: Expected::Value(json!(["2016-12-31T14"])),
                options: None,
            },
            TestCase {
                path: "Patient.a + 2 hour".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": "2016-12-31T22:00:00"
                }),
                expected: Expected::Value(json!(["2017-01-01T00:00:00"])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn evaluate_mul_path() {
        let compiled = compile(&"Patient.a * Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 2,
            "b": 6
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([12]));
    }

    #[test]
    fn evaluate_div_path() {
        let compiled = compile(&"Patient.a div Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 5,
            "b": 2
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([2]));
    }

    #[test]
    fn evaluate_div_path_2() {
        let compiled = compile(&"Patient.a div Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 5.5,
            "b": 0.7
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([7]));
    }

    #[test]
    fn evaluate_mod_path() {
        let compiled = compile(&"Patient.a mod Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 5,
            "b": 2
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([1]));
    }

    #[test]
    fn evaluate_mod_path_2() {
        let compiled = compile(&"Patient.a mod Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 5.5,
            "b": 0.7
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        // @todo - precision
        assert_json_eq!(evaluate_result, json!([0.6]));
    }

    #[test]
    fn evaluate_divide_path() {
        let compiled = compile(&"Patient.a / Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 5,
            "b": 2
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([dec!(2.50)]));
    }

    #[test]
    fn evaluate_divide_path_2() {
        let compiled = compile(&"Patient.a / Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 5.5,
            "b": 0.7
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([dec!(7.8571428571428571428571428571)])
        );
    }

    #[test]
    fn evaluate_amp_path() {
        let compiled = compile(&"Patient.a & Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": "a",
            "b": "b"
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["ab"]));
    }

    #[test]
    fn evaluate_amp_path_2() {
        let compiled = compile(&"Patient.a & Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": "a",
            "b": []
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["a"]));
    }

    #[test]
    fn evaluate_abs_path() {
        let compiled = compile(&"Patient.a.abs()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": -5,
            "b": -5.5,
            "c": "-5.5 'mg'",
            "d": [],
            "e": [-5, -6]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.abs()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([5])),
                options: None,
            },
            TestCase {
                path: "Patient.b.abs()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([5.5])),
                options: None,
            },
            TestCase {
                path: "Patient.c.abs()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!(["5.5 'mg'"])),
                options: None,
            },
            TestCase {
                path: "Patient.d.abs()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.e.abs()".to_string(),
                input: patient.clone(),
                expected: Expected::Error(FhirpathError::EvaluateError { msg: "Expected single value for node".to_string() }),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn evaluate_ceiling_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": 1,
            "b": 1.1,
            "c": -1.1,
            "d": [],
            "e": [-5, -6]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.ceiling()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1])),
                options: None,
            },
            TestCase {
                path: "Patient.b.ceiling()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([2])),
                options: None,
            },
            TestCase {
                path: "Patient.c.ceiling()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([-1])),
                options: None,
            },
            TestCase {
                path: "Patient.d.ceiling()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.e.ceiling()".to_string(),
                input: patient.clone(),
                expected: Expected::Error(FhirpathError::EvaluateError { msg: "Expected single value for node".to_string() }),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn evaluate_exp_path() {
        let compiled = compile(&"Patient.a.exp()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 2
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([7.38905609893065]));

        let compiled = compile(&"Patient.a.abs()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 2,
            "b": 0,
            "c": -0.0,
            "d": [],
            "e": [-5, -6]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.exp()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([7.38905609893065])),
                options: None,
            },
            TestCase {
                path: "Patient.b.exp()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1])),
                options: None,
            },
            TestCase {
                path: "Patient.c.exp()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1])),
                options: None,
            },
            TestCase {
                path: "Patient.d.exp()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.e.exp()".to_string(),
                input: patient.clone(),
                expected: Expected::Error(FhirpathError::EvaluateError { msg: "Expected single value for node".to_string() }),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn evaluate_floor_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": 1,
            "b": 2.1,
            "c": -2.1,
            "d": [],
            "e": [-5, -6]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.floor()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1])),
                options: None,
            },
            TestCase {
                path: "Patient.b.floor()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([2])),
                options: None,
            },
            TestCase {
                path: "Patient.c.floor()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([-3])),
                options: None,
            },
            TestCase {
                path: "Patient.d.floor()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.e.floor()".to_string(),
                input: patient.clone(),
                expected: Expected::Error(FhirpathError::EvaluateError { msg: "Expected single value for node".to_string() }),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn evaluate_ln_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": 1,
            "b": 1.0,
            "c": 1.1,
            "d": [],
            "e": [-5, -6]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.ln()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([0])),
                options: None,
            },
            TestCase {
                path: "Patient.b.ln()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([0])),
                options: None,
            },
            TestCase {
                path: "Patient.c.ln()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([dec!(0.0953101798043248600439525528)])),
                options: None,
            },
            TestCase {
                path: "Patient.d.ln()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.e.ln()".to_string(),
                input: patient.clone(),
                expected: Expected::Error(FhirpathError::EvaluateError { msg: "Expected single value for node".to_string() }),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn evaluate_log_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": 16,
            "b": 100.0,
            "c": 1.1,
            "d": [],
            "e": [-5, -6]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.log(2)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([4])),
                options: None,
            },
            TestCase {
                path: "Patient.b.log(10)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([2])),
                options: None,
            },
            TestCase {
                path: "Patient.c.log(2)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([dec!(0.137503523749935)])),
                options: None,
            },
            TestCase {
                path: "Patient.d.log(2)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.e.log(2)".to_string(),
                input: patient.clone(),
                expected: Expected::Error(FhirpathError::EvaluateError { msg: "Expected single value for node".to_string() }),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn evaluate_power_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": 2,
            "b": 2.5,
            "c": -1,
            "d": [],
            "e": [-5, -6]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.power(3)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([8])),
                options: None,
            },
            TestCase {
                path: "Patient.b.power(2)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([6.25])),
                options: None,
            },
            TestCase {
                path: "Patient.c.power(0.5)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.c.power(0.0)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1])),
                options: None,
            },
            TestCase {
                path: "Patient.d.power(0.5)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.e.power(0.5)".to_string(),
                input: patient.clone(),
                expected: Expected::Error(FhirpathError::EvaluateError { msg: "Expected single value for node".to_string() }),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn evaluate_round_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": 1,
            "b": 3.14159,
            "c": -1.2,
            "d": [],
            "e": [-5, -6]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.round()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1])),
                options: None,
            },
            TestCase {
                path: "Patient.b.round()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([3])),
                options: None,
            },
            TestCase {
                path: "Patient.b.round(3)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([3.142])),
                options: None,
            },
            TestCase {
                path: "Patient.b.round(0)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([3])),
                options: None,
            },
            TestCase {
                path: "Patient.b.round(-1)".to_string(),
                input: patient.clone(),
                expected: Expected::Error(FhirpathError::EvaluateError { msg: "Value cannot be negative".to_string() }),
                options: None,
            },
            TestCase {
                path: "Patient.d.round()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.e.round()".to_string(),
                input: patient.clone(),
                expected: Expected::Error(FhirpathError::EvaluateError { msg: "Expected single value for node".to_string() }),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn evaluate_sqrt_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": 81,
            "b": -1,
            "d": [],
            "e": [-5, -6]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.sqrt()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([9])),
                options: None,
            },
            TestCase {
                path: "Patient.b.sqrt()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.d.round()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.e.round()".to_string(),
                input: patient.clone(),
                expected: Expected::Error(FhirpathError::EvaluateError { msg: "Expected single value for node".to_string() }),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn evaluate_truncate_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": 101,
            "b": 1.00000001,
            "c": -1.56,
            "d": [],
            "e": [-5, -6]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.truncate()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([101])),
                options: None,
            },
            TestCase {
                path: "Patient.b.truncate()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1])),
                options: None,
            },
            TestCase {
                path: "Patient.c.truncate()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([-1])),
                options: None,
            },
            TestCase {
                path: "Patient.d.round()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.e.round()".to_string(),
                input: patient.clone(),
                expected: Expected::Error(FhirpathError::EvaluateError { msg: "Expected single value for node".to_string() }),
                options: None,
            },
        ];

        run_tests(test_cases);
    }
}
