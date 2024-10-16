use serde_json::json;

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{CompileResult, Evaluate, ResourceNode};

pub fn equal<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::CompileError {
            msg: "Equal function takes exactly 2 expressions".to_string(),
        });
    }

    let first = &expressions[0];
    let second = &expressions[1];

    let first_val = first.evaluate(input)?.data.unwrap_or(json!(false));
    let second_val = second.evaluate(input)?.data.unwrap_or(json!(false));

    let are_equal = first_val == second_val;

    Ok(ResourceNode {
        parent_node: Some(Box::new(input)),
        data: Some(json!(are_equal)),
    })
}
