use crate::{
    error::FhirpathError,
    evaluate::{CompileResult, Evaluate, Text},
    parser::expression::TypeExpression,
};

use super::resource_node::ResourceNode;

impl Evaluate for TypeExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::CompileError {
                msg: "TypeExpression must have exactly two children".to_string(),
            });
        }

        todo!()
    }
}

impl Text for TypeExpression {
    fn text(&self) -> CompileResult<String> {
        todo!()
    }
}
