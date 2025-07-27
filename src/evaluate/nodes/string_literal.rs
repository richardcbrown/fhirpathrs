use serde_json::{json, Value};

use crate::{
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::literal::StringLiteral,
};
use crate::evaluate::fhir_type::determine_fhir_type;
use super::resource_node::ResourceNode;

impl Evaluate for StringLiteral {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        let string_val = Value::String(self.text.clone());
        
        let fhir_types = vec![determine_fhir_type(Some(&string_val), None, input.context, false)];

        let mut node = ResourceNode::from_node(input, string_val);

        node.fhir_types = fhir_types;

        Ok(node)
    }
}

impl Text for StringLiteral {
    fn text(&self) -> EvaluateResult<String> {
        Ok(format!("'{}'", self.text.clone()))
    }
}
