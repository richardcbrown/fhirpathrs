use crate::{
    error::FhirpathError,
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::expression::UnionExpression,
};

use super::{resource_node::ResourceNode, utils::invoke_operation};

impl Evaluate for UnionExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> EvaluateResult<ResourceNode<'a>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::EvaluateError {
                msg: "UnionExpression must have exactly two children".to_string(),
            });
        }

        invoke_operation(&self.op, input, &self.children)
    }
}

impl Text for UnionExpression {
    fn text(&self) -> EvaluateResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<EvaluateResult<Vec<String>>>()?
            .join(&self.op.clone()))
    }
}
