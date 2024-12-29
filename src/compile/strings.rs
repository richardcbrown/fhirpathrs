// http://hl7.org/fhirpath/N1/#string-manipulation

use regex::Regex;
use serde_json::{json, Number, Value};

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    utils::{
        get_option_string, get_option_string_vec, get_string, get_string_from_expression,
        get_usize_from_expression,
    },
    CompileResult, Evaluate, ResourceNode,
};

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
        data_root: input.data_root.clone(),
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
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::String(sub_string.to_string())),
    })
}

pub fn starts_with<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let first_expr = expressions.first().ok_or(FhirpathError::CompileError {
        msg: "StartsWith expects one expression".to_string(),
    })?;

    let string_value = get_option_string(&input.data)?;
    let match_string = get_string_from_expression(input, first_expr)?;

    let starts_with = string_value.starts_with(&match_string);

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Bool(starts_with)),
    })
}

pub fn ends_with<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let first_expr = expressions.first().ok_or(FhirpathError::CompileError {
        msg: "StartsWith expects one expression".to_string(),
    })?;

    let string_value = get_option_string(&input.data)?;
    let match_string = get_string_from_expression(input, first_expr)?;

    let ends_with = string_value.ends_with(&match_string);

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Bool(ends_with)),
    })
}

pub fn contains<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let first_expr = expressions.first().ok_or(FhirpathError::CompileError {
        msg: "StartsWith expects one expression".to_string(),
    })?;

    let string_value = get_option_string(&input.data)?;
    let match_string = get_string_from_expression(input, first_expr)?;

    let contains = string_value.contains(&match_string);

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Bool(contains)),
    })
}

pub fn upper<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let string_values = get_option_string_vec(&input.data)?;

    let replaced: Vec<Value> = string_values
        .iter()
        .map(|val| val.to_uppercase())
        .map(|item| Value::String(item))
        .collect();

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Array(replaced)),
    })
}

pub fn lower<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let string_values = get_option_string_vec(&input.data)?;

    let replaced: Vec<Value> = string_values
        .iter()
        .map(|val| val.to_lowercase())
        .map(|item| Value::String(item))
        .collect();

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Array(replaced)),
    })
}

pub fn replace<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let string_values = get_option_string_vec(&input.data)?;
    let pattern = get_string_from_expression(input, &expressions[0])?;
    let replacement = get_string_from_expression(input, &expressions[1])?;

    let replaced: Vec<Value> = string_values
        .iter()
        .map(|val| val.replace(&pattern, &replacement))
        .map(|item| Value::String(item))
        .collect();

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Array(replaced)),
    })
}

pub fn matches<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let string_value = get_option_string(&input.data)?;
    let pattern = get_string_from_expression(input, &expressions[0])?;
    let regex = Regex::new(&pattern).map_err(|_| FhirpathError::CompileError {
        msg: "Failed to parse Regex".to_string(),
    })?;

    let matches = Regex::is_match(&regex, &string_value);

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Bool(matches)),
    })
}

pub fn replace_matches<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let string_values = get_option_string_vec(&input.data)?;
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

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Array(replace_result)),
    })
}

pub fn length<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let string_values = get_option_string_vec(&input.data)?;

    let lengths = string_values
        .iter()
        .map(|string_value| {
            let num = Number::from(string_value.len());

            Value::Number(num)
        })
        .collect();

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Array(lengths)),
    })
}

pub fn to_chars<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let string_values = get_option_string_vec(&input.data)?;

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

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Array(char_sets)),
    })
}
