use std::str::FromStr;

use rust_decimal::Decimal;

use crate::{
    error::FhirpathError,
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::literal::NumberLiteral,
};
use crate::evaluate::fhir_type::determine_fhir_type;
use super::resource_node::ResourceNode;

impl Evaluate for NumberLiteral {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        let value =
            Decimal::from_str(self.text.as_str()).map_err(|_| FhirpathError::EvaluateError {
                msg: "NumberLiteral is not a Number".to_string(),
            })?;

        let decimal_value =
            serde_json::to_value(value).map_err(|err| FhirpathError::EvaluateError {
                msg: format!("Failed to serialize Decimal: {}", err.to_string()),
            })?;

        let fhir_types = vec![determine_fhir_type(Some(&decimal_value), None, input.context, false)];

        let mut node = ResourceNode::from_node(input, decimal_value);

        node.fhir_types = fhir_types;

        Ok(node)
    }
}

impl Text for NumberLiteral {
    fn text(&self) -> EvaluateResult<String> {
        Ok(self.text.clone())
    }
}
