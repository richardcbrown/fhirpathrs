use crate::{
    error::FhirpathError,
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::expression::ExternalConstantTerm,
};

use super::resource_node::ResourceNode;

impl Evaluate for ExternalConstantTerm {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> EvaluateResult<ResourceNode<'a>> {
        let expression = self
            .children
            .first()
            .ok_or_else(|| FhirpathError::EvaluateError {
                msg: "ExternalConstantTerm expects a single Expression".to_string(),
            })?;

        let variable = expression.evaluate(input)?;

        Ok(variable)
    }
}

impl Text for ExternalConstantTerm {
    fn text(&self) -> EvaluateResult<String> {
        Ok(format!(
            "%{}",
            self.children
                .iter()
                .map(|c| c.text())
                .collect::<EvaluateResult<Vec<String>>>()?
                .join("")
        ))
    }
}
