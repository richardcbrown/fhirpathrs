use crate::{
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::invocation::Invocation,
};

use super::resource_node::ResourceNode;

impl Evaluate for Invocation {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        match self {
            Invocation::MemberInvocation(exp) => exp.evaluate(input),
            Invocation::FunctionInvocation(exp) => exp.evaluate(input),
            Invocation::IndexInvocation(exp) => exp.evaluate(input),
            Invocation::ThisInvocation(exp) => exp.evaluate(input),
            Invocation::TotalInvocation(exp) => exp.evaluate(input),
        }
    }
}

impl Text for Invocation {
    fn text(&self) -> EvaluateResult<String> {
        match self {
            Invocation::MemberInvocation(exp) => exp.text(),
            Invocation::FunctionInvocation(exp) => exp.text(),
            Invocation::IndexInvocation(exp) => exp.text(),
            Invocation::ThisInvocation(exp) => exp.text(),
            Invocation::TotalInvocation(exp) => exp.text(),
        }
    }
}
