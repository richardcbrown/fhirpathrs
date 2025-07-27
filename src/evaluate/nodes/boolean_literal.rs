use serde_json::Value;

use crate::{
    error::FhirpathError,
    evaluate::{utils::try_convert_to_boolean, EvaluateResult, Evaluate, Text},
    parser::literal::BooleanLiteral,
};
use crate::evaluate::fhir_type::determine_fhir_type;
use super::resource_node::ResourceNode;

impl Evaluate for BooleanLiteral {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        let bool_val =
            Value::Bool(try_convert_to_boolean(&Value::String(self.text.clone())).ok_or_else(|| {
                FhirpathError::EvaluateError {
                    msg: format!("Could not convert {} to Bool", self.text.clone()),
                }
            })?);

        let fhir_types = vec![determine_fhir_type(Some(&bool_val), None, input.context, false)];

        let mut node = ResourceNode::from_node(input, bool_val);

        node.fhir_types = fhir_types;

        Ok(node)
    }
}

impl Text for BooleanLiteral {
    fn text(&self) -> EvaluateResult<String> {
        Ok(self.text.clone())
    }
}
