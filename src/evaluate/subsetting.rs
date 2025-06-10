use serde_json::{json, Value};

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    target::Target,
    equality::values_are_equal,
    utils::{get_arrays, get_usize_from_expression, unique_array_elements},
    CompileResult, ResourceNode,
};

pub fn single<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let array = input.get_array()?;

    if array.len() != 1 {
        return Err(FhirpathError::CompileError {
            msg: format!("Expected array with single element but had {}", array.len()),
        });
    }

    let single_value = array.first().ok_or_else(|| FhirpathError::CompileError {
        msg: "Failed to get single item from array".to_string(),
    })?;

    Ok(ResourceNode::from_node(input, single_value.clone()))
}

pub fn first<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let array = input.get_array()?;

    let first_value = array.first();

    Ok(ResourceNode::from_node(
        input,
        match first_value {
            Some(first) => json!(first),
            None => Value::Array(vec![]),
        },
    ))
}

pub fn last<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let array = input.get_array()?;

    let last_value = array.last();

    Ok(ResourceNode::from_node(
        input,
        match last_value {
            Some(first) => json!(first),
            None => Value::Array(vec![]),
        },
    ))
}

pub fn tail<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let array = input.get_array()?;

    let tail_values: Vec<&Value> = array.iter().skip(1).collect();

    Ok(ResourceNode::from_node(input, json!(tail_values)))
}

pub fn skip<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let array = input.get_array()?;

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

    Ok(ResourceNode::from_node(
        input,
        json!(array.into_iter().skip(skip_num).collect::<Vec<&Value>>()),
    ))
}

pub fn take<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let array = input.get_array()?;

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

    Ok(ResourceNode::from_node(
        input,
        json!(array.into_iter().take(take_num).collect::<Vec<&Value>>()),
    ))
}

pub fn intersect<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let (array, second_array) = get_arrays(input, expressions, Target::AnyAtRoot)?;

    let intersect_array: Vec<Value> = array
        .into_iter()
        .filter(|item| {
            second_array
                .iter()
                .find(|second_item| values_are_equal(item, *second_item))
                .is_some()
        })
        .collect();

    Ok(ResourceNode::from_node(
        input,
        json!(unique_array_elements(&intersect_array)),
    ))
}

pub fn exclude<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let (array, second_array) = get_arrays(input, expressions, Target::AnyAtRoot)?;

    let exclude_array: Vec<Value> = array
        .into_iter()
        .filter(|item| {
            second_array
                .iter()
                .find(|second_item| values_are_equal(item, *second_item))
                .is_none()
        })
        .collect();

    Ok(ResourceNode::from_node(input, json!(exclude_array)))
}
