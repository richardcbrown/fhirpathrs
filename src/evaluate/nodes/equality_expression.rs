use crate::{
    error::FhirpathError,
    evaluate::{CompileResult, Evaluate, Text},
    parser::expression::EqualityExpression,
};

use super::{resource_node::ResourceNode, utils::invoke_operation};

impl Evaluate for EqualityExpression {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> CompileResult<ResourceNode<'a, 'b>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::CompileError {
                msg: "EqualityExpression must have exactly two children".to_string(),
            });
        }

        invoke_operation(&self.op, input, &self.children)
    }
}

impl Text for EqualityExpression {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join(&self.op.clone()))
    }
}
