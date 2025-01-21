use serde_json::json;

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    filtering::where_function,
    utils::{evaluate_array_boolean_expression, try_convert_to_boolean},
    CompileResult, ResourceNode,
};

pub fn empty<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    Ok(ResourceNode::new(
        input.data_root.clone(),
        Some(Box::new(input)),
        json!(input.is_empty()?),
    ))
}

pub fn exists<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() == 0 {
        return Ok(ResourceNode::new(
            input.data_root.clone(),
            Some(Box::new(input)),
            json!(!input.is_empty()?),
        ));
    }

    let result = where_function(input, expressions)?;

    Ok(ResourceNode::new(
        input.data_root.clone(),
        Some(Box::new(input)),
        json!(!result.is_empty()?),
    ))
}

pub fn all<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() != 1 {
        return Err(FhirpathError::CompileError {
            msg: "Expected single expression".to_string(),
        });
    }

    let expr = expressions
        .first()
        .ok_or_else(|| FhirpathError::CompileError {
            msg: "Expected single expression".to_string(),
        })?;

    let results = evaluate_array_boolean_expression(input, expr)?;

    Ok(ResourceNode::new(
        input.data_root.clone(),
        Some(Box::new(input)),
        json!(results.iter().all(|item| *item)),
    ))
}

pub fn all_true<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let array = input.get_array()?;

    let results: Vec<bool> = array
        .iter()
        .map(|item| {
            let bool_result = try_convert_to_boolean(item);

            match bool_result {
                Some(bool) => Ok(bool),
                None => Err(FhirpathError::CompileError {
                    msg: "Value was not a boolean".to_string(),
                }),
            }
        })
        .collect::<CompileResult<Vec<bool>>>()?;

    let all_true = results.iter().all(|item| *item);

    Ok(ResourceNode::new(
        input.data_root.clone(),
        Some(Box::new(input)),
        json!(all_true),
    ))
}
