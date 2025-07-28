use serde_json::{json, Value};

use crate::{
    evaluate::{data_types::time::Time, EvaluateResult, Evaluate, Text},
    parser::literal::TimeLiteral,
};
use crate::evaluate::fhir_type::determine_fhir_type;
use super::resource_node::ResourceNode;

impl Evaluate for TimeLiteral {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        let time_val = Value::String(self.text.clone());

        let fhir_types = vec![determine_fhir_type(Some(&time_val), None, input.context, false)];

        let mut node = ResourceNode::from_node(input, time_val);

        node.fhir_types = fhir_types;

        Ok(node)
    }
}

impl Text for TimeLiteral {
    fn text(&self) -> EvaluateResult<String> {
        Ok(self.text.clone())
    }
}
