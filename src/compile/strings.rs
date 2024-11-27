use std::fmt::format;

use serde_json::{json, Number, Value};

use crate::{
    error::FhirpathError,
    parser::expression::{self, Expression},
};

use super::{CompileResult, Evaluate, ResourceNode};

fn get_string(value: &Value) -> CompileResult<String> {
    match value {
        Value::String(string) => Ok(string.clone()),
        _ => Err(FhirpathError::CompileError {
            msg: format!("Value {} is not a string", value),
        }),
    }
}

fn get_option_string(value: &Option<Value>) -> CompileResult<String> {
    let string_val = value.clone().ok_or(FhirpathError::CompileError {
        msg: "Expexted String but got None".to_string(),
    })?;

    get_string(&string_val)
}

fn get_number_from_expression(
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

fn get_usize_from_expression(
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

pub fn index_of<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let second = &expressions[0];

    let first_val = input.data.as_ref().unwrap_or(&json!(null)).clone();
    let second_val = second.evaluate(input)?.data.unwrap_or(json!(null));

    let first_string = get_string(&first_val)?;
    let second_string = get_string(&second_val)?;

    let index: i64 = first_string
        .find(&second_string)
        .and_then(|val| u64::try_from(val).ok())
        .and_then(|val| i64::try_from(val).ok())
        .unwrap_or(-1);

    Ok(ResourceNode {
        parent_node: Some(Box::new(input)),
        data: Some(json!(index)),
    })
}

pub fn substring<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let first_expr = expressions.first().ok_or(FhirpathError::CompileError {
        msg: "Substring expects at least one expression".to_string(),
    })?;

    let string_value = get_option_string(&input.data)?;

    let second_expr = expressions.iter().nth(1);

    let first_index = get_usize_from_expression(input, &first_expr)?;
    let second_index = second_expr
        .and_then(|se| Some(get_usize_from_expression(input, se)))
        .unwrap_or(Ok(string_value.len()))?;

    let sub_string =
        string_value
            .get(first_index..second_index)
            .ok_or(FhirpathError::CompileError {
                msg: format!(
                    "Could not slice {} from {} to {}",
                    string_value, first_index, second_index
                ),
            })?;

    Ok(ResourceNode {
        parent_node: Some(Box::new(input)),
        data: Some(Value::String(sub_string.to_string())),
    })
}
