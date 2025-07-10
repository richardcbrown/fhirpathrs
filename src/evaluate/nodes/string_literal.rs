use serde_json::json;

use crate::{
    evaluate::{EvaluateResult, Evaluate, Text},
    parser::literal::StringLiteral,
};

use super::resource_node::ResourceNode;

impl Evaluate for StringLiteral {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        Ok(ResourceNode::from_node(input, json!(self.text)))
    }
}

impl Text for StringLiteral {
    fn text(&self) -> EvaluateResult<String> {
        Ok(format!("'{}'", self.text.clone()))
    }
}
