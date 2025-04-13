use crate::{
    error::FhirpathError,
    evaluate::{CompileResult, Evaluate, Text},
    parser::expression::TermExpression,
};

use super::resource_node::ResourceNode;

impl Evaluate for TermExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let child = self.children.first().ok_or(FhirpathError::CompileError {
            msg: "Missing TermExpression child element".to_string(),
        })?;

        child.evaluate(input)
    }
}

impl Text for TermExpression {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join(""))
    }
}
