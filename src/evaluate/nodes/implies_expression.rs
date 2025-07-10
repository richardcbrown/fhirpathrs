use crate::error::FhirpathError;
use crate::evaluate::{Evaluate, EvaluateResult, Text};
use crate::evaluate::nodes::resource_node::ResourceNode;
use crate::evaluate::nodes::utils::invoke_operation;
use crate::parser::expression::ImpliesExpression;

impl Evaluate for ImpliesExpression {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::EvaluateError {
                msg: "ImpliesExpression must have exactly two children".to_string(),
            });
        }

        invoke_operation(&self.op, input, &self.children)
    }
}

impl Text for ImpliesExpression {
    fn text(&self) -> EvaluateResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<EvaluateResult<Vec<String>>>()?
            .join(&self.op.clone()))
    }
}