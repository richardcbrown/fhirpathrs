use crate::{
    error::FhirpathError,
    evaluate::{types::type_info::TypeInfo, CompileResult, Evaluate, Text},
    parser::{
        expression::Expression,
        identifier::{Identifier, LiteralIdentifier, QualifiedIdentifier, TypeSpecifier},
    },
};

use super::resource_node::ResourceNode;

impl Evaluate for TypeSpecifier {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let specifier = match self {
            TypeSpecifier::QualifiedIdentifier(qi) => qi.evaluate(input),
        }?;

        Ok(ResourceNode::from_node(
            input,
            serde_json::to_value(TypeInfo::try_from(&specifier)?).map_err(|err| {
                FhirpathError::CompileError {
                    msg: format!("Failed to serialize TypeInfo: {}", err.to_string()),
                }
            })?,
        ))
    }
}

impl TryFrom<&Expression> for TypeSpecifier {
    type Error = FhirpathError;

    fn try_from(value: &Expression) -> Result<Self, Self::Error> {
        let text = value.text()?;

        let text_items: Vec<&str> = text.split(".").collect();

        // @todo only handles literal identifier
        let children = text_items
            .iter()
            .map(|ti| {
                Box::new(Identifier::LiteralIdentifier(Box::new(LiteralIdentifier {
                    text: ti.to_string(),
                })))
            })
            .collect();

        Ok(TypeSpecifier::QualifiedIdentifier(Box::new(
            QualifiedIdentifier { children },
        )))
    }
}

impl Text for TypeSpecifier {
    fn text(&self) -> CompileResult<String> {
        match self {
            TypeSpecifier::QualifiedIdentifier(qi) => qi.text(),
        }
    }
}
