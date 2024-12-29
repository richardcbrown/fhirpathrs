use serde_json::Value;

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    utils::{get_single_value, get_string},
    CompileResult, Evaluate, ResourceNode,
};

pub fn add<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::CompileError {
            msg: "add expects exactly two expressions".to_string(),
        });
    }

    let first = expressions[0]
        .evaluate(input)?
        .data
        .ok_or(FhirpathError::CompileError {
            msg: "first expression returned no result".to_string(),
        })?;
    let second = expressions[1]
        .evaluate(input)?
        .data
        .ok_or(FhirpathError::CompileError {
            msg: "second expression returned no result".to_string(),
        })?;

    let first_result = get_single_value(first)?;
    let second_result = get_single_value(second)?;

    let result: Value = match first_result {
        Value::String(mut first_string) => {
            let second_string = get_string(&second_result)?;

            first_string.push_str(second_string.as_str());

            Value::String(first_string)
        }
        _ => todo!(),
    };

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(result),
    })
}
