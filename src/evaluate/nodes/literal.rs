use crate::{
    evaluate::{CompileResult, Evaluate, Text},
    parser::literal::Literal,
};

use super::resource_node::ResourceNode;

impl Evaluate for Literal {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        match self {
            Literal::BooleanLiteral(exp) => exp.evaluate(input),
            Literal::DatetimeLiteral(exp) => exp.evaluate(input),
            Literal::NullLiteral(exp) => todo!(),
            Literal::NumberLiteral(exp) => exp.evaluate(input),
            Literal::QuantityLiteral(exp) => exp.evaluate(input),
            Literal::StringLiteral(exp) => exp.evaluate(input),
            Literal::TimeLiteral(exp) => exp.evaluate(input),
        }
    }
}

impl Text for Literal {
    fn text(&self) -> CompileResult<String> {
        match self {
            Literal::BooleanLiteral(exp) => exp.text(),
            Literal::DatetimeLiteral(exp) => exp.text(),
            Literal::NullLiteral(exp) => todo!(),
            Literal::NumberLiteral(exp) => exp.text(),
            Literal::QuantityLiteral(exp) => exp.text(),
            Literal::StringLiteral(exp) => exp.text(),
            Literal::TimeLiteral(exp) => exp.text(),
        }
    }
}
