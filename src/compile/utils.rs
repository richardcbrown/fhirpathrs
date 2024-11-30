use serde_json::{Number, Value};

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{CompileResult, Evaluate, ResourceNode};

pub fn get_string(value: &Value) -> CompileResult<String> {
    match value {
        Value::String(string) => Ok(string.clone()),
        _ => Err(FhirpathError::CompileError {
            msg: format!("Value {} is not a string", value),
        }),
    }
}

pub fn get_option_string(value: &Option<Value>) -> CompileResult<String> {
    let string_val = value.clone().ok_or(FhirpathError::CompileError {
        msg: "Expected String but got None".to_string(),
    })?;

    get_string(&string_val)
}

pub fn get_option_string_vec(value: &Option<Value>) -> CompileResult<Vec<String>> {
    let val = value.clone().ok_or(FhirpathError::CompileError {
        msg: "Expected String or Vec<String> but got None".to_string(),
    })?;

    match val {
        Value::Array(array) => {
            let string_vec: CompileResult<Vec<String>> =
                array.iter().map(|item| get_string(item)).collect();

            Ok(string_vec?)
        }
        Value::String(string_item) => Ok(vec![string_item]),
        _ => Err(FhirpathError::CompileError {
            msg: "Expected String or Vec<String>".to_string(),
        }),
    }
}

pub fn get_value_from_expression(
    input: &ResourceNode,
    expression: &Expression,
) -> CompileResult<Value> {
    expression.evaluate(input).and_then(|res| {
        res.data.ok_or(FhirpathError::CompileError {
            msg: "Expression evaluation contained no result".to_string(),
        })
    })
}

pub fn get_number_from_expression(
    input: &ResourceNode,
    expression: &Expression,
) -> CompileResult<Number> {
    let expr_result = expression.evaluate(input).and_then(|res| {
        res.data.ok_or(FhirpathError::CompileError {
            msg: "Expression evaluation contained no result".to_string(),
        })
    })?;

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
        .as_u64()
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
    let value = get_value_from_expression(input, expression)?;

    match value {
        Value::String(str_result) => Ok(str_result),
        _ => Err(FhirpathError::CompileError {
            msg: "Value was not a String".to_string(),
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
