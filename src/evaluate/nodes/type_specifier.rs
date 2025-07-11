use crate::{
    error::FhirpathError,
    evaluate::{data_types::type_info::TypeInfo, EvaluateResult, Evaluate, Text},
    parser::{
        expression::{Expression, Term, TermExpression},
        identifier::{Identifier, LiteralIdentifier, QualifiedIdentifier, TypeSpecifier},
        literal::{Literal, LiteralTerm, StringLiteral},
    },
};

use super::resource_node::ResourceNode;

impl Evaluate for TypeSpecifier {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        let specifier = match self {
            TypeSpecifier::QualifiedIdentifier(qi) => qi.evaluate(input),
        }?;

        Ok(ResourceNode::from_node(
            input,
            serde_json::to_value(TypeInfo::try_from(&specifier)?).map_err(|err| {
                FhirpathError::EvaluateError {
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

impl TryInto<Expression> for String {
    type Error = FhirpathError;

    fn try_into(self) -> Result<Expression, Self::Error> {
        Ok(Expression::TermExpression(Box::new(TermExpression {
            children: vec![Box::new(Term::LiteralTerm(Box::new(LiteralTerm {
                children: vec![Box::new(Literal::StringLiteral(Box::new(StringLiteral {
                    text: self.clone(),
                })))],
            })))],
        })))
    }
}

impl Text for TypeSpecifier {
    fn text(&self) -> EvaluateResult<String> {
        match self {
            TypeSpecifier::QualifiedIdentifier(qi) => qi.text(),
        }
    }
}
