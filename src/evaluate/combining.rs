use serde_json::{json, Value};

use crate::parser::expression::Expression;

use super::{
    equality::values_are_equal,
    target::Target,
    utils::{get_arrays, unique_array_elements},
    EvaluateResult, ResourceNode,
};

pub fn union<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
    arity: Target,
) -> EvaluateResult<ResourceNode<'a>> {
    let (array, mut second_array) = get_arrays(input, expressions, arity)?;

    let mut union_array: Vec<Value> = array
        .into_iter()
        .filter(|item| {
            second_array
                .iter()
                .find(|second_item| values_are_equal(item, *second_item))
                .is_none()
        })
        .collect();

    union_array.append(&mut second_array);

    Ok(ResourceNode::from_node(
        input,
        json!(unique_array_elements(&union_array)),
    ))
}

pub fn combine<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a>> {
    let (mut first_array, mut second_array) = get_arrays(input, expressions, Target::AnyAtRoot)?;

    first_array.append(&mut second_array);

    Ok(ResourceNode::from_node(input, json!(first_array)))
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use crate::error::FhirpathError;
    use crate::evaluate::test::test::{run_tests, Expected, TestCase};

    #[test]
    fn test_union_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": [1],
            "b": [],
            "c": [1, 2, 3],
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a | Patient.c".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1, 2, 3])),
                options: None,
            },
            TestCase {
                path: "Patient.a.union(Patient.c)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1, 2, 3])),
                options: None,
            },
            TestCase {
                path: "Patient.b.union(Patient.b)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.a | Patient.b".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_combine_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": [1],
            "b": [],
            "c": [1, 2, 3],
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.combine(Patient.c)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1, 1, 2, 3])),
                options: None,
            },
            TestCase {
                path: "Patient.b.combine(Patient.b)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.a.combine(Patient.b)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }
}