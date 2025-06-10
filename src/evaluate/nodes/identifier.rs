use crate::{
    evaluate::{CompileResult, Evaluate, Text},
    parser::identifier::Identifier,
};

use super::resource_node::ResourceNode;

impl Evaluate for Identifier {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> CompileResult<ResourceNode<'a, 'b>> {
        match self {
            Identifier::LiteralIdentifier(exp) => exp.evaluate(input),
            Identifier::LiteralAs(exp) => exp.evaluate(input),
            Identifier::LiteralContains(exp) => exp.evaluate(input),
            Identifier::LiteralDelimitedIdentifier(exp) => todo!(),
            Identifier::LiteralIn(exp) => todo!(),
            Identifier::LiteralIs(exp) => exp.evaluate(input),
        }
    }
}

impl Text for Identifier {
    fn text(&self) -> CompileResult<String> {
        match self {
            Identifier::LiteralIdentifier(exp) => exp.text(),
            Identifier::LiteralAs(exp) => exp.text(),
            Identifier::LiteralContains(exp) => exp.text(),
            Identifier::LiteralDelimitedIdentifier(exp) => todo!(),
            Identifier::LiteralIn(exp) => todo!(),
            Identifier::LiteralIs(exp) => exp.text(),
        }
    }
}
