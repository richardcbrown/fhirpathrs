use crate::{
    evaluate::{CompileResult, Evaluate, Text},
    parser::invocation::ThisInvocation,
};

use super::resource_node::ResourceNode;

impl Evaluate for ThisInvocation {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> CompileResult<ResourceNode<'a, 'b>> {
        Ok(ResourceNode::from_node(input, input.data.clone()))
    }
}

impl Text for ThisInvocation {
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
                path: "Patient.a.where($this > 4)".to_string(),
                input: patient.clone(),
                expected: json!([6, 5]),
                options: None,
            },
            TestCase {
                path: "Patient.a.select($this)".to_string(),
                input: patient.clone(),
                expected: json!([6, 5, 4, 3, 2, 1]),
                options: None,
            },
            TestCase {
                path: "Patient.a.all($this < 7)".to_string(),
                input: patient.clone(),
                expected: json!([true]),
                options: None,
            },
        ];

        run_tests(test_cases);
    }
}
