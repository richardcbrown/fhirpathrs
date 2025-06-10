use crate::{
    error::FhirpathError,
    evaluate::{CompileResult, Evaluate, Text},
    parser::expression::TypeExpression,
};

use super::{resource_node::ResourceNode, utils::invoke_operation};

impl Evaluate for TypeExpression {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> CompileResult<ResourceNode<'a, 'b>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::CompileError {
                msg: "TypeExpression must have exactly two children".to_string(),
            });
        }

        let expression = self.children.first().ok_or(FhirpathError::CompileError {
            msg: "Missing Expression".to_string(),
        })?;

        let type_specifier = self
            .children
            .iter()
            .nth(1)
            .ok_or(FhirpathError::CompileError {
                msg: "Missing Type Specifier".to_string(),
            })?;

        let specifier_text = type_specifier.text()?;

        let result = expression.evaluate(input)?;

        let invoke_result = invoke_operation(
            &self.op,
            &result,
            &vec![Box::new(specifier_text.try_into()?)],
        )?;

        Ok(ResourceNode::from_node(input, invoke_result.data))
    }
}

impl Text for TypeExpression {
    fn text(&self) -> CompileResult<String> {
        todo!()
    }
}
