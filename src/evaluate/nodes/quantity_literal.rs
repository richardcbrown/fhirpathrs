use crate::{
    error::FhirpathError,
    evaluate::{data_types::quantity::Quantity, EvaluateResult, Evaluate, Text},
    parser::literal::QuantityLiteral,
};
use crate::evaluate::fhir_type::determine_fhir_type;
use super::resource_node::ResourceNode;

impl Evaluate for QuantityLiteral {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        let q_val = serde_json::to_value(Quantity::try_from(self)?).map_err(|err| {
            FhirpathError::EvaluateError {
                msg: format!("Could not serialize Quantity: {}", err.to_string()),
            }
        })?;

        let fhir_types = vec![determine_fhir_type(Some(&q_val), None, input.context, false)];

        let mut node = ResourceNode::from_node(input, q_val);

        node.fhir_types = fhir_types;

        Ok(node)
    }
}

impl Text for QuantityLiteral {
    fn text(&self) -> EvaluateResult<String> {
        Ok(self.text.clone())
    }
}
