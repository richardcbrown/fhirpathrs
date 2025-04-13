use crate::{
    error::FhirpathError,
    evaluate::{types::quantity::Quantity, CompileResult, Evaluate, Text},
    parser::literal::QuantityLiteral,
};

use super::resource_node::ResourceNode;

impl Evaluate for QuantityLiteral {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        Ok(ResourceNode::from_node(
            input,
            serde_json::to_value(Quantity::try_from(self)?).map_err(|err| {
                FhirpathError::CompileError {
                    msg: format!("Could not serialize Quantity: {}", err.to_string()),
                }
            })?,
        ))
    }
}

impl Text for QuantityLiteral {
    fn text(&self) -> CompileResult<String> {
        Ok(self.text.clone())
    }
}
