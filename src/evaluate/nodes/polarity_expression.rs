use std::ops::Neg;
use serde_json::Value;

use crate::{
    error::FhirpathError,
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::expression::PolarityExpression,
};
use crate::evaluate::utils::{decimal_to_number, get_decimal_from_expression};
use super::resource_node::ResourceNode;

impl Evaluate for PolarityExpression {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        let child = self.children.first();

        child
            .ok_or(FhirpathError::EvaluateError {
                msg: "PolarityExpression must have a single child expression".to_string(),
            })
            .and_then(|child_expr| Ok(get_decimal_from_expression(input, child_expr)?))
            .and_then(|decimal| {
                    let mut result = decimal;

                    if self.op == "-" {
                        result = -result.neg();
                    }

                    Ok(Value::Number(decimal_to_number(result)?))
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
