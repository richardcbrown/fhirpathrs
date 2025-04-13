use crate::{
    error::FhirpathError,
    evaluate::{CompileResult, Evaluate, Text},
    parser::literal::LiteralTerm,
};

use super::resource_node::ResourceNode;

impl Evaluate for LiteralTerm {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        if self.children.len() != 1 {
            return Err(FhirpathError::CompileError {
                msg: "LiteralTerm should have exactly one child".to_string(),
            });
        }

        let first = &self.children[0];

        Ok(ResourceNode::from_node(input, first.evaluate(input)?.data))
    }
}

impl Text for LiteralTerm {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join(""))
    }
}
