use crate::{
    error::FhirpathError,
    evaluate::{invocation_table::invocation_table, CompileResult},
    parser::expression::Expression,
};

use super::resource_node::ResourceNode;

pub fn invoke_operation<'a, 'b>(
    op: &String,
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    invocation_table()
        .get(op)
        .ok_or(FhirpathError::CompileError {
            msg: format!("No such operator {}", op),
        })
        .and_then(|function| function(input, expressions))
}
