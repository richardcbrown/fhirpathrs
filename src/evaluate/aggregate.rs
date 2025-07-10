use serde_json::Value;

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    nodes::resource_node::{ResourceContext, ResourceNode},
    EvaluateResult, Evaluate,
};

pub fn aggregate<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let first = expressions
        .first()
        .ok_or_else(|| FhirpathError::EvaluateError {
            msg: "aggregate expects at least one expression".to_string(),
        })?;

    let second = expressions.iter().nth(1);

    let array = input.get_array()?;

    let init = match second {
        Some(sec) => sec.evaluate(input)?.get_single()?,
        None => Value::Array(vec![]),
    };

    let agg = array
        .iter()
        .enumerate()
        .try_fold(init, |acc, (index, item)| {
            let node = ResourceNode::new(
                input.data_root,
                item.to_owned(),
                input.context,
                input.path.clone(),
                input.fhir_types.clone(),
                Some(ResourceContext {
                    index: Some(index),
                    total: Some(acc),
                }),
                input.reflection_types.clone(),
            );

            first.evaluate(&node).and_then(|n| Ok(n.data))
        })?;

    Ok(ResourceNode::from_node(input, agg))
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::evaluate::test::test::{run_tests, Expected, TestCase};

    #[test]
    fn test_aggregate_path() {
        let patient = json!({ "resourceType": "Patient", "a": [1, 1, 1, 1, 1] });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.aggregate($this + $total, 0)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([5])),
                options: None,
            },
            TestCase {
                path: "Patient.a.aggregate($this + $total, 2)".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([7])),
                options: None,
            },
            TestCase {
                path: "Patient.a.aggregate(iif($total.empty(), $this, iif($this < $total, $this, $total)))".to_string(),
                input: json!({ "resourceType": "Patient", "a": [2, 3, 4, 5, 6] }),
                expected: Expected::Value(json!([2])),
                options: None,
            },
            TestCase {
              path: "Patient.a.aggregate($total + $this, 0) / Patient.a.count()".to_string(),
              input: json!({ "resourceType": "Patient", "a": [2, 3, 4, 5, 6] }),
              expected: Expected::Value(json!([4])),
              options: None,
          },
        ];

        run_tests(test_cases);
    }
}
