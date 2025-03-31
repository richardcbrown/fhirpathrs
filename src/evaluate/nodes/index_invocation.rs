use serde_json::Value;

use crate::{
    evaluate::{CompileResult, Evaluate, Text},
    parser::invocation::IndexInvocation,
};

use super::resource_node::ResourceNode;

impl Evaluate for IndexInvocation {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let index = input
            .get_index()
            .and_then(|index| Some(Value::Number(index.into())))
            .unwrap_or(Value::Array(vec![]));

        Ok(ResourceNode::from_node(input, index))
    }
}

impl Text for IndexInvocation {
    fn text(&self) -> CompileResult<String> {
        Ok(self.text.clone())
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::evaluate::test::test::{run_tests, TestCase};

    #[test]
    fn test_index_path() {
        let patient = json!({ "resourceType": "Patient", "a": [6, 5, 4, 3, 2, 1] });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.where($index > 0)".to_string(),
                input: patient.clone(),
                expected: json!([5, 4, 3, 2, 1]),
                options: None,
            },
            TestCase {
                path: "Patient.a.select($index)".to_string(),
                input: patient.clone(),
                expected: json!([0, 1, 2, 3, 4, 5]),
                options: None,
            },
            TestCase {
                path: "Patient.a.all($index < 7)".to_string(),
                input: patient.clone(),
                expected: json!([true]),
                options: None,
            },
        ];

        run_tests(test_cases);
    }
}
