use serde_json::Value;

use crate::{
    evaluate::{EvaluateResult, Evaluate, Text},
};
use crate::parser::literal::NullLiteral;
use super::resource_node::ResourceNode;

impl Evaluate for NullLiteral {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        Ok(ResourceNode::from_node(input, Value::Array(vec![])))
    }
}

impl Text for NullLiteral {
    fn text(&self) -> EvaluateResult<String> {
        Ok("{}".to_string())
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use crate::evaluate::test::test::{run_tests, Expected, TestCase};

    #[test]
    fn test_delimited_path() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "{}".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "name": [
                        {
                            "given": ["test1", "test2"]
                        }
                    ]
                }),
                expected: Expected::Value(json!([])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }
}