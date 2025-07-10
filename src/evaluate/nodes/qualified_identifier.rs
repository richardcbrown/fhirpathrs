use serde_json::Value;

use crate::{
    error::FhirpathError,
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::identifier::QualifiedIdentifier,
};

use super::resource_node::ResourceNode;

impl Evaluate for QualifiedIdentifier {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        let identifiers: Vec<String> = self
            .children
            .iter()
            .map(|child| {
                child
                    .evaluate(input)
                    .and_then(|node| match node.get_single()? {
                        Value::String(string_val) => Ok(string_val),
                        _ => Err(FhirpathError::EvaluateError {
                            msg: "Invalid Identifier".to_string(),
                        }),
                    })
            })
            .collect::<EvaluateResult<Vec<String>>>()?;

        Ok(ResourceNode::from_node(
            input,
            Value::String(identifiers.join(".")),
        ))
    }
}

impl Text for QualifiedIdentifier {
    fn text(&self) -> EvaluateResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<EvaluateResult<Vec<String>>>()?
            .join("."))
    }
}
