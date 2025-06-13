use serde_json::Value;

use crate::{
    error::FhirpathError,
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::expression::IndexerExpression,
};

use super::resource_node::ResourceNode;

impl Evaluate for IndexerExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> EvaluateResult<ResourceNode<'a>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::EvaluateError {
                msg: "IndexerExpression must have exactly two children".to_string(),
            });
        }

        let array_node = self.children[0].evaluate(input)?;
        let index_node = self.children[1].evaluate(input)?;

        let index_value = index_node.get_single()?;

        let index: i64 = match index_value {
            Value::Number(num) => {
                let fpn: f64 = num.as_f64().ok_or(FhirpathError::EvaluateError {
                    msg: "IndexerExpression index is not a Number".to_string(),
                })?;

                // @todo check conversion here
                Ok(fpn as i64)
            }
            Value::String(num_string) => {
                num_string.parse().map_err(|_| FhirpathError::EvaluateError {
                    msg: "IndexerExpression index is not a Number".to_string(),
                })
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: "IndexerExpression index is not a Number".to_string(),
            }),
        }?;

        let array_data = array_node.get_array()?;

        let mut node = ResourceNode::from_node(input, array_data[index as usize].clone());

        node.fhir_types = array_node.fhir_types;
        node.path = array_node.path;

        Ok(node)
    }
}

impl Text for IndexerExpression {
    fn text(&self) -> EvaluateResult<String> {
        let first = self.children.first().ok_or(FhirpathError::EvaluateError {
            msg: "First child of IndexerExpression should be an identifier".to_string(),
        })?.text()?;

        let second = self.children.iter().nth(1).ok_or(FhirpathError::EvaluateError {
            msg: "Second child of IndexerExpression should be an index".to_string(),
        })?.text()?;

        Ok(format!("{}[{}]", first, second))
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluate::{compile, Text};

    #[test]
    fn test_indexer_expression_text() {
        let compiled = compile(&"Patient.name[0]".to_string());

        let text = compiled.unwrap().expression.text().unwrap();
        assert_eq!(text, "Patient.name[0]");
    }
}