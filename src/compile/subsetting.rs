use serde_json::Value;

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    arity::{get_input_for_arity, Arity},
    equality::values_are_equal,
    utils::{get_array_from_expression, get_option_array, get_usize_from_expression},
    CompileResult, ResourceNode,
};

pub fn single<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let array = get_option_array(&input.data)?;

    if array.len() != 1 {
        return Err(FhirpathError::CompileError {
            msg: format!("Expected array with single element but had {}", array.len()),
        });
    }

    let single_value = array.first().ok_or_else(|| FhirpathError::CompileError {
        msg: "Failed to get single item from array".to_string(),
    })?;

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Array(vec![single_value.clone()])),
    })
}

pub fn first<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let array = get_option_array(&input.data)?;

    let first_value = array.first();

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: match first_value {
            Some(first) => Some(Value::Array(vec![first.clone()])),
            None => Some(Value::Array(vec![])),
        },
    })
}

pub fn last<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let array = get_option_array(&input.data)?;

    let last_value = array.last();

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: match last_value {
            Some(first) => Some(Value::Array(vec![first.clone()])),
            None => Some(Value::Array(vec![])),
        },
    })
}

pub fn tail<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let array = get_option_array(&input.data)?;

    let tail_values: Vec<Value> = array.into_iter().skip(1).collect();

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Array(tail_values)),
    })
}

pub fn skip<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let array = get_option_array(&input.data)?;

    if expressions.len() > 1 {
        return Err(FhirpathError::CompileError {
            msg: "Skip expects exactly one expression".to_string(),
        });
    }

    let expression = expressions
        .first()
        .ok_or_else(|| FhirpathError::CompileError {
            msg: "Skip expects exactly one expression".to_string(),
        })?;

    let skip_num = get_usize_from_expression(input, expression)?;

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Array(array.into_iter().skip(skip_num).collect())),
    })
}

pub fn take<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let array = get_option_array(&input.data)?;

    if expressions.len() > 1 {
        return Err(FhirpathError::CompileError {
            msg: "Skip expects exactly one expression".to_string(),
        });
    }

    let expression = expressions
        .first()
        .ok_or_else(|| FhirpathError::CompileError {
            msg: "Skip expects exactly one expression".to_string(),
        })?;

    let take_num = get_usize_from_expression(input, expression)?;

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Array(array.into_iter().take(take_num).collect())),
    })
}

pub fn intersect<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let array = get_option_array(&input.data)?;

    if expressions.len() > 1 {
        return Err(FhirpathError::CompileError {
            msg: "Skip expects exactly one expression".to_string(),
        });
    }

    let expression = expressions
        .first()
        .ok_or_else(|| FhirpathError::CompileError {
            msg: "Skip expects exactly one expression".to_string(),
        })?;

    let second_input = get_input_for_arity(input, Arity::AnyAtRoot);

    let second_array = get_array_from_expression(&second_input, &expression)?;

    let intersect: Vec<Value> = array
        .into_iter()
        .filter(|item| {
            second_array
                .iter()
                .find(|second_item| values_are_equal(item, *second_item))
                .is_some()
        })
        .collect();

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Array(intersect)),
    })
}
