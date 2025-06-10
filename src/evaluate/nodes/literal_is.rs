use serde_json::json;

use crate::{
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::identifier::LiteralIs,
};

use super::resource_node::ResourceNode;

impl Evaluate for LiteralIs {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> EvaluateResult<ResourceNode<'a>> {
        Ok(ResourceNode::from_node(input, json!(self.text.clone())))
    }
}

impl Text for LiteralIs {
    fn text(&self) -> EvaluateResult<String> {
        Ok(self.text.clone())
    }
}
