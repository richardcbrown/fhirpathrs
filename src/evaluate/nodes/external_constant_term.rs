use crate::{
    error::FhirpathError,
    evaluate::{CompileResult, Evaluate, Text},
    parser::expression::ExternalConstantTerm,
};

use super::resource_node::ResourceNode;

impl Evaluate for ExternalConstantTerm {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let expression = self
            .children
            .first()
            .ok_or_else(|| FhirpathError::CompileError {
                msg: "ExternalConstantTerm expects a single Expression".to_string(),
            })?;

        let variable = expression.evaluate(input)?;

        Ok(variable)
    }
}

impl Text for ExternalConstantTerm {
    fn text(&self) -> CompileResult<String> {
        Ok(format!(
            "%{}",
            self.children
                .iter()
                .map(|c| c.text())
                .collect::<CompileResult<Vec<String>>>()?
                .join("")
        ))
    }
}
