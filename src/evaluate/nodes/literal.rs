use crate::{
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::literal::Literal,
};

use super::resource_node::ResourceNode;

impl Evaluate for Literal {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        match self {
            Literal::BooleanLiteral(exp) => exp.evaluate(input),
            Literal::DatetimeLiteral(exp) => exp.evaluate(input),
            Literal::NullLiteral(exp) => exp.evaluate(input),
            Literal::NumberLiteral(exp) => exp.evaluate(input),
            Literal::QuantityLiteral(exp) => exp.evaluate(input),
            Literal::StringLiteral(exp) => exp.evaluate(input),
            Literal::TimeLiteral(exp) => exp.evaluate(input),
        }
    }
}

impl Text for Literal {
    fn text(&self) -> EvaluateResult<String> {
        match self {
            Literal::BooleanLiteral(exp) => exp.text(),
            Literal::DatetimeLiteral(exp) => exp.text(),
            Literal::NullLiteral(exp) => exp.text(),
            Literal::NumberLiteral(exp) => exp.text(),
            Literal::QuantityLiteral(exp) => exp.text(),
            Literal::StringLiteral(exp) => exp.text(),
            Literal::TimeLiteral(exp) => exp.text(),
        }
    }
}
