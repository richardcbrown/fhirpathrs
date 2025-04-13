use serde_json::json;

use crate::{
    evaluate::{CompileResult, Evaluate, Text},
    parser::identifier::LiteralIdentifier,
};

use super::resource_node::ResourceNode;

impl Evaluate for LiteralIdentifier {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        Ok(ResourceNode::from_node(input, json!(self.text.clone())))
    }
}

impl Text for LiteralIdentifier {
    fn text(&self) -> CompileResult<String> {
        Ok(self.text.clone())
    }
}
