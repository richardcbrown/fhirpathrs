use serde_json::Value;

use crate::{
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::invocation::TotalInvocation,
};

use super::resource_node::ResourceNode;

impl Evaluate for TotalInvocation {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        let index = input.get_total().unwrap_or(Value::Array(vec![]));

        Ok(ResourceNode::from_node(input, index))
    }
}

impl Text for TotalInvocation {
    fn text(&self) -> EvaluateResult<String> {
        Ok(self.text.clone())
    }
}
