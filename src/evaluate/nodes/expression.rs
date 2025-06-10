use crate::{
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::expression::Expression,
};

use super::resource_node::ResourceNode;

impl Evaluate for Expression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> EvaluateResult<ResourceNode<'a>> {
        match self {
            Expression::TermExpression(exp) => exp.evaluate(input),
            Expression::InvocationExpression(exp) => exp.evaluate(input),
            Expression::IndexerExpression(exp) => exp.evaluate(input),
            Expression::PolarityExpression(exp) => exp.evaluate(input),
            Expression::MultiplicativeExpression(exp) => exp.evaluate(input),
            Expression::AdditiveExpression(exp) => exp.evaluate(input),
            Expression::UnionExpression(exp) => exp.evaluate(input),
            Expression::InequalityExpression(exp) => exp.evaluate(input),
            Expression::TypeExpression(exp) => exp.evaluate(input),
            Expression::EqualityExpression(exp) => exp.evaluate(input),
            Expression::MembershipExpression(exp) => exp.evaluate(input),
            Expression::AndExpression(exp) => exp.evaluate(input),
            Expression::OrExpression(exp) => exp.evaluate(input),
            Expression::ImpliesExpression(exp) => todo!(),
        }
    }
}

impl Text for Expression {
    fn text(&self) -> EvaluateResult<String> {
        match self {
            Expression::TermExpression(exp) => exp.text(),
            Expression::InvocationExpression(exp) => exp.text(),
            Expression::IndexerExpression(exp) => exp.text(),
            Expression::PolarityExpression(exp) => exp.text(),
            Expression::MultiplicativeExpression(exp) => exp.text(),
            Expression::AdditiveExpression(exp) => exp.text(),
            Expression::UnionExpression(exp) => exp.text(),
            Expression::InequalityExpression(exp) => exp.text(),
            Expression::TypeExpression(exp) => exp.text(),
            Expression::EqualityExpression(exp) => exp.text(),
            Expression::MembershipExpression(exp) => exp.text(),
            Expression::AndExpression(exp) => exp.text(),
            Expression::OrExpression(exp) => exp.text(),
            Expression::ImpliesExpression(exp) => todo!(),
        }
    }
}
