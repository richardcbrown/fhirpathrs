use crate::{
    error::FhirpathError,
    evaluate::{CompileResult, Evaluate, Text},
    parser::expression::AdditiveExpression,
};

use super::{resource_node::ResourceNode, utils::invoke_operation};

impl Evaluate for AdditiveExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::CompileError {
                msg: "AdditiveExpression must have exactly two children".to_string(),
            });
        }

        invoke_operation(&self.op, input, &self.children)
    }
}

impl Text for AdditiveExpression {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join(&self.op.clone()))
    }
}
