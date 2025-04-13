use crate::{
    evaluate::{CompileResult, Evaluate, Text},
    parser::identifier::Identifier,
};

use super::resource_node::ResourceNode;

impl Evaluate for Identifier {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        match self {
            Identifier::LiteralIdentifier(exp) => exp.evaluate(input),
            Identifier::LiteralAs(exp) => todo!(),
            Identifier::LiteralContains(exp) => exp.evaluate(input),
            Identifier::LiteralDelimitedIdentifier(exp) => todo!(),
            Identifier::LiteralIn(exp) => todo!(),
            Identifier::LiteralIs(exp) => todo!(),
        }
    }
}

impl Text for Identifier {
    fn text(&self) -> CompileResult<String> {
        match self {
            Identifier::LiteralIdentifier(exp) => exp.text(),
            Identifier::LiteralAs(exp) => todo!(),
            Identifier::LiteralContains(exp) => exp.text(),
            Identifier::LiteralDelimitedIdentifier(exp) => todo!(),
            Identifier::LiteralIn(exp) => todo!(),
            Identifier::LiteralIs(exp) => todo!(),
        }
    }
}
