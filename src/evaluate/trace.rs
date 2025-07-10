use serde_json::Value;
use crate::error::FhirpathError;
use crate::evaluate::{Evaluate, EvaluateResult};
use crate::evaluate::nodes::resource_node::{FhirContext, ResourceNode};
use crate::evaluate::utils::get_string;
use crate::parser::expression::Expression;

const DEFAULT_TRACE: fn(String, Value) = |name: String, output_result: Value|  println!("TRACE:[{}] {}", name.clone(), output_result.clone());

fn call_trace(ctx: &FhirContext, name: String, value: Value) -> EvaluateResult<()> {
    match ctx.trace_function {
        Some(ref f) => {
            let mut func = f.lock().map_err(|e| {
                FhirpathError::EvaluateError {
                    msg: format!("Could not obtain lock on trace function: {}", e),
                }
            })?;

            func(name.clone(), value);
        },
        None => DEFAULT_TRACE(name.clone(), value),
    };
    
    Ok(())
}

pub fn trace<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let name_expr = expressions.first().ok_or(FhirpathError::EvaluateError {
        msg: "Trace expects at least one expression".to_string()
    })?;

    let name = get_string(&name_expr.evaluate(input)?.get_single()?)?;

    let projection_expr = expressions.iter().nth(1);

    let output_result = match projection_expr {
        None => input.data.clone(),
        Some(exp) => exp.evaluate(input)?.data,
    };

    call_trace(input.context, name.clone(), output_result.clone());
    
    Ok(ResourceNode::from_node(input, input.data.clone()))
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use serde_json::{json, Value};
    use crate::evaluate::EvaluateOptions;
    use crate::evaluate::test::test::{run_tests, Expected, TestCase};

    #[test]
    fn test_trace_path() {
        let patient = json!({ "resourceType": "Patient", "a": [{ "b": true }] });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.trace('test', b).b.anyTrue()".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([true])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_custom_trace_path() {
        let patient = json!({ "resourceType": "Patient", "a": [{ "b": true }] });

        let mut map: HashMap<String, Value> = HashMap::new();

        {
            let mut trace_function = |name: String, trace_details: Value| {
                map.insert(name, trace_details);
            };

            let test_cases: Vec<TestCase> = vec![
                TestCase {
                    path: "Patient.a.trace('test', b).b.anyTrue()".to_string(),
                    input: patient.clone(),
                    expected: Expected::Value(json!([true])),
                    options: Some(EvaluateOptions {
                        model: None,
                        vars: None,
                        now: None,
                        trace_function: Some(Arc::new(Mutex::new(&mut trace_function))),
                    }),
                },
            ];

            run_tests(test_cases);
        }

        assert_eq!(map.get("test").unwrap().to_owned(),  json!([true]));
    }
}