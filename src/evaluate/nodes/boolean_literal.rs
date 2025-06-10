use serde_json::Value;

use crate::{
    error::FhirpathError,
    evaluate::{utils::try_convert_to_boolean, CompileResult, Evaluate, Text},
    parser::literal::BooleanLiteral,
};

use super::resource_node::ResourceNode;

impl Evaluate for BooleanLiteral {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> CompileResult<ResourceNode<'a, 'b>> {
        let bool_val =
            try_convert_to_boolean(&Value::String(self.text.clone())).ok_or_else(|| {
                FhirpathError::CompileError {
                    msg: format!("Could not convert {} to Bool", self.text.clone()),
                }
            })?;

        Ok(ResourceNode::from_node(input, Value::Bool(bool_val)))
    }
}

impl Text for BooleanLiteral {
    fn text(&self) -> CompileResult<String> {
        Ok(self.text.clone())
    }
}
