use serde_json::Value;

use crate::parser::expression::Expression;

use super::{
    arity::Arity,
    equality::values_are_equal,
    utils::{get_arrays, unique_array_elements},
    CompileResult, ResourceNode,
};

pub fn union<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
    arity: Arity,
) -> CompileResult<ResourceNode<'a>> {
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

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Array(unique_array_elements(&union_array))),
    })
}

pub fn combine<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let (mut first_array, mut second_array) = get_arrays(input, expressions, Arity::AnyAtRoot)?;

    first_array.append(&mut second_array);

    Ok(ResourceNode {
        data_root: input.data_root.clone(),
        parent_node: Some(Box::new(input)),
        data: Some(Value::Array(first_array)),
    })
}
