use crate::{
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::expression::ExpressionAndTypeSpecifier,
};

use super::resource_node::ResourceNode;

impl Evaluate for ExpressionAndTypeSpecifier {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        match self {
            ExpressionAndTypeSpecifier::Expression(expr) => expr.evaluate(input),
            ExpressionAndTypeSpecifier::TypeSpecifier(ts) => ts.evaluate(input),
        }
    }
}

impl Text for ExpressionAndTypeSpecifier {
    fn text(&self) -> EvaluateResult<String> {
        match self {
            ExpressionAndTypeSpecifier::Expression(expr) => expr.text(),
            ExpressionAndTypeSpecifier::TypeSpecifier(ts) => ts.text(),
        }
    }
}
