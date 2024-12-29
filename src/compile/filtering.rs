use serde_json::Value;

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{CompileResult, Evaluate, ResourceNode};

fn evaluate_array_expression(
    input: ResourceNode,
    array: &Vec<Value>,
    expr: &Expression,
) -> Vec<Value> {
    let results: Vec<Value> = array
        .iter()
        .filter_map(|item| {
            let node = ResourceNode {
                data_root: input.data_root.clone(),
                data: Some(item.to_owned()),
                parent_node: None,
            };

            expr.evaluate(&node)
                .ok()
                .and_then(|result| result.data)
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
        .and_then(|expr| {
            let data = input.data.as_ref().and_then(|val| match val {
                Value::Array(array) => {
                    let results: Vec<Value> = evaluate_array_expression(input.clone(), array, expr);

                    Some(Value::Array(results))
                }
                _ => None,
            });

            Some(ResourceNode {
                data_root: input.data_root.clone(),
                parent_node: Some(Box::new(input)),
                data,
            })
        })
        .ok_or(FhirpathError::CompileError {
            msg: "where_function requires single expression argument".to_string(),
        })
}

pub fn select<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let expression = expressions.first().ok_or(FhirpathError::CompileError {
        msg: "select function requires single expression argument".to_string(),
    })?;

    let value = input
        .data
        .as_ref()
        .and_then(|data| match data {
            Value::Array(array) => Some(array),
            _ => None,
        })
        .ok_or(FhirpathError::CompileError {
            msg: "expect input data to be an Array".to_string(),
        })?;

    let result: CompileResult<Vec<Value>> = value
        .iter()
        .map(|val| {
            let node = ResourceNode {
                data_root: input.data_root.clone(),
                parent_node: None,
                data: Some(val.clone()),
            };

            expression
                .evaluate(&node)?
                .data
                .ok_or(FhirpathError::CompileError {
                    msg: "expected evaluation to return a value but recieved None".to_string(),
                })
        })
        .collect();

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Array(result?)),
    })
}
