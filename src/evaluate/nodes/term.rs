use crate::{
    evaluate::{CompileResult, Evaluate, Text},
    parser::expression::Term,
};

use super::resource_node::ResourceNode;

impl Evaluate for Term {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        match self {
            Term::InvocationTerm(exp) => exp.evaluate(input),
            Term::LiteralTerm(exp) => exp.evaluate(input),
            Term::ExternalConstantTerm(exp) => exp.evaluate(input),
            Term::ParenthesizedTerm(exp) => exp.evaluate(input),
        }
    }
}

impl Text for Term {
    fn text(&self) -> CompileResult<String> {
        match self {
            Term::InvocationTerm(exp) => exp.text(),
            Term::LiteralTerm(exp) => exp.text(),
            Term::ExternalConstantTerm(exp) => exp.text(),
            Term::ParenthesizedTerm(exp) => exp.text(),
        }
    }
}
