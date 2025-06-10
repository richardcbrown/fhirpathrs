use serde_json::Value;

use crate::{
    error::FhirpathError,
    evaluate::{Evaluate, Text},
    parser::{expression::Expression, identifier::TypeSpecifier},
};

use super::{data_types::type_info::TypeInfo, nodes::resource_node::ResourceNode, CompileResult};

fn is_type<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<Option<bool>> {
    let expression = expressions
        .first()
        .ok_or_else(|| FhirpathError::CompileError {
            msg: "expected exactly 1 Expression".to_string(),
        })?;

    let type_details = expression.text()?;

    dbg!(type_details);

    let type_node = TypeSpecifier::try_from(&**expression)?.evaluate(input)?;

    let type_info: TypeInfo = serde_json::from_value(type_node.get_single()?).map_err(|err| {
        FhirpathError::CompileError {
            msg: format!("Failed to deserialize TypeInfo: {}", err.to_string()),
        }
    })?;

    if !input.is_single()? {
        return Err(FhirpathError::CompileError {
            msg: "expected expects a single input".to_string(),
        });
    }

    let input_types = input.get_type_info();

    let input_type = input_types.first();

    Ok(input_type
        .and_then(|it| it.to_owned())
        .and_then(|type_details| Some(type_details.eq(&type_info))))
}

pub fn is<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let type_match = is_type(input, expressions)?;

    let result = type_match
        .and_then(|tm| Some(Value::Bool(tm)))
        .unwrap_or(Value::Array(vec![]));

    Ok(ResourceNode::from_node(input, result))
}

pub fn as_fn<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a, 'b>> {
    let type_match = is_type(input, expressions)?;

    let result: Value = match type_match {
        Some(matched) => {
            if matched {
                input.get_single()?
            } else {
                Value::Array(vec![])
            }
        }
        None => Value::Array(vec![]),
    };

    Ok(ResourceNode::from_node(input, result))
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::{
        evaluate::{
            test::test::{run_tests, TestCase},
            EvaluateOptions,
        },
        models::{get_model_details, ModelType},
    };

    #[test]
    fn test_is_fn_path() {
        let observation = json!({
            "resourceType": "Observation",
            "valueQuantity": {
                "value": 1,
                "unit": "s"
            },
            "component": [
                {
                    "valueQuantity": {
                        "value": 1,
                        "unit": "s"
                    }
                },
                {
                    "valueString": "abc"
                }
            ]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Observation.value.is(FHIR.Quantity)".to_string(),
                input: observation.clone(),
                expected: json!([true]),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
            TestCase {
                path: "Observation.value.is(Quantity)".to_string(),
                input: observation.clone(),
                expected: json!([true]),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
            TestCase {
                path: "Observation.component[0].value.is(FHIR.Quantity)".to_string(),
                input: observation.clone(),
                expected: json!([true]),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
            TestCase {
                path: "Observation.component[1].value.is(System.String)".to_string(),
                input: observation.clone(),
                expected: json!([true]),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_is_path() {
        let observation = json!({
            "resourceType": "Observation",
            "valueQuantity": {
                "value": 1,
                "unit": "s"
            },
            "component": [
                {
                    "valueQuantity": {
                        "value": 1,
                        "unit": "s"
                    }
                },
                {
                    "valueString": "abc"
                }
            ]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Observation.value is FHIR.Quantity".to_string(),
                input: observation.clone(),
                expected: json!([true]),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
            TestCase {
                path: "Observation.value is Quantity".to_string(),
                input: observation.clone(),
                expected: json!([true]),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
            TestCase {
                path: "Observation.component[0].value is FHIR.Quantity".to_string(),
                input: observation.clone(),
                expected: json!([true]),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
            TestCase {
                path: "Observation.component[1].value is System.String".to_string(),
                input: observation.clone(),
                expected: json!([true]),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_as_fn_path() {
        let observation = json!({
            "resourceType": "Observation",
            "valueQuantity": {
                "value": 1,
                "unit": "s"
            },
            "component": [
                {
                    "valueQuantity": {
                        "value": 1,
                        "unit": "s"
                    }
                },
                {
                    "valueString": "abc"
                }
            ]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Observation.value.as(FHIR.Quantity)".to_string(),
                input: observation.clone(),
                expected: json!([{
                    "value": 1,
                    "unit": "s"
                }]),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
            TestCase {
                path: "Observation.value.as(Quantity)".to_string(),
                input: observation.clone(),
                expected: json!([{
                    "value": 1,
                    "unit": "s"
                }]),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
            TestCase {
                path: "Observation.component[0].value.as(FHIR.Quantity)".to_string(),
                input: observation.clone(),
                expected: json!([{
                    "value": 1,
                    "unit": "s"
                }]),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
            TestCase {
                path: "Observation.component[1].value.as(System.String)".to_string(),
                input: observation.clone(),
                expected: json!(["abc"]),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
            TestCase {
                path: "Observation.component[1].value.as(FHIR.Quantity)".to_string(),
                input: observation.clone(),
                expected: json!([]),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_as_path() {
        let observation = json!({
            "resourceType": "Observation",
            "valueQuantity": {
                "value": 1,
                "unit": "s"
            },
            "component": [
                {
                    "valueQuantity": {
                        "value": 1,
                        "unit": "s"
                    }
                },
                {
                    "valueString": "abc"
                }
            ]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Observation.value as FHIR.Quantity".to_string(),
                input: observation.clone(),
                expected: json!([{
                    "value": 1,
                    "unit": "s"
                }]),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
            TestCase {
                path: "Observation.value as Quantity".to_string(),
                input: observation.clone(),
                expected: json!([{
                    "value": 1,
                    "unit": "s"
                }]),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
            TestCase {
                path: "Observation.component[0].value as FHIR.Quantity".to_string(),
                input: observation.clone(),
                expected: json!([{
                    "value": 1,
                    "unit": "s"
                }]),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
            TestCase {
                path: "Observation.component[1].value as System.String".to_string(),
                input: observation.clone(),
                expected: json!(["abc"]),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
            TestCase {
                path: "Observation.component[1].value as FHIR.Quantity".to_string(),
                input: observation.clone(),
                expected: json!([]),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
        ];

        run_tests(test_cases);
    }
}
