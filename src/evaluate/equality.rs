use std::cmp::Ordering;

use serde_json::Value;

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    types::{arithmetic_type::ArithmeticType, utils::implicit_convert},
    CompileResult, Evaluate, ResourceNode,
};

impl ArithmeticType {
    fn eq(&self, other: &Self) -> Value {
        let (first, second) = implicit_convert(self, other);

        match (first, second) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => {
                Value::Bool(num1.eq(&num2))
            }
            (ArithmeticType::DateTime(dt1), ArithmeticType::DateTime(dt2)) => {
                let cmp = dt1.partial_cmp(&dt2);

                match cmp {
                    Some(order) => match order {
                        Ordering::Equal => Value::Bool(true),
                        _ => Value::Bool(false),
                    },
                    None => Value::Array(vec![]),
                }
            }
            (ArithmeticType::String(s1), ArithmeticType::String(s2)) => Value::Bool(s1.eq(&s2)),
            (ArithmeticType::Time(t1), ArithmeticType::Time(t2)) => {
                let cmp = t1.partial_cmp(&t2);

                match cmp {
                    Some(order) => match order {
                        Ordering::Equal => Value::Bool(true),
                        _ => Value::Bool(false),
                    },
                    None => Value::Array(vec![]),
                }
            }
            (ArithmeticType::Quantity(q1), ArithmeticType::Quantity(q2)) => {
                let cmp = q1.partial_cmp(&q2);

                match cmp {
                    Some(order) => match order {
                        Ordering::Equal => Value::Bool(true),
                        _ => Value::Bool(false),
                    },
                    None => Value::Array(vec![]),
                }
            }
            _ => Value::Bool(false),
        }
    }
}

// todo - pad out
pub fn values_are_equal(first: &Value, second: &Value) -> bool {
    first == second
}

fn are_equal(input: &ResourceNode, expressions: &Vec<Box<Expression>>) -> CompileResult<Value> {
    if expressions.len() != 2 {
        return Err(FhirpathError::CompileError {
            msg: "Equal function takes exactly 2 expressions".to_string(),
        });
    }

    let first = &expressions[0].evaluate(input)?;
    let second = &expressions[1].evaluate(input)?;

    let first_val = first.get_array()?;
    let second_val = second.get_array()?;

    if first_val.is_empty() || second_val.is_empty() {
        return Ok(Value::Array(vec![]));
    }

    if first_val.len() != second_val.len() {
        return Ok(Value::Bool(false));
    }

    for (index, first_item) in first_val.iter().enumerate() {
        let result = second_val
            .iter()
            .nth(index)
            .and_then(|second_item| {
                if let (Ok(t1), Ok(t2)) = (
                    ArithmeticType::try_from(first_item),
                    ArithmeticType::try_from(second_item),
                ) {
                    return Some(t1.eq(&t2));
                }

                Some(Value::Bool(values_are_equal(first_item, second_item)))
            })
            .unwrap_or(Value::Bool(false));

        match result {
            Value::Bool(bool_val) => {
                if !bool_val {
                    return Ok(result);
                }
            }
            _ => return Ok(result),
        }
    }

    Ok(Value::Bool(true))
}

pub fn equal<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let result = are_equal(input, expressions)?;

    Ok(ResourceNode::from_node(input, result))
}

pub fn not_equal<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let result = are_equal(input, expressions)?;

    let inverse = match result {
        Value::Bool(val) => Ok(Value::Bool(!val)),
        _ => Err(FhirpathError::CompileError {
            msg: "Invalid Boolean value".to_string(),
        }),
    }?;

    Ok(ResourceNode::from_node(input, inverse))
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::evaluate::test::test::{run_tests, TestCase};

    #[test]
    fn test_evaluate_eq_path() {
        let patient = json!({
            "resourceType": "Patient",
            "birthDate": "2022-01-01",
            "name": [{ "family": "test", "given": ["test"] }, { "family": "test", "given": ["test"]  }]
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.birthDate = @2022-01-01".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.birthDate = @2022-03".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
            TestCase {
                path: "Patient.birthDate = @2022-03-01T00:00:00".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
            TestCase {
                path: "1 = 2".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "1 = 1".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "1.0 = 1".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "1.1 = 1".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "1.100 = 1.1".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "'abc' = 'ABC'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "'abc' = 'abc'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "2 'years' = 2 'years'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "2 'years' = 3 'years'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "2 'a' = 3 'b'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
            TestCase {
                path: "@2012 = @2012".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "@2012 = @2013".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "@2012-01 = @2012".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
            TestCase {
                path: "@2012-01-01T10:30 = @2012-01-01T10:30".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "@2012-01-01T10:30 = @2012-01-01T10:31".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "@2012-01-01T10:30:31 = @2012-01-01T10:30".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
            TestCase {
                path: "@2012-01-01T10:30:31.0 = @2012-01-01T10:30:31".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "@2012-01-01T10:30:31.1 = @2012-01-01T10:30:31".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "@2017-11-05T01:30:00.0-04:00 = @2017-11-05T01:15:00.0-05:00".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "@2017-11-05T01:30:00.0-04:00 = @2017-11-05T00:30:00.0-05:00".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.name[0] = Patient.name[1]".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
        ];

        run_tests(tests);
    }
}
