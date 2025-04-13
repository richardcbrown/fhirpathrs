use crate::{
    error::FhirpathError,
    evaluate::{utils::get_string, CompileResult, Evaluate, Text},
    parser::expression::IdentifierOrStringLiteral,
};

use super::resource_node::ResourceNode;

impl Evaluate for IdentifierOrStringLiteral {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let result = match &self {
            IdentifierOrStringLiteral::Identifier(identifier) => identifier.evaluate(input),
            IdentifierOrStringLiteral::StringLiteral(literal) => literal.evaluate(input),
        }?;

        let var_name = get_string(&result.get_single()?)?;

        let variable = input
            .get_var(&var_name)
            .ok_or_else(|| FhirpathError::CompileError {
                msg: format!("Unknown variable {}", var_name),
            })?;

        Ok(ResourceNode::from_node(input, variable))
    }
}

impl Text for IdentifierOrStringLiteral {
    fn text(&self) -> CompileResult<String> {
        match &self {
            IdentifierOrStringLiteral::Identifier(identifier) => identifier.text(),
            IdentifierOrStringLiteral::StringLiteral(literal) => literal.text(),
        }
    }
}
