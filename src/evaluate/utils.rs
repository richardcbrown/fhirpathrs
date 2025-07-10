use std::str::FromStr;

use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde_json::{Number, Value};

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    equality::values_are_equal,
    nodes::resource_node::ResourceContext,
    target::{get_input_for_target, Target},
    EvaluateResult, Evaluate, ResourceNode,
};

static TRUTHY_STRINGS: [&str; 6] = ["1", "1.0", "y", "yes", "t", "true"];
static FALSEY_STRINGS: [&str; 6] = ["0", "0.0", "n", "no", "f", "false"];

pub fn bool_from_string(string_value: &String) -> Option<bool> {
    let lower_value = string_value.to_lowercase();

    if TRUTHY_STRINGS.contains(&lower_value.as_str()) {
        Some(true)
    } else if FALSEY_STRINGS.contains(&&lower_value.as_str()) {
        Some(false)
    } else {
        None
    }
}

pub fn get_string(value: &Value) -> EvaluateResult<String> {
    match value {
        Value::String(string) => Ok(string.clone()),
        _ => Err(FhirpathError::EvaluateError {
            msg: format!("Value {} is not a string", value),
        }),
    }
}

pub fn get_string_vec(value: &Value) -> EvaluateResult<Vec<String>> {
    match value {
        Value::Array(array) => {
            let string_vec: EvaluateResult<Vec<String>> =
                array.iter().map(|item| get_string(item)).collect();

            Ok(string_vec?)
        }
        _ => Err(FhirpathError::EvaluateError {
            msg: "Expected Vec<String>".to_string(),
        }),
    }
}

pub fn get_value_from_expression<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expression: &Expression,
) -> EvaluateResult<Value> {
    expression.evaluate(input).and_then(|res| Ok(res.data))
}

pub fn get_number_from_expression<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expression: &Expression,
) -> EvaluateResult<Number> {
    let expr_result = expression
        .evaluate(input)
        .and_then(|res| Ok(res.get_single()?))?;

    match expr_result {
        Value::Number(num) => Ok(num),
        _ => Err(FhirpathError::EvaluateError {
            msg: "Value was not a Number".to_string(),
        }),
    }
}

pub fn get_i32_from_expression<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expression: &Expression,
) -> EvaluateResult<i32> {
    let json_num = get_number_from_expression(input, expression)?;

    let num = json_num.as_f64().ok_or(FhirpathError::EvaluateError {
        msg: "Could not convert number to f64".to_string(),
    })?;

    let decimal = Decimal::from_f64(num).ok_or(FhirpathError::EvaluateError {
        msg: "Could not convert number to Decimal".to_string(),
    })?;

    if !decimal.is_integer() {
        return Err(FhirpathError::EvaluateError {
            msg: "Decimal is not an integer".to_string(),
        })
    }

    let converted: i32 = decimal.try_into().map_err(|e| FhirpathError::EvaluateError {
        msg: format!("Decimal cannot be converted to usize: {}", e),
    })?;

    Ok(converted)
}

pub fn get_usize_from_expression(
    input: &ResourceNode,
    expression: &Expression,
) -> EvaluateResult<usize> {
    let int_val = get_i32_from_expression(input, expression)?;

    if int_val < 0 {
        return Err(FhirpathError::EvaluateError {
            msg: "Value cannot be negative".to_string(),
        })
    }

    let converted: usize = int_val.try_into().map_err(|e| FhirpathError::EvaluateError {
        msg: format!("Value cannot be converted to usize: {}", e),
    })?;

    Ok(converted)
}

pub fn get_string_from_expression<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expression: &Expression,
) -> EvaluateResult<String> {
    let value = get_single_value(get_value_from_expression(input, expression)?)?;

    match value {
        Value::String(str_result) => Ok(str_result),
        _ => Err(FhirpathError::EvaluateError {
            msg: "Value was not a String".to_string(),
        }),
    }
}

pub fn get_boolean_from_expression<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expression: &Expression,
) -> EvaluateResult<bool> {
    let value = get_single_value(get_value_from_expression(input, expression)?)?;

    match value {
        Value::Bool(bool_result) => Ok(bool_result),
        _ => Err(FhirpathError::EvaluateError {
            msg: "Value was not a Boolean".to_string(),
        }),
    }
}

pub fn get_single_value(value: Value) -> EvaluateResult<Value> {
    match value {
        Value::Array(array) => {
            if array.len() != 1 {
                return Err(FhirpathError::EvaluateError {
                    msg: "expected single result".to_string(),
                });
            }

            Ok(array
                .first()
                .ok_or(FhirpathError::EvaluateError {
                    msg: "expected single result".to_string(),
                })?
                .clone())
        }
        _ => Ok(value),
    }
}

pub fn get_array_from_expression<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expression: &Expression,
) -> EvaluateResult<Vec<Value>> {
    let value = get_value_from_expression(input, expression)?;

    dbg!(value.clone());

    match value {
        Value::Array(array) => Ok(array),
        _ => Err(FhirpathError::EvaluateError {
            msg: "Value was not an Array".to_string(),
        }),
    }
}

pub fn get_arrays<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
    arity: Target,
) -> EvaluateResult<(Vec<Value>, Vec<Value>)> {
    match arity {
        Target::AnyAtRoot => {
            let array = input.get_array()?;

            if expressions.len() > 1 {
                return Err(FhirpathError::EvaluateError {
                    msg: "Expected exactly one expression".to_string(),
                });
            }

            let expression = expressions
                .first()
                .ok_or_else(|| FhirpathError::EvaluateError {
                    msg: "Expected exactly one expression".to_string(),
                })?;

            let second_input = get_input_for_target(input, arity);

            let second_array = get_array_from_expression(&second_input, &expression)?;

            Ok((array.to_vec(), second_array))
        }
        Target::Expr => {
            if expressions.len() != 2 {
                return Err(FhirpathError::EvaluateError {
                    msg: "Expected exactly two expressions".to_string(),
                });
            }

            let first_expression =
                expressions
                    .first()
                    .ok_or_else(|| FhirpathError::EvaluateError {
                        msg: "Expected exactly two expressions".to_string(),
                    })?;

            let second_expression =
                expressions
                    .into_iter()
                    .nth(1)
                    .ok_or_else(|| FhirpathError::EvaluateError {
                        msg: "Expected exactly two expressions".to_string(),
                    })?;

            let first_array = get_array_from_expression(input, first_expression)?;

            let second_array = get_array_from_expression(input, second_expression)?;

            Ok((first_array, second_array))
        }
    }
}

pub fn unique_array_elements(array: &Vec<Value>) -> Vec<Value> {
    let mut unique: Vec<Value> = vec![];

    array.into_iter().for_each(|item| {
        let exists = unique
            .iter()
            .find(|unique_item| values_are_equal(unique_item, item))
            .is_some();

        if !exists {
            unique.push(item.clone());
        }
    });

    unique
}

pub fn evaluate_array_boolean_expression(
    input: &ResourceNode,
    expr: &Expression,
) -> EvaluateResult<Vec<bool>> {
    let results: Vec<bool> = input
        .get_array()?
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let node = ResourceNode::new(
                input.data_root,
                item.to_owned(),
                input.context,
                input.path.clone(),
                input.fhir_types.clone(),
                Some(ResourceContext {
                    index: Some(index),
                    total: None,
                }),
                input.reflection_types.clone(),
            );

            expr.evaluate(&node)
                .and_then(|result| result.get_single())
                .and_then(|value| {
                    let bool_result = try_convert_to_boolean(&value);

                    match bool_result {
                        Some(bool) => Ok(bool),
                        None => Err(FhirpathError::EvaluateError {
                            msg: "Value was not a boolean".to_string(),
                        }),
                    }
                })
        })
        .collect::<EvaluateResult<Vec<bool>>>()?;

    Ok(results)
}

pub fn try_convert_to_boolean(value: &Value) -> Option<bool> {
    match value {
        Value::Bool(val) => Some(*val),
        Value::Number(val) => {
            if let Some(num) = val.as_i64() {
                match num {
                    0 => return Some(false),
                    1 => return Some(true),
                    _ => return None,
                }
            }

            if let Some(num) = val.as_f64() {
                match num {
                    0.0 => return Some(false),
                    1.0 => return Some(true),
                    _ => return None,
                }
            }

            None
        }
        Value::String(val) => match bool_from_string(&val) {
            Some(val) => Some(val),
            None => None,
        },
        _ => None,
    }
}

pub fn try_convert_to_decimal(value: &Value) -> Option<Decimal> {
    match value {
        Value::Bool(val) => match val {
            true => Decimal::from_u32(1),
            false => Decimal::from_u32(0),
        },
        Value::Number(val) => Decimal::from_f64(val.as_f64()?),
        Value::String(val) => Decimal::from_str(val).ok(),
        _ => None,
    }
}

pub fn get_f64_from_expression<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expression: &Expression,
) -> EvaluateResult<f64> {
    let value = get_single_value(get_value_from_expression(input, expression)?)?;

    match value {
        Value::Number(num) => num.as_f64().ok_or_else(|| FhirpathError::EvaluateError {
            msg: "Value was not an f64".to_string(),
        }),
        _ => Err(FhirpathError::EvaluateError {
            msg: "Value was not a Number".to_string(),
        }),
    }
}

pub fn from_decimal(dec: Decimal) -> EvaluateResult<f64> {
    dec.try_into().map_err(|_| FhirpathError::EvaluateError {
        msg: format!("Could not convert from Decimal"),
    })
}
