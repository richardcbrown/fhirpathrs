use serde_json::Value;

use crate::{
    error::FhirpathError,
    evaluate::{utils::try_convert_to_boolean, EvaluateResult, Evaluate, Text},
    parser::literal::BooleanLiteral,
};

use super::resource_node::ResourceNode;

impl Evaluate for BooleanLiteral {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> EvaluateResult<ResourceNode<'a>> {
        let bool_val =
            try_convert_to_boolean(&Value::String(self.text.clone())).ok_or_else(|| {
                FhirpathError::EvaluateError {
                    msg: format!("Could not convert {} to Bool", self.text.clone()),
                }
            })?;

        Ok(ResourceNode::from_node(input, Value::Bool(bool_val)))
    }
}

impl Text for BooleanLiteral {
    fn text(&self) -> EvaluateResult<String> {
        Ok(self.text.clone())
    }
}
