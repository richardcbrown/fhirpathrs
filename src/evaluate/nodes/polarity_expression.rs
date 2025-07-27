use std::ops::Neg;
use serde_json::Value;

use crate::{
    error::FhirpathError,
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::expression::PolarityExpression,
};
use crate::evaluate::data_types::quantity::Quantity;
use crate::evaluate::utils::{decimal_to_number, get_decimal_from_expression, number_to_decimal};
use super::resource_node::ResourceNode;

impl Evaluate for PolarityExpression {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        let child = self.children.first();

        child
            .ok_or(FhirpathError::EvaluateError {
                msg: "PolarityExpression must have a single child expression".to_string(),
            })
            .and_then(|child| child.evaluate(input))
            .and_then(|evaluated| {
                let value = evaluated.get_single()?;

                match value {
                    Value::Number(num) => {
                        let mut decimal = number_to_decimal(&num)?;

                        if self.op == "-" {
                            decimal = decimal.neg();
                        }

                        Ok(Value::Number(decimal_to_number(decimal)?))
                    },
                    Value::String(string_value) => {
                        let mut quantity = Quantity::try_from(&string_value)?;

                        if self.op == "-" {
                            quantity.value =  quantity.value.neg();
                        }

                        Ok(Value::String(quantity.to_string()))
                    },
                    Value::Object(_) => {
                        let value = evaluated.get_single()?;

                        let mut  quantity = Quantity::try_from(&value)?;

                        if self.op == "-" {
                            quantity.value =  quantity.value.neg();
                        }

                        Ok(Value::String(quantity.to_string()))
                    },
                    _ => Err(FhirpathError::EvaluateError {
                        msg: "Polarity not supported for value".to_string()
                    })
                }
            })
            .and_then(|result| Ok(ResourceNode::from_node(input, result)))
    }
}

impl Text for PolarityExpression {
    fn text(&self) -> EvaluateResult<String> {
        Ok(format!(
            "{}{}",
            self.op.clone(),
            self.children
                .iter()
                .map(|c| c.text())
                .collect::<EvaluateResult<Vec<String>>>()?
                .join("")
        ))
    }
}
