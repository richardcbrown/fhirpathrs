use crate::{
    evaluate::{CompileResult, Evaluate, Text},
    parser::expression::ExpressionAndInvocation,
};

use super::resource_node::ResourceNode;

impl Evaluate for ExpressionAndInvocation {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> CompileResult<ResourceNode<'a, 'b>> {
        match self {
            ExpressionAndInvocation::Expression(expr) => expr.evaluate(input),
            ExpressionAndInvocation::Invocation(invocation) => invocation.evaluate(input),
        }
    }
}

impl Text for ExpressionAndInvocation {
    fn text(&self) -> CompileResult<String> {
        match self {
            ExpressionAndInvocation::Expression(expr) => expr.text(),
            ExpressionAndInvocation::Invocation(invocation) => invocation.text(),
        }
    }
}
