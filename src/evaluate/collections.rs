use serde_json::Value;

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    equality::values_are_equal, nodes::resource_node::ResourceNode, CompileResult, Evaluate,
};

pub fn in_collection<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::CompileError {
            msg: "in expects exactly two expression".to_string(),
        });
    }

    let first = expressions
        .first()
        .ok_or_else(|| FhirpathError::CompileError {
            msg: "in expects exactly two expressions".to_string(),
        })?;

    let second = expressions
        .iter()
        .nth(1)
        .ok_or_else(|| FhirpathError::CompileError {
            msg: "in expects exactly two expressions".to_string(),
        })?;

    let left_operand_node = first.evaluate(input)?;

    if left_operand_node.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(vec![])));
    }

    let left_operand = left_operand_node.get_single()?;

    let right_operand_node = second.evaluate(input)?;

    if right_operand_node.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Bool(false)));
    }

    let right_operand = right_operand_node.get_array()?;

    dbg!(left_operand.clone());
    dbg!(right_operand.clone());

    let left_in_right = right_operand
        .iter()
        .any(|ro| values_are_equal(&left_operand, ro));

    Ok(ResourceNode::from_node(input, Value::Bool(left_in_right)))
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
        ];

        run_tests(test_cases);
    }
}
