use serde_json::{json, Value};

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{CompileResult, Evaluate, ResourceNode};

// todo - pad out
pub fn values_are_equal(first: &Value, second: &Value) -> bool {
    first == second
}

fn are_equal(input: &ResourceNode, expressions: &Vec<Box<Expression>>) -> CompileResult<bool> {
    if expressions.len() != 2 {
        return Err(FhirpathError::CompileError {
            msg: "Equal function takes exactly 2 expressions".to_string(),
        });
    }

    let first = &expressions[0];
    let second = &expressions[1];

    let first_val = first.evaluate(input)?.data;
    let second_val = second.evaluate(input)?.data;

    Ok(values_are_equal(&first_val, &second_val))
}

pub fn equal<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let result = are_equal(input, expressions)?;

    Ok(ResourceNode::from_node(input, json!(result)))
}

pub fn not_equal<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let result = are_equal(input, expressions)?;

    Ok(ResourceNode::from_node(input, json!(!result)))
}
