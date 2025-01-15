use serde_json::Value;

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    utils::{bool_from_string, get_array_from_expression, get_boolean_from_expression},
    CompileResult, Evaluate, ResourceNode,
};

pub fn iif<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() < 2 || expressions.len() > 3 {
        return Err(FhirpathError::CompileError {
            msg: "iif expects 2-3 Expressions".to_string(),
        });
    }

    let mut exp_iter = expressions.iter();

    let first = exp_iter.next().ok_or_else(|| FhirpathError::CompileError {
        msg: "Missing iif condition".to_string(),
    })?;

    let second = exp_iter.next().ok_or_else(|| FhirpathError::CompileError {
        msg: "Missing iif true result".to_string(),
    })?;

    let third = exp_iter.next();

    let condition = get_boolean_from_expression(input, first)?;

    let output = match condition {
        true => get_array_from_expression(input, second)?,
        false => match third {
            Some(exp) => get_array_from_expression(input, exp)?,
            None => vec![],
        },
    };

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Array(output)),
    })
}

fn try_convert_to_boolean(value: &Value) -> Option<bool> {
    match value {
        Value::Bool(val) => Some(*val),
        Value::Number(val) => match val.as_i64() {
            Some(num) => match num {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            },
            None => None,
        },
        Value::String(val) => match bool_from_string(&val) {
            Some(val) => Some(val),
            None => None,
        },
        _ => None,
    }
}

pub fn toBoolean<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let result = input
        .data
        .clone()
        .ok_or_else(|| FhirpathError::CompileError {
            msg: "Node has no data".to_string(),
        })?;

    let bool_result: Vec<Value> = match try_convert_to_boolean(&result) {
        Some(val) => vec![Value::Bool(val)],
        None => vec![],
    };

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Array(bool_result)),
    })
}

pub fn convertsToBoolean<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let result = input
        .data
        .clone()
        .ok_or_else(|| FhirpathError::CompileError {
            msg: "Node has no data".to_string(),
        })?;

    let converts_bool: bool = match try_convert_to_boolean(&result) {
        Some(_val) => true,
        None => false,
    };

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Bool(converts_bool)),
    })
}
