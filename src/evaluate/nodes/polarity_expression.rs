use serde_json::{Number, Value};

use crate::{
    error::FhirpathError,
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::expression::PolarityExpression,
};

use super::resource_node::ResourceNode;

impl Evaluate for PolarityExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> EvaluateResult<ResourceNode<'a>> {
        let child = self.children.first();

        child
            .ok_or(FhirpathError::EvaluateError {
                msg: "PolarityExpression must have a single child expression".to_string(),
            })
            .and_then(|child_expr| child_expr.evaluate(input))
            .and_then(|result| Ok(result.get_single()?))
            .and_then(|expr_result| match expr_result {
                Value::Number(json_num) => {
                    let mut num: i64 = json_num.as_i64().ok_or(FhirpathError::EvaluateError {
                        msg: "PolarityExpression result was not a number".to_string(),
                    })?;

                    if self.op == "-" {
                        num = -num;
                    }

                    Ok(Value::Number(Number::from(num)))
                }
                _ => Err(FhirpathError::EvaluateError {
                    msg: "PolarityExpression result was not a number".to_string(),
                }),
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
