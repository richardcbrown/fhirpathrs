use serde_json::{json, Value};

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    target::Target,
    equality::values_are_equal,
    utils::{get_arrays, get_usize_from_expression, unique_array_elements},
    EvaluateResult, ResourceNode,
};

pub fn single<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a>> {
    let array = input.get_array()?;

    if array.len() != 1 {
        return Err(FhirpathError::EvaluateError {
            msg: format!("Expected array with single element but had {}", array.len()),
        });
    }

    let single_value = array.first().ok_or_else(|| FhirpathError::EvaluateError {
        msg: "Failed to get single item from array".to_string(),
    })?;

    Ok(ResourceNode::from_node(input, single_value.clone()))
}

pub fn first<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a>> {
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

pub fn last<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a>> {
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

pub fn tail<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a>> {
    let array = input.get_array()?;

    let tail_values: Vec<&Value> = array.iter().skip(1).collect();

    Ok(ResourceNode::from_node(input, json!(tail_values)))
}

pub fn skip<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a>> {
    let array = input.get_array()?;

    if expressions.len() > 1 {
        return Err(FhirpathError::EvaluateError {
            msg: "Skip expects exactly one expression".to_string(),
        });
    }

    let expression = expressions
        .first()
        .ok_or_else(|| FhirpathError::EvaluateError {
            msg: "Skip expects exactly one expression".to_string(),
        })?;

    let skip_num = get_usize_from_expression(input, expression)?;

    Ok(ResourceNode::from_node(
        input,
        json!(array.into_iter().skip(skip_num).collect::<Vec<&Value>>()),
    ))
}

pub fn take<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a>> {
    let array = input.get_array()?;

    if expressions.len() > 1 {
        return Err(FhirpathError::EvaluateError {
            msg: "Skip expects exactly one expression".to_string(),
        });
    }

    let expression = expressions
        .first()
        .ok_or_else(|| FhirpathError::EvaluateError {
            msg: "Skip expects exactly one expression".to_string(),
        })?;

    let take_num = get_usize_from_expression(input, expression)?;

    Ok(ResourceNode::from_node(
        input,
        json!(array.into_iter().take(take_num).collect::<Vec<&Value>>()),
    ))
}

pub fn intersect<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a>> {
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

pub fn exclude<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a>> {
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
