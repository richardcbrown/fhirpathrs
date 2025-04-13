use serde_json::Value;

use crate::{
    evaluate::{CompileResult, Evaluate, Text},
    parser::invocation::TotalInvocation,
};

use super::resource_node::ResourceNode;

impl Evaluate for TotalInvocation {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let index = input.get_total().unwrap_or(Value::Array(vec![]));

        Ok(ResourceNode::from_node(input, index))
    }
}

impl Text for TotalInvocation {
    fn text(&self) -> CompileResult<String> {
        Ok(self.text.clone())
    }
}
