use serde_json::json;

use crate::{
    evaluate::{data_types::date_time::DateTime, EvaluateResult, Evaluate, Text},
    parser::literal::DatetimeLiteral,
};

use super::resource_node::ResourceNode;

impl Evaluate for DatetimeLiteral {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        Ok(ResourceNode::from_node(
            input,
            json!(DateTime::try_from(&self.text)?),
        ))
    }
}

impl Text for DatetimeLiteral {
    fn text(&self) -> EvaluateResult<String> {
        Ok(self.text.clone())
    }
}
