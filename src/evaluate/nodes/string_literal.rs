use serde_json::json;

use crate::{
    evaluate::{CompileResult, Evaluate, Text},
    parser::literal::StringLiteral,
};

use super::resource_node::ResourceNode;

impl Evaluate for StringLiteral {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        Ok(ResourceNode::from_node(input, json!(self.text)))
    }
}

impl Text for StringLiteral {
    fn text(&self) -> CompileResult<String> {
        Ok(self.text.clone())
    }
}
