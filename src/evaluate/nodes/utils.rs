use crate::{
    error::FhirpathError,
    evaluate::{invocation_table::invocation_table, EvaluateResult},
    parser::expression::Expression,
};

use super::resource_node::ResourceNode;

pub fn invoke_operation<'a>(
    op: &String,
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a>> {
    invocation_table()
        .get(op)
        .ok_or(FhirpathError::EvaluateError {
            msg: format!("No such operator {}", op),
        })
        .and_then(|function| function(input, expressions))
}
