use serde_json::json;

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{CompileResult, Evaluate, ResourceNode};

fn are_equal(input: &ResourceNode, expressions: &Vec<Box<Expression>>) -> CompileResult<bool> {
    if expressions.len() != 2 {
        return Err(FhirpathError::CompileError {
            msg: "Equal function takes exactly 2 expressions".to_string(),
        });
    }

    let first = &expressions[0];
    let second = &expressions[1];

    let first_val = first.evaluate(input)?.data.unwrap_or(json!(false));
    let second_val = second.evaluate(input)?.data.unwrap_or(json!(false));

    let equal_vals = first_val == second_val;

    Ok(equal_vals)
}

pub fn equal<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let result = are_equal(input, expressions)?;

    Ok(ResourceNode {
        parent_node: Some(Box::new(input)),
        data: Some(json!(result)),
    })
}

pub fn not_equal<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let result = are_equal(input, expressions)?;

    Ok(ResourceNode {
        parent_node: Some(Box::new(input)),
        data: Some(json!(!result)),
    })
}
