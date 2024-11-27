use serde_json::{json, Value};

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{CompileResult, Evaluate, ResourceNode};

fn get_string(value: &Value) -> CompileResult<String> {
    match value {
        Value::String(string) => Ok(string.clone()),
        _ => Err(FhirpathError::CompileError {
            msg: format!("Value {} is not a string", value),
        }),
    }
}

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
        parent_node: Some(Box::new(input)),
        data: Some(json!(index)),
    })
}
