// http://hl7.org/fhirpath/N1/#string-manipulation

use std::cell::LazyCell;

use regex::Regex;
use serde_json::{json, Number, Value};

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    utils::{get_string, get_string_from_expression, get_string_vec, get_usize_from_expression},
    CompileResult, Evaluate, ResourceNode,
};

const WHITESPACE_REGEX: LazyCell<Regex> = LazyCell::new(|| Regex::new(r"[\s\n\r\t]+").unwrap());

pub fn normalise(string_val: &String) -> String {
    WHITESPACE_REGEX
        .replace_all(string_val.to_lowercase().as_str(), " ")
        .to_string()
}

pub fn index_of<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let second = &expressions[0];

    let first_val = input.get_single()?;
    let second_val = second.evaluate(input)?.get_single()?;

    let first_string = get_string(&first_val)?;
    let second_string = get_string(&second_val)?;

    let index: i64 = first_string
        .find(&second_string)
        .and_then(|val| u64::try_from(val).ok())
        .and_then(|val| i64::try_from(val).ok())
        .unwrap_or(-1);

    Ok(ResourceNode::from_node(input, json!(index)))
}

pub fn substring<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let first_expr = expressions.first().ok_or(FhirpathError::CompileError {
        msg: "Substring expects at least one expression".to_string(),
    })?;

    let string_value = get_string(&input.get_single()?)?;

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

    Ok(ResourceNode::from_node(
        input,
        json!(sub_string.to_string()),
    ))
}

pub fn starts_with<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let first_expr = expressions.first().ok_or(FhirpathError::CompileError {
        msg: "StartsWith expects one expression".to_string(),
    })?;

    let string_value = get_string(&input.get_single()?)?;
    let match_string = get_string_from_expression(input, first_expr)?;

    let starts_with = string_value.starts_with(&match_string);

    Ok(ResourceNode::from_node(input, json!(starts_with)))
}

pub fn ends_with<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let first_expr = expressions.first().ok_or(FhirpathError::CompileError {
        msg: "StartsWith expects one expression".to_string(),
    })?;

    let string_value = get_string(&input.get_single()?)?;
    let match_string = get_string_from_expression(input, first_expr)?;

    let ends_with = string_value.ends_with(&match_string);

    Ok(ResourceNode::from_node(input, json!(ends_with)))
}

pub fn contains<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let first_expr = expressions.first().ok_or(FhirpathError::CompileError {
        msg: "StartsWith expects one expression".to_string(),
    })?;

    let string_value = get_string(&input.get_single()?)?;
    let match_string = get_string_from_expression(input, first_expr)?;

    let contains = string_value.contains(&match_string);

    Ok(ResourceNode::from_node(input, json!(contains)))
}

pub fn upper<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let string_values = get_string_vec(&input.data)?;

    let replaced: Vec<Value> = string_values
        .iter()
        .map(|val| val.to_uppercase())
        .map(|item| Value::String(item))
        .collect();

    Ok(ResourceNode::from_node(input, json!(replaced)))
}

pub fn lower<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let string_values = get_string_vec(&input.data)?;

    let replaced: Vec<Value> = string_values
        .iter()
        .map(|val| val.to_lowercase())
        .map(|item| Value::String(item))
        .collect();

    Ok(ResourceNode::from_node(input, json!(replaced)))
}

pub fn replace<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let string_values = get_string_vec(&input.data)?;
    let pattern = get_string_from_expression(input, &expressions[0])?;
    let replacement = get_string_from_expression(input, &expressions[1])?;

    let replaced: Vec<Value> = string_values
        .iter()
        .map(|val| val.replace(&pattern, &replacement))
        .map(|item| Value::String(item))
        .collect();

    Ok(ResourceNode::from_node(input, json!(replaced)))
}

pub fn matches<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let string_value = get_string(&input.get_single()?)?;
    let pattern = get_string_from_expression(input, &expressions[0])?;
    let regex = Regex::new(&pattern).map_err(|_| FhirpathError::CompileError {
        msg: "Failed to parse Regex".to_string(),
    })?;

    let matches = Regex::is_match(&regex, &string_value);

    Ok(ResourceNode::from_node(input, Value::Bool(matches)))
}

pub fn replace_matches<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let string_values = get_string_vec(&input.data)?;
    let pattern = get_string_from_expression(input, &expressions[0])?;
    let replacement = get_string_from_expression(input, &expressions[1])?;
    let regex = Regex::new(&pattern).map_err(|_| FhirpathError::CompileError {
        msg: "Failed to parse Regex".to_string(),
    })?;

    let replace_result: Vec<Value> = string_values
        .iter()
        .map(|string_value| {
            Value::String(regex.replace_all(&string_value, &replacement).to_string())
        })
        .collect();

    Ok(ResourceNode::from_node(input, json!(replace_result)))
}

pub fn length<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let string_values = get_string_vec(&input.data)?;

    let lengths: Vec<Value> = string_values
        .iter()
        .map(|string_value| {
            let num = Number::from(string_value.len());

            Value::Number(num)
        })
        .collect();

    Ok(ResourceNode::from_node(input, json!(lengths)))
}

pub fn to_chars<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let string_values = get_string_vec(&input.data)?;

    let char_sets = string_values
        .iter()
        .map(|string_value| {
            let string_chars: Vec<Value> = string_value
                .chars()
                .map(|c| Value::String(c.to_string()))
                .collect();

            Value::Array(string_chars)
        })
        .collect();

    Ok(ResourceNode::from_node(input, Value::Array(char_sets)))
}
