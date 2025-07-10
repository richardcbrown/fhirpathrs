use serde_json::{json, Value};

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    equality::{values_are_equal},
    filtering::where_function,
    target::Target,
    utils::{
        evaluate_array_boolean_expression, get_array_from_expression, get_arrays,
        try_convert_to_boolean, unique_array_elements,
    },
    EvaluateResult, Evaluate, ResourceNode,
};

pub fn empty<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    Ok(ResourceNode::from_node(input, json!(input.is_empty()?)))
}

pub fn exists<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if expressions.len() == 0 {
        return Ok(ResourceNode::from_node(input, json!(!input.is_empty()?)));
    }

    let result = where_function(input, expressions)?;

    Ok(ResourceNode::from_node(input, json!(!result.is_empty()?)))
}

pub fn all<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if expressions.len() != 1 {
        return Err(FhirpathError::EvaluateError {
            msg: "Expected single expression".to_string(),
        });
    }

    let expr = expressions
        .first()
        .ok_or_else(|| FhirpathError::EvaluateError {
            msg: "Expected single expression".to_string(),
        })?;

    let results = evaluate_array_boolean_expression(input, expr)?;

    Ok(ResourceNode::from_node(
        input,
        json!(results.iter().all(|item| *item)),
    ))
}

fn input_to_bool_array<'a, 'b>(input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<Vec<bool>> {
    let array = input.get_array()?;

    array
        .iter()
        .map(|item| {
            let bool_result = try_convert_to_boolean(item);

            match bool_result {
                Some(bool) => Ok(bool),
                None => Err(FhirpathError::EvaluateError {
                    msg: "Value was not a boolean".to_string(),
                }),
            }
        })
        .collect::<EvaluateResult<Vec<bool>>>()
}

pub fn all_true<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let array = input_to_bool_array(input)?;

    let all_true = array.iter().all(|item| *item);

    Ok(ResourceNode::from_node(input, json!(all_true)))
}

pub fn any_true<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let array = input_to_bool_array(input)?;

    let any_true = array.iter().any(|item| *item);

    Ok(ResourceNode::from_node(input, json!(any_true)))
}

pub fn all_false<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let array = input_to_bool_array(input)?;

    let all_false = array.iter().all(|item| !*item);

    Ok(ResourceNode::from_node(input, json!(all_false)))
}

pub fn any_false<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let array = input_to_bool_array(input)?;

    let any_false = array.iter().any(|item| !*item);

    Ok(ResourceNode::from_node(input, json!(any_false)))
}

pub fn subset_of<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
    target: Target,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let (first_array, second_array) = get_arrays(input, expressions, target)?;

    let is_subset = first_array.iter().all(|self_item| {
        second_array
            .iter()
            .any(|other_item| values_are_equal(self_item, other_item))
    });

    Ok(ResourceNode::from_node(input, json!(is_subset)))
}

pub fn superset_of<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
    target: Target,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let (first_array, second_array) = get_arrays(input, expressions, target)?;

    let is_superset = second_array.iter().all(|self_item| {
        first_array
            .iter()
            .any(|other_item| values_are_equal(self_item, other_item))
    });

    Ok(ResourceNode::from_node(input, json!(is_superset)))
}

pub fn count<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let array = input.get_array()?;

    Ok(ResourceNode::from_node(input, json!(array.len())))
}

pub fn distinct<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let array = unique_array_elements(input.get_array()?);

    Ok(ResourceNode::from_node(input, Value::Array(array)))
}

pub fn is_distinct<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let total_array = input.get_array()?;

    let array = unique_array_elements(total_array);

    let is_distinct = total_array.len() == array.len();

    Ok(ResourceNode::from_node(input, Value::Bool(is_distinct)))
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use crate::evaluate::EvaluateOptions;
    use crate::evaluate::test::test::{run_tests, Expected, TestCase};
    use crate::models::{get_model_details, ModelType};

    #[test]
    fn test_empty_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2,3,4],
            "generalPractitioner": [
                {
                    "reference": "Practitioner/123"
                }
            ],
            "b": [],
            "contained": [
                {
                    "resourceType": "Patient"
                }
            ]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "{}.empty()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.a.empty()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([false])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_all_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2,3,4],
            "generalPractitioner": [
                {
                    "reference": "Practitioner/123"
                }
            ],
            "b": [],
            "contained": [
                {
                    "resourceType": "Patient"
                }
            ]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.b.all($this = true)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.a.all($this > 0)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.a.all($this > 1)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([false])),
                options: None,
            },
            // @todo - should pass according to spec
            // but doesn't as we don't reset the path
            // for contained resources
            // TestCase {
            //     path: "Patient.contained.all($this is Patient)".to_string(),
            //     input: patient.clone(),
            //     expected: Expected::Value(json!([true]),
            //     options: Some(EvaluateOptions {
            //         model: Some(get_model_details(ModelType::Stu3).unwrap()),
            //         vars: None,
            //         now: None,
            //     }),
            // },
            // @todo - should pass according to spec
            // but doesn't as we cannot yet determine the
            // type of a reference
            // TestCase {
            //     path: "Patient.generalPractitioner.all($this is Patient)".to_string(),
            //     input: patient.clone(),
            //     expected: Expected::Value(json!([true]),
            //     options: Some(EvaluateOptions {
            //         model: Some(get_model_details(ModelType::Stu3).unwrap()),
            //         vars: None,
            //         now: None,
            //     }),
            // },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_all_true_path() {
        let patient = json!({ "resourceType": "Patient", "a": [true, false], "b": [] });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.allTrue()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([false])),
                options: None,
            },
            TestCase {
                path: "Patient.a.select(value = true).allTrue()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.b.allTrue()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([true])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_any_true_path() {
        let patient = json!({ "resourceType": "Patient", "a": [true] });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.anyTrue()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.a.select(value > 2).anyTrue()".to_string(),
                input: json!({ "resourceType": "Patient", "a": [{ "value": 1 }] }),
                expected: Expected::Value(json!([false])),
                options: None,
            },
            TestCase {
                path: "Patient.a.select(value > 2).anyTrue()".to_string(),
                input: json!({ "resourceType": "Patient", "a": [{ "value": 1 }, { "value": 1 }, { "value": 2 }, { "value": 3 }] }),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.a.select(value > 2).anyTrue()".to_string(),
                input: json!({ "resourceType": "Patient", "a": [] }),
                expected: Expected::Value(json!([false])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_all_false_path() {
        let patient = json!({ "resourceType": "Patient", "a": [false] });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.allFalse()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.a.select(value > 2).allFalse()".to_string(),
                input: json!({ "resourceType": "Patient", "a": [{ "value": 1 }] }),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.a.select(value > 2).allFalse()".to_string(),
                input: json!({ "resourceType": "Patient", "a": [{ "value": 1 }, { "value": 1 }, { "value": 2 }, { "value": 2 }] }),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.a.select(value > 2).allFalse()".to_string(),
                input: json!({ "resourceType": "Patient", "a": [{ "value": 3 }, { "value": 1 }, { "value": 2 }, { "value": 2 }] }),
                expected: Expected::Value(json!([false])),
                options: None,
            },
            TestCase {
                path: "Patient.a.select(value > 2).allFalse()".to_string(),
                input: json!({ "resourceType": "Patient", "a": [] }),
                expected: Expected::Value(json!([true])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_any_false_path() {
        let patient = json!({ "resourceType": "Patient", "a": [false] });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.anyFalse()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.a.select(value > 2).anyFalse()".to_string(),
                input: json!({ "resourceType": "Patient", "a": [{ "value": 1 }] }),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.a.select(value > 2).anyFalse()".to_string(),
                input: json!({ "resourceType": "Patient", "a": [{ "value": 3 }, { "value": 3 }, { "value": 3 }, { "value": 2 }] }),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.a.select(value > 2).anyFalse()".to_string(),
                input: json!({ "resourceType": "Patient", "a": [{ "value": 3 }, { "value": 3 }, { "value": 3 }, { "value": 3 }] }),
                expected: Expected::Value(json!([false])),
                options: None,
            },
            TestCase {
                path: "Patient.a.select(value > 2).anyFalse()".to_string(),
                input: json!({ "resourceType": "Patient", "a": [] }),
                expected: Expected::Value(json!([false])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_subset_of_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": [false],
            "b": [false]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.subsetOf(Patient.b)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.a.subsetOf(Patient.b)".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": [true, false],
                    "b": [false]
                }),
                expected: Expected::Value(json!([false])),
                options: None,
            },
            TestCase {
                path: "Patient.a.subsetOf(Patient.b)".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": [true],
                    "b": [true, false]
                }),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.a.subsetOf(Patient.b)".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": [],
                    "b": [false]
                }),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.a.subsetOf(Patient.b)".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": [false],
                    "b": []
                }),
                expected: Expected::Value(json!([false])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_superset_of_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": [false],
            "b": [false]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.supersetOf(Patient.b)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.a.supersetOf(Patient.b)".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": [true, false],
                    "b": [false]
                }),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.a.supersetOf(Patient.b)".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": [true],
                    "b": [true, false]
                }),
                expected: Expected::Value(json!([false])),
                options: None,
            },
            TestCase {
                path: "Patient.a.supersetOf(Patient.b)".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": [],
                    "b": [false]
                }),
                expected: Expected::Value(json!([false])),
                options: None,
            },
            TestCase {
                path: "Patient.a.supersetOf(Patient.b)".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": [false],
                    "b": []
                }),
                expected: Expected::Value(json!([true])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_count_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": [{ "value": true }, { "value": false }]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.count()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([2])),
                options: None,
            },
            TestCase {
                path: "Patient.a.where(value = true).count()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1])),
                options: None,
            },
            TestCase {
                path: "Patient.a.count()".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": [],
                }),
                expected: Expected::Value(json!([0])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_distinct_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": ['a', 'a', 'b']
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.distinct()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!(['a', 'b'])),
                options: None,
            },
            TestCase {
                path: "Patient.a.distinct()".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": []
                }),
                expected: Expected::Value(json!([])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_is_distinct_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": ['a', 'a', 'b']
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.isDistinct()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([false])),
                options: None,
            },
            TestCase {
                path: "Patient.a.isDistinct()".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": []
                }),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.a.isDistinct()".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "a": ['a', 'b', 'c']
                }),
                expected: Expected::Value(json!([true])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }
}
