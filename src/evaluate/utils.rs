use serde_json::{Number, Value};

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    arity::{get_input_for_arity, Arity},
    equality::values_are_equal,
    CompileResult, Evaluate, ResourceNode,
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

pub fn get_string(value: &Value) -> CompileResult<String> {
    match value {
        Value::String(string) => Ok(string.clone()),
        _ => Err(FhirpathError::CompileError {
            msg: format!("Value {} is not a string", value),
        }),
    }
}

pub fn get_string_vec(value: &Value) -> CompileResult<Vec<String>> {
    match value {
        Value::Array(array) => {
            let string_vec: CompileResult<Vec<String>> =
                array.iter().map(|item| get_string(item)).collect();

            Ok(string_vec?)
        }
        _ => Err(FhirpathError::CompileError {
            msg: "Expected Vec<String>".to_string(),
        }),
    }
}

pub fn get_value_from_expression(
    input: &ResourceNode,
    expression: &Expression,
) -> CompileResult<Value> {
    expression.evaluate(input).and_then(|res| Ok(res.data))
}

pub fn get_number_from_expression(
    input: &ResourceNode,
    expression: &Expression,
) -> CompileResult<Number> {
    let expr_result = expression
        .evaluate(input)
        .and_then(|res| Ok(res.get_single()?))?;

    match expr_result {
        Value::Number(num) => Ok(num),
        _ => Err(FhirpathError::CompileError {
            msg: "Value was not a Number".to_string(),
        }),
    }
}

pub fn get_usize_from_expression(
    input: &ResourceNode,
    expression: &Expression,
) -> CompileResult<usize> {
    let json_num = get_number_from_expression(input, expression)?;

    json_num
        .as_f64()
        .and_then(|num| Some(num as u64))
        .ok_or(FhirpathError::CompileError {
            msg: "Could not convert number to u64".to_string(),
        })
        .and_then(|num| {
            usize::try_from(num).map_err(|_| FhirpathError::CompileError {
                msg: "Could not convert number to u64".to_string(),
            })
        })
}

pub fn get_string_from_expression(
    input: &ResourceNode,
    expression: &Expression,
) -> CompileResult<String> {
    let value = get_single_value(get_value_from_expression(input, expression)?)?;

    match value {
        Value::String(str_result) => Ok(str_result),
        _ => Err(FhirpathError::CompileError {
            msg: "Value was not a String".to_string(),
        }),
    }
}

pub fn get_boolean_from_expression(
    input: &ResourceNode,
    expression: &Expression,
) -> CompileResult<bool> {
    let value = get_single_value(get_value_from_expression(input, expression)?)?;

    match value {
        Value::Bool(bool_result) => Ok(bool_result),
        _ => Err(FhirpathError::CompileError {
            msg: "Value was not a Boolean".to_string(),
        }),
    }
}

pub fn get_single_value(value: Value) -> CompileResult<Value> {
    match value {
        Value::Array(array) => {
            if array.len() != 1 {
                return Err(FhirpathError::CompileError {
                    msg: "expected single result".to_string(),
                });
            }

            Ok(array
                .first()
                .ok_or(FhirpathError::CompileError {
                    msg: "expected single result".to_string(),
                })?
                .clone())
        }
        _ => Ok(value),
    }
}

pub fn get_array_from_expression(
    input: &ResourceNode,
    expression: &Expression,
) -> CompileResult<Vec<Value>> {
    let value = get_value_from_expression(input, expression)?;

    dbg!(value.clone());

    match value {
        Value::Array(array) => Ok(array),
        _ => Err(FhirpathError::CompileError {
            msg: "Value was not an Array".to_string(),
        }),
    }
}

pub fn get_arrays<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
    arity: Arity,
) -> CompileResult<(Vec<Value>, Vec<Value>)> {
    match arity {
        Arity::AnyAtRoot => {
            let array = input.get_array()?;

            if expressions.len() > 1 {
                return Err(FhirpathError::CompileError {
                    msg: "Expected exactly one expression".to_string(),
                });
            }

            let expression = expressions
                .first()
                .ok_or_else(|| FhirpathError::CompileError {
                    msg: "Expected exactly one expression".to_string(),
                })?;

            let second_input = get_input_for_arity(input, arity);

            let second_array = get_array_from_expression(&second_input, &expression)?;

            Ok((array.to_vec(), second_array))
        }
        Arity::Expr => {
            if expressions.len() != 2 {
                return Err(FhirpathError::CompileError {
                    msg: "Expected exactly two expressions".to_string(),
                });
            }

            let first_expression =
                expressions
                    .first()
                    .ok_or_else(|| FhirpathError::CompileError {
                        msg: "Expected exactly two expressions".to_string(),
                    })?;

            let second_expression =
                expressions
                    .into_iter()
                    .nth(1)
                    .ok_or_else(|| FhirpathError::CompileError {
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
) -> CompileResult<Vec<bool>> {
    let results: Vec<bool> = input
        .get_array()?
        .iter()
        .map(|item| {
            let node = ResourceNode::new(
                input.data_root,
                None,
                item.to_owned(),
                input.context,
                input.path.clone(),
            );

            expr.evaluate(&node)
                .and_then(|result| result.get_single())
                .and_then(|value| {
                    let bool_result = try_convert_to_boolean(&value);

                    match bool_result {
                        Some(bool) => Ok(bool),
                        None => Err(FhirpathError::CompileError {
                            msg: "Value was not a boolean".to_string(),
                        }),
                    }
                })
        })
        .collect::<CompileResult<Vec<bool>>>()?;

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

pub fn get_i128_from_expression(
    input: &ResourceNode,
    expression: &Expression,
) -> CompileResult<i128> {
    let value = get_single_value(get_value_from_expression(input, expression)?)?;

    match value {
        Value::Number(num) => num.as_i128().ok_or_else(|| FhirpathError::CompileError {
            msg: "Value was not an i128".to_string(),
        }),
        _ => Err(FhirpathError::CompileError {
            msg: "Value was not a Number".to_string(),
        }),
    }
}

pub fn get_f64_from_expression(
    input: &ResourceNode,
    expression: &Expression,
) -> CompileResult<f64> {
    let value = get_single_value(get_value_from_expression(input, expression)?)?;

    match value {
        Value::Number(num) => num.as_f64().ok_or_else(|| FhirpathError::CompileError {
            msg: "Value was not an f64".to_string(),
        }),
        _ => Err(FhirpathError::CompileError {
            msg: "Value was not a Number".to_string(),
        }),
    }
}
