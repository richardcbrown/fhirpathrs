use crate::{
    evaluate::{CompileResult, Evaluate, Text},
    parser::expression::ExpressionAndTypeSpecifier,
};

use super::resource_node::ResourceNode;

impl Evaluate for ExpressionAndTypeSpecifier {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        match self {
            ExpressionAndTypeSpecifier::Expression(expr) => expr.evaluate(input),
            ExpressionAndTypeSpecifier::TypeSpecifier(ts) => ts.evaluate(input),
        }
    }
}

impl Text for ExpressionAndTypeSpecifier {
    fn text(&self) -> CompileResult<String> {
        match self {
            ExpressionAndTypeSpecifier::Expression(expr) => expr.text(),
            ExpressionAndTypeSpecifier::TypeSpecifier(ts) => ts.text(),
        }
    }
}
