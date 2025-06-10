use serde_json::Value;

use crate::{
    error::FhirpathError,
    evaluate::{CompileResult, Evaluate, Text},
    parser::expression::IndexerExpression,
};

use super::resource_node::ResourceNode;

impl Evaluate for IndexerExpression {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> CompileResult<ResourceNode<'a, 'b>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::CompileError {
                msg: "IndexerExpression must have exactly two children".to_string(),
            });
        }

        let array_node = self.children[0].evaluate(input)?;
        let index_node = self.children[1].evaluate(input)?;

        let index_value = index_node.get_single()?;

        let index: i64 = match index_value {
            Value::Number(num) => {
                let fpn: f64 = num.as_f64().ok_or(FhirpathError::ParserError {
                    msg: "IndexerExpression index is not a Number".to_string(),
                })?;

                // @todo check conversion here
                Ok(fpn as i64)
            }
            Value::String(num_string) => {
                num_string.parse().map_err(|_| FhirpathError::ParserError {
                    msg: "IndexerExpression index is not a Number".to_string(),
                })
            }
            _ => Err(FhirpathError::ParserError {
                msg: "IndexerExpression index is not a Number".to_string(),
            }),
        }?;

        let array_data = array_node.get_array()?;

        dbg!(array_data.clone());
        dbg!(array_node.fhir_types.clone());
        dbg!(input.path.clone());
        dbg!(input.fhir_types.clone());

        let mut node = ResourceNode::from_node(input, array_data[index as usize].clone());

        node.fhir_types = array_node.fhir_types;
        node.path = array_node.path;

        Ok(node)
    }
}

impl Text for IndexerExpression {
    fn text(&self) -> CompileResult<String> {
        todo!()
    }
}
