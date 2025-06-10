use serde_json::Value;

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    equality::values_are_equal, nodes::resource_node::ResourceNode, EvaluateResult, Evaluate,
};

fn collection_contains_item<'a>(
    input: &'a ResourceNode<'a>,
    collection_exp: &Expression,
    item_expr: &Expression,
) -> EvaluateResult<Value> {
    let item_node = item_expr.evaluate(input)?;

    if item_node.is_empty()? {
        return Ok(Value::Array(vec![]));
    }

    let item = item_node.get_single()?;

    let collection_node = collection_exp.evaluate(input)?;

    if collection_node.is_empty()? {
        return Ok(Value::Bool(false));
    }

    let collection = collection_node.get_array()?;

    dbg!(item.clone());
    dbg!(collection.clone());

    let item_in_collection = collection.iter().any(|ro| values_are_equal(&item, ro));

    Ok(Value::Bool(item_in_collection))
}

pub fn in_collection<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::EvaluateError {
            msg: "in expects exactly two expression".to_string(),
        });
    }

    let first = expressions
        .first()
        .ok_or_else(|| FhirpathError::EvaluateError {
            msg: "in expects exactly two expressions".to_string(),
        })?;

    let second = expressions
        .iter()
        .nth(1)
        .ok_or_else(|| FhirpathError::EvaluateError {
            msg: "in expects exactly two expressions".to_string(),
        })?;

    Ok(ResourceNode::from_node(
        input,
        collection_contains_item(input, second, first)?,
    ))
}

pub fn collection_contains<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::EvaluateError {
            msg: "in expects exactly two expression".to_string(),
        });
    }

    let first = expressions
        .first()
        .ok_or_else(|| FhirpathError::EvaluateError {
            msg: "in expects exactly two expressions".to_string(),
        })?;

    let second = expressions
        .iter()
        .nth(1)
        .ok_or_else(|| FhirpathError::EvaluateError {
            msg: "in expects exactly two expressions".to_string(),
        })?;

    Ok(ResourceNode::from_node(
        input,
        collection_contains_item(input, first, second)?,
    ))
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::evaluate::test::test::{run_tests, TestCase};

    #[test]
    fn test_in_path() {
        let patient = json!({
            "resourceType": "Patient",
            "name": [{
                "use": "usual",
                "given": ["test1", "test2"]
            }]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "'test1' in Patient.name.given".to_string(),
                input: patient.clone(),
                expected: json!([true]),
                options: None,
            },
            TestCase {
                path: "'test3' in Patient.name.given".to_string(),
                input: patient.clone(),
                expected: json!([false]),
                options: None,
            },
            TestCase {
                path: "Patient.a in Patient.name.given".to_string(),
                input: patient.clone(),
                expected: json!([]),
                options: None,
            },
            TestCase {
                path: "'test1' in Patient.a".to_string(),
                input: patient.clone(),
                expected: json!([false]),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn test_contains_path() {
        let patient = json!({
            "resourceType": "Patient",
            "name": [{
                "use": "usual",
                "given": ["test1", "test2"]
            }]
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.name.given contains 'test1'".to_string(),
                input: patient.clone(),
                expected: json!([true]),
                options: None,
            },
            TestCase {
                path: "Patient.name.given contains 'test3'".to_string(),
                input: patient.clone(),
                expected: json!([false]),
                options: None,
            },
            TestCase {
                path: "Patient.name.given contains Patient.a".to_string(),
                input: patient.clone(),
                expected: json!([]),
                options: None,
            },
            TestCase {
                path: "Patient.a contains 'test1'".to_string(),
                input: patient.clone(),
                expected: json!([false]),
                options: None,
            },
        ];

        run_tests(test_cases);
    }
}
