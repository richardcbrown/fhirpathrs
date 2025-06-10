use crate::{
    error::FhirpathError,
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::literal::LiteralTerm,
};

use super::resource_node::ResourceNode;

impl Evaluate for LiteralTerm {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> EvaluateResult<ResourceNode<'a>> {
        if self.children.len() != 1 {
            return Err(FhirpathError::EvaluateError {
                msg: "LiteralTerm should have exactly one child".to_string(),
            });
        }

        let first = &self.children[0];

        Ok(ResourceNode::from_node(input, first.evaluate(input)?.data))
    }
}

impl Text for LiteralTerm {
    fn text(&self) -> EvaluateResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<EvaluateResult<Vec<String>>>()?
            .join(""))
    }
}
