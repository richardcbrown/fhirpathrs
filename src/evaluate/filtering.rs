use serde_json::{json, Value};

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{CompileResult, Evaluate, ResourceNode};

fn evaluate_filter_expression(
    input: ResourceNode,
    array: &Vec<Value>,
    expr: &Expression,
) -> Vec<Value> {
    let results: Vec<Value> = array
        .iter()
        .filter_map(|item| {
            let node = ResourceNode::new(
                input.data_root,
                None,
                item.to_owned(),
                input.context,
                input.path.clone(),
            );

            expr.evaluate(&node)
                .ok()
                .and_then(|result| result.get_single().ok())
                .and_then(|value| {
                    if let Value::Bool(bool) = value {
                        if bool {
                            return Some(item.to_owned());
                        }
                    }

                    None
                })
        })
        .collect();

    results
}

pub fn where_function<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    expressions
        .first()
        .ok_or(FhirpathError::CompileError {
            msg: "Missing first expression".to_string(),
        })
        .and_then(|expr| {
            let data = input
                .get_array()
                .and_then(|val| Ok(evaluate_filter_expression(input.clone(), val, expr)))?;

            Ok(ResourceNode::from_node(input, json!(data)))
        })
}

pub fn select<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let expression = expressions.first().ok_or(FhirpathError::CompileError {
        msg: "select function requires single expression argument".to_string(),
    })?;

    let value = input.get_array()?;

    let result = value
        .iter()
        .map(|val| {
            let node = ResourceNode::new(
                input.data_root,
                None,
                json!(val.clone()),
                input.context,
                input.path.clone(),
            );

            let result = expression.evaluate(&node)?.data;

            Ok(result)
        })
        .collect::<CompileResult<Vec<Value>>>()?;

    let combined: Vec<Value> = result
        .iter()
        .map(|item| match item {
            Value::Array(array) => Ok(array.to_owned()),
            _ => Err(FhirpathError::CompileError {
                msg: "Result was not an array".to_string(),
            }),
        })
        .collect::<CompileResult<Vec<Vec<Value>>>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(ResourceNode::from_node(input, json!(combined)))
}
