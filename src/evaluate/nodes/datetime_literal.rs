use serde_json::{json, Value};

use crate::{
    evaluate::{data_types::date_time::DateTime, EvaluateResult, Evaluate, Text},
    parser::literal::DatetimeLiteral,
};
use crate::evaluate::fhir_type::determine_fhir_type;
use super::resource_node::ResourceNode;

impl Evaluate for DatetimeLiteral {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        let dt_val = Value::String(self.text.clone());
        
        let fhir_types = vec![determine_fhir_type(Some(&dt_val), None, input.context, false)];

        let mut node = ResourceNode::from_node(input, dt_val);

        node.fhir_types = fhir_types;

        Ok(node)
    }
}

impl Text for DatetimeLiteral {
    fn text(&self) -> EvaluateResult<String> {
        Ok(self.text.clone())
    }
}
