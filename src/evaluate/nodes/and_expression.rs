use crate::{
    error::FhirpathError,
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::expression::AndExpression,
};

use super::{resource_node::ResourceNode, utils::invoke_operation};

impl Evaluate for AndExpression {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::EvaluateError {
                msg: "AndExpression must have exactly two children".to_string(),
            });
        }

        invoke_operation(&self.op, input, &self.children)
    }
}

impl Text for AndExpression {
    fn text(&self) -> EvaluateResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<EvaluateResult<Vec<String>>>()?
            .join(&self.op.clone()))
    }
}
