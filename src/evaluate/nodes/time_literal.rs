use serde_json::json;

use crate::{
    evaluate::{data_types::time::Time, EvaluateResult, Evaluate, Text},
    parser::literal::TimeLiteral,
};

use super::resource_node::ResourceNode;

impl Evaluate for TimeLiteral {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        Ok(ResourceNode::from_node(
            input,
            json!(Time::try_from(&self.text)?),
        ))
    }
}

impl Text for TimeLiteral {
    fn text(&self) -> EvaluateResult<String> {
        Ok(self.text.clone())
    }
}
