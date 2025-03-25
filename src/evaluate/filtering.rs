use serde_json::{json, Value};

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    equality::values_are_equal,
    types::type_info::TypeInfo,
    utils::{get_array_from_expression, unique_array_elements},
    CompileResult, Evaluate, ResourceNode,
};

fn evaluate_filter_expression(
    input: ResourceNode,
    array: &Vec<Value>,
    expr: &Expression,
) -> Vec<Value> {
    let results: Vec<Value> = array
        .iter()
        .filter_map(|item| {
            let node = ResourceNode::new(
                input.data_root,
                None,
                item.to_owned(),
                input.context,
                input.path.clone(),
            );

            expr.evaluate(&node)
                .ok()
                .and_then(|result| result.get_single().ok())
                .and_then(|value| {
                    if let Value::Bool(bool) = value {
                        if bool {
                            return Some(item.to_owned());
                        }
                    }

                    None
                })
        })
        .collect();

    results
}

pub fn where_function<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    expressions
        .first()
        .ok_or(FhirpathError::CompileError {
            msg: "Missing first expression".to_string(),
        })
        .and_then(|expr| {
            let data = input
                .get_array()
                .and_then(|val| Ok(evaluate_filter_expression(input.clone(), val, expr)))?;

            Ok(ResourceNode::from_node(input, json!(data)))
        })
}

pub fn select<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let expression = expressions.first().ok_or(FhirpathError::CompileError {
        msg: "select function requires single expression argument".to_string(),
    })?;

    let value = input.get_array()?;

    let result = value
        .iter()
        .map(|val| {
            let node = ResourceNode::new(
                input.data_root,
                None,
                json!(val.clone()),
                input.context,
                input.path.clone(),
            );

            let result = expression.evaluate(&node)?.data;

            Ok(result)
        })
        .collect::<CompileResult<Vec<Value>>>()?;

    let combined: Vec<Value> = result
        .iter()
        .map(|item| match item {
            Value::Array(array) => Ok(array.to_owned()),
            _ => Err(FhirpathError::CompileError {
                msg: "Result was not an array".to_string(),
            }),
        })
        .collect::<CompileResult<Vec<Vec<Value>>>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(ResourceNode::from_node(input, json!(combined)))
}

fn repeat_expr<'a>(
    input: &'a ResourceNode<'a>,
    mut values: Vec<Value>,
    expression: &Expression,
) -> CompileResult<Vec<Value>> {
    let mut items: Vec<Value> = values.iter().try_fold(vec![], |mut acc, val| {
        let node = ResourceNode::from_node(input, val.clone());

        let node_results = unique_array_elements(&get_array_from_expression(&node, &expression)?);

        // unique results not currently in values
        let mut unique_results: Vec<Value> = node_results
            .into_iter()
            .filter(|res| !acc.iter().any(|accu| values_are_equal(res, accu)))
            .collect();

        acc.append(&mut unique_results);

        let mut repeated_results = repeat_expr(input, unique_results, expression)?;

        acc.append(&mut repeated_results);

        Ok(acc)
    })?;

    values.append(&mut items);

    Ok(values)
}

pub fn repeat<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let expression = expressions
        .first()
        .ok_or_else(|| FhirpathError::CompileError {
            msg: "repeat expects exactly 1 Expression".to_string(),
        })?;

    let initial_items = unique_array_elements(&get_array_from_expression(input, expression)?);

    let accumulated = repeat_expr(input, initial_items, expression)?;

    Ok(ResourceNode::from_node(input, Value::Array(accumulated)))
}

pub fn of_type<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let expression = expressions
        .first()
        .ok_or_else(|| FhirpathError::CompileError {
            msg: "ofType expects exactly 1 Expression".to_string(),
        })?;

    let type_details = expression.evaluate(input)?;

    dbg!(type_details.data.clone());

    let type_info: TypeInfo =
        serde_json::from_value(type_details.get_single()?).map_err(|err| {
            FhirpathError::CompileError {
                msg: format!("Failed to deserialize TypeInfo: {}", err.to_string()),
            }
        })?;

    let array = input.get_array()?;

    let type_array = array
        .to_owned()
        .into_iter()
        .filter_map(|item| {
            let item_node = ResourceNode::from_node(input, item.clone());

            let item_type = item_node.get_type_info()?;

            match type_info.eq(&item_type) {
                true => Some(item),
                false => None,
            }
        })
        .collect();

    Ok(ResourceNode::from_node(input, Value::Array(type_array)))
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::{
        evaluate::{
            test::test::{run_tests, TestCase},
            EvaluateOptions,
        },
        models::{get_model_details, ModelDetails, ModelType},
    };

    #[test]
    fn test_repeat_path() {
        let patient = json!({
            "resourceType": "Patient",
            "value": {
                "value": {
                    "item": 'a'
                }
            }
        });

        let test_cases: Vec<TestCase> = vec![TestCase {
            path: "Patient.repeat(value)".to_string(),
            input: patient.clone(),
            expected: json!([
                {
                    "value": {
                        "item": 'a'
                    }
                },
                {
                    "item": 'a'
                }
            ]),
            options: None,
        }];

        run_tests(test_cases);
    }

    #[test]
    fn test_of_type_path() {
        let observation = json!({
            "resourceType": "Observation",
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

        let test_cases: Vec<TestCase> = vec![TestCase {
            path: "Observation.component.value.ofType(Quantity)".to_string(),
            input: observation.clone(),
            expected: json!([{
                "value": 1,
                "unit": "s"
            }]),
            options: Some(EvaluateOptions {
                model: Some(get_model_details(ModelType::Stu3).unwrap()),
                vars: None,
            }),
        }];

        run_tests(test_cases);
    }
}
