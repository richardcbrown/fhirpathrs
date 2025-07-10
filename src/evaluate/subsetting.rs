use serde_json::{json, Value};

use crate::{error::FhirpathError, parser::expression::Expression};
use crate::evaluate::utils::get_i32_from_expression;
use super::{
    target::Target,
    equality::values_are_equal,
    utils::{get_arrays, get_usize_from_expression, unique_array_elements},
    EvaluateResult, ResourceNode,
};

pub fn single<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(vec![])))
    }

    let array = input.get_array()?;

    if array.len() != 1 {
        return Err(FhirpathError::EvaluateError {
            msg: format!("Expected array with single element but had {}", array.len()),
        });
    }

    let single_value = array.first().ok_or_else(|| FhirpathError::EvaluateError {
        msg: "Failed to get single item from array".to_string(),
    })?;

    Ok(ResourceNode::from_node(input, single_value.clone()))
}

pub fn first<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let array = input.get_array()?;

    let first_value = array.first();

    Ok(ResourceNode::from_node(
        input,
        match first_value {
            Some(first) => json!(first),
            None => Value::Array(vec![]),
        },
    ))
}

pub fn last<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let array = input.get_array()?;

    let last_value = array.last();

    Ok(ResourceNode::from_node(
        input,
        match last_value {
            Some(first) => json!(first),
            None => Value::Array(vec![]),
        },
    ))
}

pub fn tail<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let array = input.get_array()?;

    let tail_values: Vec<&Value> = array.iter().skip(1).collect();

    Ok(ResourceNode::from_node(input, json!(tail_values)))
}

pub fn skip<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let array = input.get_array()?.to_vec();

    if expressions.len() > 1 {
        return Err(FhirpathError::EvaluateError {
            msg: "Skip expects exactly one expression".to_string(),
        });
    }

    let expression = expressions
        .first()
        .ok_or_else(|| FhirpathError::EvaluateError {
            msg: "Skip expects exactly one expression".to_string(),
        })?;

    let int_num = get_i32_from_expression(input, expression)?;

    if int_num <= 0 {
        return Ok(ResourceNode::from_node(input, Value::Array(array)));
    }

    let skip_num: usize = int_num.try_into().map_err(|e| FhirpathError::EvaluateError {
        msg: format!("Value cannot be converted to usize: {}", e),
    })?;

    Ok(ResourceNode::from_node(
        input,
        Value::Array(array.into_iter().skip(skip_num).collect::<Vec<Value>>()),
    ))
}

pub fn take<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let array = input.get_array()?.to_vec();

    if expressions.len() > 1 {
        return Err(FhirpathError::EvaluateError {
            msg: "Skip expects exactly one expression".to_string(),
        });
    }

    let expression = expressions
        .first()
        .ok_or_else(|| FhirpathError::EvaluateError {
            msg: "Skip expects exactly one expression".to_string(),
        })?;

    let int_num = get_i32_from_expression(input, expression)?;

    if int_num <= 0 {
        return Ok(ResourceNode::from_node(input, Value::Array(vec![])));
    }

    let take_num: usize = int_num.try_into().map_err(|e| FhirpathError::EvaluateError {
        msg: format!("Value cannot be converted to usize: {}", e),
    })?;

    Ok(ResourceNode::from_node(
        input,
        Value::Array(array.into_iter().take(take_num).collect::<Vec<Value>>()),
    ))
}

pub fn intersect<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let (array, second_array) = get_arrays(input, expressions, Target::AnyAtRoot)?;

    let intersect_array: Vec<Value> = array
        .into_iter()
        .filter(|item| {
            second_array
                .iter()
                .find(|second_item| values_are_equal(item, *second_item))
                .is_some()
        })
        .collect();

    Ok(ResourceNode::from_node(
        input,
        json!(unique_array_elements(&intersect_array)),
    ))
}

pub fn exclude<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let (array, second_array) = get_arrays(input, expressions, Target::AnyAtRoot)?;

    let exclude_array: Vec<Value> = array
        .into_iter()
        .filter(|item| {
            second_array
                .iter()
                .find(|second_item| values_are_equal(item, *second_item))
                .is_none()
        })
        .collect();

    Ok(ResourceNode::from_node(input, json!(exclude_array)))
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use crate::error::FhirpathError;
    use crate::evaluate::test::test::{run_tests, Expected, TestCase};

    #[test]
    fn test_single_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": [1],
            "b": [],
            "c": [1, 2, 3],
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.single()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1])),
                options: None,
            },
            TestCase {
                path: "Patient.b.single()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.c.single()".to_string(),
                input: patient.clone(),
                expected: Expected::Error(FhirpathError::EvaluateError { msg: "Expected array with single element but had 3".to_string() }),
                options: None,
            }
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_first_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": [1],
            "b": [],
            "c": [1, 2, 3],
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.first()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1])),
                options: None,
            },
            TestCase {
                path: "Patient.b.first()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.c.first()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1])),
                options: None,
            }
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_last_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": [1],
            "b": [],
            "c": [1, 2, 3],
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.last()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1])),
                options: None,
            },
            TestCase {
                path: "Patient.b.last()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.c.last()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([3])),
                options: None,
            }
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_tail_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": [1],
            "b": [],
            "c": [1, 2, 3],
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.tail()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.b.tail()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.c.tail()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([2, 3])),
                options: None,
            }
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_skip_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": [1],
            "b": [],
            "c": [1, 2, 3],
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.skip(0)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1])),
                options: None,
            },
            TestCase {
                path: "Patient.b.skip(1)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.c.skip(2)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([3])),
                options: None,
            },
            TestCase {
                path: "Patient.c.skip(-1)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1, 2, 3])),
                options: None,
            }
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_take_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": [1],
            "b": [],
            "c": [1, 2, 3],
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.take(0)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.b.take(1)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.c.take(2)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1, 2])),
                options: None,
            },
            TestCase {
                path: "Patient.c.take(-1)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            }
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_intersect_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": [1, 1],
            "b": [],
            "c": [1, 2, 3],
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.intersect(1)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1])),
                options: None,
            },
            TestCase {
                path: "Patient.b.intersect(1)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.c.intersect(1 | 2)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1, 2])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_exclude_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": [1, 1],
            "b": [],
            "c": [1, 2, 3, 3],
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.exclude(1)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.a.exclude(2)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([1, 1])),
                options: None,
            },
            TestCase {
                path: "Patient.b.exclude(1)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.c.exclude(1 | 2)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([3, 3])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }
}