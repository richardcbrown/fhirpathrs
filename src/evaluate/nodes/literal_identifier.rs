use serde_json::json;

use crate::{
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::identifier::LiteralIdentifier,
};

use super::resource_node::ResourceNode;

impl Evaluate for LiteralIdentifier {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        Ok(ResourceNode::from_node(input, json!(self.text.clone())))
    }
}

impl Text for LiteralIdentifier {
    fn text(&self) -> EvaluateResult<String> {
        Ok(self.text.clone())
    }
}
