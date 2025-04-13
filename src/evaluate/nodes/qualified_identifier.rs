use serde_json::Value;

use crate::{
    error::FhirpathError,
    evaluate::{CompileResult, Evaluate, Text},
    parser::identifier::QualifiedIdentifier,
};

use super::resource_node::ResourceNode;

impl Evaluate for QualifiedIdentifier {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let identifiers: Vec<String> = self
            .children
            .iter()
            .map(|child| {
                child
                    .evaluate(input)
                    .and_then(|node| match node.get_single()? {
                        Value::String(string_val) => Ok(string_val),
                        _ => Err(FhirpathError::CompileError {
                            msg: "Invalid Identifier".to_string(),
                        }),
                    })
            })
            .collect::<CompileResult<Vec<String>>>()?;

        Ok(ResourceNode::from_node(
            input,
            Value::String(identifiers.join(".")),
        ))
    }
}

impl Text for QualifiedIdentifier {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join("."))
    }
}
