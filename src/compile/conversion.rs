use serde_json::{json, Value};

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    utils::{
        bool_from_string, get_array_from_expression, get_boolean_from_expression,
        try_convert_to_boolean,
    },
    CompileResult, ResourceNode,
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

    Ok(ResourceNode::new(
        input.data_root.clone(),
        Some(Box::new(input)),
        json!(output),
    ))
}

pub fn toBoolean<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let result = input.get_single()?;

    let bool_result: Vec<Value> = match try_convert_to_boolean(&result) {
        Some(val) => vec![Value::Bool(val)],
        None => vec![],
    };

    Ok(ResourceNode::new(
        input.data_root.clone(),
        Some(Box::new(input)),
        json!(bool_result),
    ))
}

pub fn convertsToBoolean<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let result = input.get_single()?;

    let converts_bool: bool = match try_convert_to_boolean(&result) {
        Some(_val) => true,
        None => false,
    };

    Ok(ResourceNode::new(
        input.data_root.clone(),
        Some(Box::new(input)),
        json!(converts_bool),
    ))
}
