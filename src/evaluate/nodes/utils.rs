use crate::{
    error::FhirpathError,
    evaluate::{invocation_table::invocation_table, EvaluateResult},
    parser::expression::Expression,
};

use super::resource_node::ResourceNode;

pub fn invoke_operation<'a, 'b>(
    op: &String,
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    invocation_table()
        .get(op)
        .ok_or(FhirpathError::EvaluateError {
            msg: format!("No such operator {}", op),
        })
        .and_then(|function| function(input, expressions))
}

pub fn capitalise(string_value: &String) -> String {
    let mut c = string_value.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}