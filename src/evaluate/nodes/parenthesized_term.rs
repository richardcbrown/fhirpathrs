use crate::{
    error::FhirpathError,
    evaluate::{CompileResult, Evaluate, Text},
    parser::expression::ParenthesizedTerm,
};

use super::resource_node::ResourceNode;

impl Evaluate for ParenthesizedTerm {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> CompileResult<ResourceNode<'a, 'b>> {
        let expression = self
            .children
            .first()
            .ok_or_else(|| FhirpathError::CompileError {
                msg: "ParenthesizedTerm must have exactly one child".to_string(),
            })?;

        Ok(expression.evaluate(input)?)
    }
}

impl Text for ParenthesizedTerm {
    fn text(&self) -> CompileResult<String> {
        Ok(format!(
            "({})",
            self.children
                .iter()
                .map(|c| c.text())
                .collect::<CompileResult<Vec<String>>>()?
                .join(""),
        ))
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::evaluate::test::test::{run_tests, TestCase};

    #[test]
    fn test_parenthesized_path() {
        let patient = json!({ "resourceType": "Patient", "a": true, "b": false, "c": true });
        let patient2 = json!({ "resourceType": "Patient", "a": 1, "b": 2, "c": 3 });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "(Patient.a or Patient.b or Patient.c)".to_string(),
                input: patient.clone(),
                expected: json!([true]),
                options: None,
            },
            TestCase {
                path: "(Patient.a and (Patient.b or Patient.c))".to_string(),
                input: patient.clone(),
                expected: json!([true]),
                options: None,
            },
            TestCase {
                path: "((Patient.a + Patient.b) * Patient.c)".to_string(),
                input: patient2.clone(),
                expected: json!([9]),
                options: None,
            },
            TestCase {
                path: "(Patient.a + (Patient.b * Patient.c))".to_string(),
                input: patient2.clone(),
                expected: json!([7]),
                options: None,
            },
        ];

        run_tests(test_cases);
    }
}
