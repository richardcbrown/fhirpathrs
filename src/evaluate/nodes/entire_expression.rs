use crate::{
    error::FhirpathError,
    evaluate::{CompileResult, Evaluate, Text},
    parser::expression::EntireExpression,
};

use super::resource_node::ResourceNode;

impl Evaluate for EntireExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let child = self.children.first().ok_or(FhirpathError::CompileError {
            msg: "Missing EntireExpression child element".to_string(),
        })?;

        child.evaluate(input)
    }
}

impl Text for EntireExpression {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join(""))
    }
}
