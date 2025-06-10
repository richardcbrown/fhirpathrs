use crate::{
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::identifier::Identifier,
};

use super::resource_node::ResourceNode;

impl Evaluate for Identifier {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> EvaluateResult<ResourceNode<'a>> {
        match self {
            Identifier::LiteralIdentifier(exp) => exp.evaluate(input),
            Identifier::LiteralAs(exp) => exp.evaluate(input),
            Identifier::LiteralContains(exp) => exp.evaluate(input),
            Identifier::LiteralDelimitedIdentifier(exp) => exp.evaluate(input),
            Identifier::LiteralIn(exp) => todo!(),
            Identifier::LiteralIs(exp) => exp.evaluate(input),
        }
    }
}

impl Text for Identifier {
    fn text(&self) -> EvaluateResult<String> {
        match self {
            Identifier::LiteralIdentifier(exp) => exp.text(),
            Identifier::LiteralAs(exp) => exp.text(),
            Identifier::LiteralContains(exp) => exp.text(),
            Identifier::LiteralDelimitedIdentifier(exp) => exp.text(),
            Identifier::LiteralIn(exp) => todo!(),
            Identifier::LiteralIs(exp) => exp.text(),
        }
    }
}
