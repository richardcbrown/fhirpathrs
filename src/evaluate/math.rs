use serde_json::{json, Value};

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{utils::get_string, CompileResult, Evaluate, ResourceNode};

pub fn add<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::CompileError {
            msg: "add expects exactly two expressions".to_string(),
        });
    }

    let first = expressions[0].evaluate(input)?.get_single()?;

    let second = expressions[1].evaluate(input)?.get_single()?;

    let result: Value = match first {
        Value::String(mut first_string) => {
            let second_string = get_string(&second)?;

            first_string.push_str(second_string.as_str());

            Value::String(first_string)
        }
        _ => todo!(),
    };

    Ok(ResourceNode::from_node(input, json!(result)))
}
