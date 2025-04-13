use crate::{
    error::FhirpathError,
    evaluate::{invocation_table::invocation_table, CompileResult},
    parser::expression::Expression,
};

use super::resource_node::ResourceNode;

pub fn invoke_operation<'a>(
    op: &String,
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    invocation_table()
        .get(op)
        .ok_or(FhirpathError::CompileError {
            msg: format!("No such operator {}", op),
        })
        .and_then(|function| function(input, expressions))
}
