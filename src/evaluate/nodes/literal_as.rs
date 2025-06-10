use serde_json::json;

use crate::{
    evaluate::{CompileResult, Evaluate, Text},
    parser::identifier::LiteralAs,
};

use super::resource_node::ResourceNode;

impl Evaluate for LiteralAs {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> CompileResult<ResourceNode<'a, 'b>> {
        Ok(ResourceNode::from_node(input, json!(self.text.clone())))
    }
}

impl Text for LiteralAs {
    fn text(&self) -> CompileResult<String> {
        Ok(self.text.clone())
    }
}
