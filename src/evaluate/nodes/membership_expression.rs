use crate::{
    error::FhirpathError,
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::expression::MembershipExpression,
};

use super::{resource_node::ResourceNode, utils::invoke_operation};

impl Evaluate for MembershipExpression {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::EvaluateError {
                msg: "MembershipExpression must have exactly two children".to_string(),
            });
        }

        let op = match self.op.as_str() {
            "contains" => "collection_contains",
            _ => &self.op,
        };

        invoke_operation(&op.to_string(), input, &self.children)
    }
}

impl Text for MembershipExpression {
    fn text(&self) -> EvaluateResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<EvaluateResult<Vec<String>>>()?
            .join(&self.op.clone()))
    }
}
