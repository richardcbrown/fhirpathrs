use serde_json::json;

use crate::{
    evaluate::{EvaluateResult, Evaluate, Text},
};
use crate::parser::identifier::LiteralDelimitedIdentifier;
use super::resource_node::ResourceNode;

impl Evaluate for LiteralDelimitedIdentifier {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> EvaluateResult<ResourceNode<'a>> {
        Ok(ResourceNode::from_node(input, json!(self.text.clone())))
    }
}

impl Text for LiteralDelimitedIdentifier {
    fn text(&self) -> EvaluateResult<String> {
        Ok(format!("`{}`", self.text.clone()))
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
                path: "`Patient`.name.`given`".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "name": [
                        {
                            "given": ["test1", "test2"]
                        }
                    ]
                }),
                expected: Expected::Value(json!(["test1", "test2"])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }
}
