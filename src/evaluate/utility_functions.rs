use serde_json::Value;

use crate::parser::expression::Expression;

use super::{nodes::resource_node::ResourceNode, CompileResult};

pub fn now<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    Ok(ResourceNode::from_node(
        input,
        Value::String(
            input
                .context
                .now
                .format("@%Y-%m-%dT%H:%M:%S.%3f%:z")
                .to_string(),
        ),
    ))
}

pub fn today<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    Ok(ResourceNode::from_node(
        input,
        Value::String(input.context.now.format("@%Y-%m-%d").to_string()),
    ))
}

pub fn time_of_day<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    Ok(ResourceNode::from_node(
        input,
        Value::String(input.context.now.format("@%H:%M:%S.%3f").to_string()),
    ))
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use chrono::DateTime;
    use serde_json::json;

    use crate::evaluate::{
        test::test::{run_tests, TestCase},
        EvaluateOptions,
    };

    #[test]
    fn evaluate_now_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": 2,
            "b": 6
        });

        let test_datetime = DateTime::from_str("2025-04-13T23:00:00.000+00:00").unwrap();

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "now() = now()".to_string(),
                input: patient.clone(),
                expected: json!([true]),
                options: None,
            },
            TestCase {
                path: "now()".to_string(),
                input: patient.clone(),
                expected: json!(["@2025-04-13T23:00:00.000+00:00"]),
                options: Some(EvaluateOptions {
                    now: Some(test_datetime),
                    model: None,
                    vars: None,
                }),
            },
            TestCase {
                path: "now() = @2025-04-13T23:00:00.000+00:00".to_string(),
                input: patient.clone(),
                expected: json!([true]),
                options: Some(EvaluateOptions {
                    now: Some(test_datetime),
                    model: None,
                    vars: None,
                }),
            },
        ];

        run_tests(test_cases);
    }
}
