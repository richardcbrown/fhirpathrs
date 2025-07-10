use serde_json::Value;

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{utils::try_convert_to_boolean, EvaluateResult, Evaluate, ResourceNode};

pub fn and<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::EvaluateError {
            msg: "and expects exactly two expressions".to_string(),
        });
    }

    let first = expressions[0]
        .evaluate(input)?
        .get_single_or_empty()?
        .and_then(|val| try_convert_to_boolean(&val));

    let second = expressions[1]
        .evaluate(input)?
        .get_single_or_empty()?
        .and_then(|val| try_convert_to_boolean(&val));

    let and_result = match (first, second) {
        (None, None) => None,
        (None, Some(val2)) => {
            if val2 == false {
                Some(val2)
            } else {
                None
            }
        }
        (Some(val1), None) => {
            if val1 == false {
                Some(val1)
            } else {
                None
            }
        }
        (Some(val1), Some(val2)) => Some(val1 && val2),
    };

    let output = match and_result {
        Some(val) => Value::Bool(val),
        None => Value::Array(vec![]),
    };

    Ok(ResourceNode::from_node(input, output))
}

pub fn or<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::EvaluateError {
            msg: "or expects exactly two expressions".to_string(),
        });
    }

    let first = expressions[0]
        .evaluate(input)?
        .get_single_or_empty()?
        .and_then(|val| try_convert_to_boolean(&val));

    let second = expressions[1]
        .evaluate(input)?
        .get_single_or_empty()?
        .and_then(|val| try_convert_to_boolean(&val));

    let and_result = match (first, second) {
        (None, None) => None,
        (None, Some(val2)) => {
            if val2 == true {
                Some(val2)
            } else {
                None
            }
        }
        (Some(val1), None) => {
            if val1 == true {
                Some(val1)
            } else {
                None
            }
        }
        (Some(val1), Some(val2)) => Some(val1 || val2),
    };

    let output = match and_result {
        Some(val) => Value::Bool(val),
        None => Value::Array(vec![]),
    };

    Ok(ResourceNode::from_node(input, output))
}

pub fn not<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let data = input
        .get_single_or_empty()?
        .and_then(|val| try_convert_to_boolean(&val));

    let not_result = match data {
        None => None,
        Some(val1) => Some(!val1),
    };

    let output = match not_result {
        Some(val) => Value::Bool(val),
        None => Value::Array(vec![]),
    };

    Ok(ResourceNode::from_node(input, output))
}

pub fn xor<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::EvaluateError {
            msg: "xor expects exactly two expressions".to_string(),
        });
    }

    let first = expressions[0]
        .evaluate(input)?
        .get_single_or_empty()?
        .and_then(|val| try_convert_to_boolean(&val));

    let second = expressions[1]
        .evaluate(input)?
        .get_single_or_empty()?
        .and_then(|val| try_convert_to_boolean(&val));

    let xor_result = match (first, second) {
        (None, None) => None,
        (None, Some(_)) => None,
        (Some(_), None) => None,
        (Some(val1), Some(val2)) => Some(val1 != val2),
    };

    let output = match xor_result {
        Some(val) => Value::Bool(val),
        None => Value::Array(vec![]),
    };

    Ok(ResourceNode::from_node(input, output))
}

pub fn implies<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::EvaluateError {
            msg: "xor expects exactly two expressions".to_string(),
        });
    }

    let first = expressions[0]
        .evaluate(input)?
        .get_single_or_empty()?
        .and_then(|val| try_convert_to_boolean(&val));

    let second = expressions[1]
        .evaluate(input)?
        .get_single_or_empty()?
        .and_then(|val| try_convert_to_boolean(&val));

    let implies_result = match (first, second) {
        (None, None) => None,
        (None, Some(val)) => {
            if val {
                Some(val)
            } else {
                None
            }
        }
        (Some(val), None) => {
            if val {
                None
            } else {
                Some(val)
            }
        }
        (Some(val1), Some(val2)) => {
            if val1 {
                Some(val2)
            } else {
                Some(true)
            }
        }
    };

    let output = match implies_result {
        Some(val) => Value::Bool(val),
        None => Value::Array(vec![]),
    };

    Ok(ResourceNode::from_node(input, output))
}

#[cfg(test)]
mod test {
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    use crate::evaluate::compile;
    use crate::evaluate::test::test::{run_tests, Expected, TestCase};

    #[test]
    fn evaluate_and_true_path() {
        let compiled = compile(&"Patient.a and Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 1,
            "b": 1
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_and_false_path() {
        let compiled = compile(&"Patient.a and Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": true,
            "b": 0
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([false]));
    }

    #[test]
    fn evaluate_and_empty_path() {
        let compiled = compile(&"Patient.a and Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": true,
            "b": []
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_or_true_path() {
        let compiled = compile(&"Patient.a or Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 1,
            "b": 0
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_or_false_path() {
        let compiled = compile(&"Patient.a or Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": false,
            "b": 0
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([false]));
    }

    #[test]
    fn evaluate_or_empty_path() {
        let compiled = compile(&"Patient.a or Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": false,
            "b": []
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_not_true_path() {
        let compiled = compile(&"Patient.a.not()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 0,
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_not_false_path() {
        let compiled = compile(&"Patient.a.not()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": true
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([false]));
    }

    #[test]
    fn evaluate_not_empty_path() {
        let compiled = compile(&"Patient.a.not()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": []
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_xor_true_path() {
        let compiled = compile(&"Patient.a xor Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 1,
            "b": 0
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_xor_false_path() {
        let compiled = compile(&"Patient.a xor Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": true,
            "b": true
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([false]));
    }

    #[test]
    fn evaluate_xor_empty_path() {
        let compiled = compile(&"Patient.a xor Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": false,
            "b": []
        });

        let evaluate_result = compiled.evaluate_single(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn test_implies_path() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.name.given.exists() implies Patient.name.family.exists()".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "name": [
                        {
                            "given": ["test"],
                            "family": "test"
                        }
                    ]
                }),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.name.given.exists() implies Patient.name.family.exists()".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "name": [
                        {
                            "given": ["test"]
                        }
                    ]
                }),
                expected: Expected::Value(json!([false])),
                options: None,
            },
            TestCase {
                path: "Patient.name.given.exists() implies Patient.name.family.exists()".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "name": [
                        {
                            "family": "test"
                        }
                    ]
                }),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.name.given implies Patient.name.family.exists()".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "name": [
                        {
                            "family": "test"
                        }
                    ]
                }),
                expected: Expected::Value(json!([true])),
                options: None,
            },
            TestCase {
                path: "Patient.name.given implies Patient.name.family.exists()".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "name": [
                        {
                            "use": "usual"
                        }
                    ]
                }),
                expected: Expected::Value(json!([])),
                options: None,
            },
            TestCase {
                path: "Patient.name.given implies Patient.name.family".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "name": [
                        {
                            "use": "usual"
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
