use serde_json::{json, Value};

use crate::parser::expression::Expression;

use super::{
    equality::values_are_equal,
    target::Target,
    utils::{get_arrays, unique_array_elements},
    CompileResult, ResourceNode,
};

pub fn union<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
    arity: Target,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let (array, mut second_array) = get_arrays(input, expressions, arity)?;

    let mut union_array: Vec<Value> = array
        .into_iter()
        .filter(|item| {
            second_array
                .iter()
                .find(|second_item| values_are_equal(item, *second_item))
                .is_none()
        })
        .collect();

    union_array.append(&mut second_array);

    Ok(ResourceNode::from_node(
        input,
        json!(unique_array_elements(&union_array)),
    ))
}

pub fn combine<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let (mut first_array, mut second_array) = get_arrays(input, expressions, Target::AnyAtRoot)?;

    first_array.append(&mut second_array);

    Ok(ResourceNode::from_node(input, json!(first_array)))
}
