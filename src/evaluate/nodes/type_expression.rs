use crate::{
    error::FhirpathError,
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::expression::TypeExpression,
};

use super::{resource_node::ResourceNode, utils::invoke_operation};

impl Evaluate for TypeExpression {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::EvaluateError {
                msg: "TypeExpression must have exactly two children".to_string(),
            });
        }

        let expression = self.children.first().ok_or(FhirpathError::EvaluateError {
            msg: "Missing Expression".to_string(),
        })?;

        let type_specifier = self
            .children
            .iter()
            .nth(1)
            .ok_or(FhirpathError::EvaluateError {
                msg: "Missing Type Specifier".to_string(),
            })?;

        let specifier_text = type_specifier.text()?;

        let result = expression.evaluate(input)?;

        let invoke_result = invoke_operation(
            &self.op,
            &result,
            &vec![Box::new(specifier_text.try_into()?)],
        )?;

        Ok(ResourceNode::from_node(input, invoke_result.data))
    }
}

impl Text for TypeExpression {
    fn text(&self) -> EvaluateResult<String> {
        Ok(
            self.children
                .iter()
                .map(|node| {
                    node.text()
                })
                .collect::<EvaluateResult<Vec<String>>>()
                ?.join(format!(" {} ",  self.op).as_str())
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluate::{compile, Text};

    #[test]
    fn test_indexer_expression_text() {
        let compiled = compile(&"Patient.name.where($this is FHIR.Patient)".to_string());

        let text = compiled.unwrap().expression.text().unwrap();
        assert_eq!(text, "Patient.name.where($this is FHIR.Patient)");
    }
}