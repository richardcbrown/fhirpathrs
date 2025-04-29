use std::cmp::Ordering;

use serde_json::Value;

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    data_types::{arithmetic_type::ArithmeticType, utils::implicit_convert},
    CompileResult, Evaluate, ResourceNode,
};

impl ArithmeticType {
    pub fn gt(&self, other: &ArithmeticType) -> CompileResult<Value> {
        let (first, second) = implicit_convert(self, other);

        match (first, second) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => {
                Ok(Value::Bool(num1.gt(&num2)))
            }
            (ArithmeticType::DateTime(dt1), ArithmeticType::DateTime(dt2)) => {
                let cmp = dt1.partial_cmp(&dt2);

                if let Some(order) = cmp {
                    match order {
                        Ordering::Greater => return Ok(Value::Bool(true)),
                        _ => return Ok(Value::Bool(false)),
                    }
                }

                Ok(Value::Array(vec![]))
            }
            (ArithmeticType::String(s1), ArithmeticType::String(s2)) => Ok(Value::Bool(s1.gt(&s2))),
            (ArithmeticType::Time(t1), ArithmeticType::Time(t2)) => {
                let cmp = t1.partial_cmp(&t2);

                if let Some(order) = cmp {
                    match order {
                        Ordering::Greater => return Ok(Value::Bool(true)),
                        _ => return Ok(Value::Bool(false)),
                    }
                }

                Ok(Value::Array(vec![]))
            }
            (ArithmeticType::Quantity(q1), ArithmeticType::Quantity(q2)) => {
                let cmp = q1.partial_cmp(&q2);

                if let Some(order) = cmp {
                    match order {
                        Ordering::Greater => return Ok(Value::Bool(true)),
                        _ => return Ok(Value::Bool(false)),
                    }
                }

                Ok(Value::Array(vec![]))
            }
            _ => todo!(),
        }
    }

    pub fn lt(&self, other: &ArithmeticType) -> CompileResult<Value> {
        match (self, other) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => {
                Ok(Value::Bool(num1 < num2))
            }
            (ArithmeticType::DateTime(dt1), ArithmeticType::DateTime(dt2)) => {
                let cmp = dt1.partial_cmp(dt2);

                if let Some(order) = cmp {
                    match order {
                        Ordering::Less => return Ok(Value::Bool(true)),
                        _ => return Ok(Value::Bool(false)),
                    }
                }

                Ok(Value::Array(vec![]))
            }
            (ArithmeticType::Time(t1), ArithmeticType::Time(t2)) => {
                let cmp = t1.partial_cmp(&t2);

                if let Some(order) = cmp {
                    match order {
                        Ordering::Less => return Ok(Value::Bool(true)),
                        _ => return Ok(Value::Bool(false)),
                    }
                }

                Ok(Value::Array(vec![]))
            }
            (ArithmeticType::String(s1), ArithmeticType::String(s2)) => Ok(Value::Bool(s1.lt(&s2))),
            (ArithmeticType::Quantity(q1), ArithmeticType::Quantity(q2)) => {
                let cmp = q1.partial_cmp(&q2);

                if let Some(order) = cmp {
                    match order {
                        Ordering::Less => return Ok(Value::Bool(true)),
                        _ => return Ok(Value::Bool(false)),
                    }
                }

                Ok(Value::Array(vec![]))
            }
            _ => todo!(),
        }
    }

    pub fn gte(&self, other: &ArithmeticType) -> CompileResult<Value> {
        match (self, other) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => {
                Ok(Value::Bool(num1 >= num2))
            }
            (ArithmeticType::DateTime(dt1), ArithmeticType::DateTime(dt2)) => {
                let cmp = dt1.partial_cmp(dt2);

                if let Some(order) = cmp {
                    match order {
                        Ordering::Greater | Ordering::Equal => return Ok(Value::Bool(true)),
                        _ => return Ok(Value::Bool(false)),
                    }
                }

                Ok(Value::Array(vec![]))
            }
            (ArithmeticType::Quantity(t1), ArithmeticType::Quantity(t2)) => {
                let cmp = t1.partial_cmp(&t2);

                if let Some(order) = cmp {
                    match order {
                        Ordering::Greater | Ordering::Equal => return Ok(Value::Bool(true)),
                        _ => return Ok(Value::Bool(false)),
                    }
                }

                Ok(Value::Array(vec![]))
            }
            (ArithmeticType::String(s1), ArithmeticType::String(s2)) => Ok(Value::Bool(s1 >= s2)),
            (ArithmeticType::Time(t1), ArithmeticType::Time(t2)) => {
                let cmp = t1.partial_cmp(&t2);

                if let Some(order) = cmp {
                    match order {
                        Ordering::Greater | Ordering::Equal => return Ok(Value::Bool(true)),
                        _ => return Ok(Value::Bool(false)),
                    }
                }

                Ok(Value::Array(vec![]))
            }
            _ => todo!(),
        }
    }

    pub fn lte(&self, other: &ArithmeticType) -> CompileResult<Value> {
        match (self, other) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => {
                Ok(Value::Bool(num1 <= num2))
            }
            (ArithmeticType::DateTime(dt1), ArithmeticType::DateTime(dt2)) => {
                let cmp = dt1.partial_cmp(dt2);

                if let Some(order) = cmp {
                    match order {
                        Ordering::Less | Ordering::Equal => return Ok(Value::Bool(true)),
                        _ => return Ok(Value::Bool(false)),
                    }
                }

                Ok(Value::Array(vec![]))
            }
            (ArithmeticType::Quantity(t1), ArithmeticType::Quantity(t2)) => {
                let cmp = t1.partial_cmp(&t2);

                if let Some(order) = cmp {
                    match order {
                        Ordering::Less | Ordering::Equal => return Ok(Value::Bool(true)),
                        _ => return Ok(Value::Bool(false)),
                    }
                }

                Ok(Value::Array(vec![]))
            }
            (ArithmeticType::String(s1), ArithmeticType::String(s2)) => Ok(Value::Bool(s1 <= s2)),
            (ArithmeticType::Time(t1), ArithmeticType::Time(t2)) => {
                let cmp = t1.partial_cmp(&t2);

                if let Some(order) = cmp {
                    match order {
                        Ordering::Less | Ordering::Equal => return Ok(Value::Bool(true)),
                        _ => return Ok(Value::Bool(false)),
                    }
                }

                Ok(Value::Array(vec![]))
            }
            _ => todo!(),
        }
    }
}

pub fn gt<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::CompileError {
            msg: "> expects exactly two expressions".to_string(),
        });
    }

    let first = expressions[0].evaluate(input)?.get_single()?;
    let second = expressions[1].evaluate(input)?.get_single()?;

    if let Ok(result) = ArithmeticType::try_from(&first)?.gt(&ArithmeticType::try_from(&second)?) {
        return Ok(ResourceNode::from_node(input, result));
    }

    Err(FhirpathError::CompileError {
        msg: "> operator not supported for types".to_string(),
    })
}

pub fn gte<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::CompileError {
            msg: ">= expects exactly two expressions".to_string(),
        });
    }

    let first = expressions[0].evaluate(input)?.get_single()?;
    let second = expressions[1].evaluate(input)?.get_single()?;

    if let Ok(result) = ArithmeticType::try_from(&first)?.gte(&ArithmeticType::try_from(&second)?) {
        return Ok(ResourceNode::from_node(input, result));
    }

    Err(FhirpathError::CompileError {
        msg: ">= operator not supported for types".to_string(),
    })
}

pub fn lt<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::CompileError {
            msg: "< expects exactly two expressions".to_string(),
        });
    }

    let first = expressions[0].evaluate(input)?.get_single()?;
    let second = expressions[1].evaluate(input)?.get_single()?;

    if let Ok(result) = ArithmeticType::try_from(&first)?.lt(&ArithmeticType::try_from(&second)?) {
        return Ok(ResourceNode::from_node(input, result));
    }

    Err(FhirpathError::CompileError {
        msg: "< operator not supported for types".to_string(),
    })
}

pub fn lte<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::CompileError {
            msg: "< expects exactly two expressions".to_string(),
        });
    }

    let first = expressions[0].evaluate(input)?.get_single()?;
    let second = expressions[1].evaluate(input)?.get_single()?;

    if let Ok(result) = ArithmeticType::try_from(&first)?.lte(&ArithmeticType::try_from(&second)?) {
        return Ok(ResourceNode::from_node(input, result));
    }

    Err(FhirpathError::CompileError {
        msg: "< operator not supported for types".to_string(),
    })
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::evaluate::test::test::{run_tests, TestCase};

    #[test]
    fn test_evaluate_gt_path() {
        let patient = json!({
            "resourceType": "Patient",
            "birthDate": "2022-02-01",
            "name": [{ "family": "test" }]
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.birthDate > @2022-01-01".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.birthDate > @2022-03-01".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "Patient.birthDate > @2022-03-01T00:00:00".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
            TestCase {
                path: "1 > 2".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "2 > 1".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "'abc' > 'ABC'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "'ABC' > 'abc'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "3 'years' > 2 'years'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "2 'years' > 3 'years'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "2 'a' > 3 'b'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
            TestCase {
                path: "@T10:30:10 > @T09:29:50".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "@T08:30:10 > @T09:29:50".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "@T10:30:10 > @T09:29".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_evaluate_gte_path() {
        let patient = json!({
            "resourceType": "Patient",
            "birthDate": "2022-02-01",
            "name": [{ "family": "test" }]
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.birthDate >= @2022-01-01".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.birthDate >= @2022-02-01".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.birthDate >= @2022-03-01".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "Patient.birthDate >= @2022-03-01T00:00:00".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
            TestCase {
                path: "1 >= 2".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "1 >= 1".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "2 >= 1".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "'abc' >= 'ABC'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "'abc' >= 'abc'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "'ABC' >= 'abc'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "3 'years' >= 2 'years'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "3 'years' >= 3 'years'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "2 'years' >= 3 'years'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "2 'a' >= 3 'b'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
            TestCase {
                path: "@T10:30:10 >= @T09:29:50".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "@T10:30:10 >= @T10:30:10".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "@T08:30:10 >= @T09:29:50".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "@T10:30:10 >= @T09:29".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_evaluate_lt_path() {
        let patient = json!({
            "resourceType": "Patient",
            "birthDate": "2022-02-01",
            "name": [{ "family": "test" }]
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.birthDate < @2022-01-01".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "Patient.birthDate < @2022-03-01".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.birthDate < @2022-03-01T00:00:00".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
            TestCase {
                path: "1 < 2".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "2 < 1".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "'abc' < 'ABC'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "'ABC' < 'abc'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "3 'years' < 2 'years'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "2 'years' < 3 'years'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "2 'a' < 3 'b'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
            TestCase {
                path: "@T10:30:10 < @T09:29:50".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "@T08:30:10 < @T09:29:50".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "@T10:30:10 < @T09:29".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_evaluate_lte_path() {
        let patient = json!({
            "resourceType": "Patient",
            "birthDate": "2022-02-01",
            "name": [{ "family": "test" }]
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.birthDate <= @2022-01-01".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "Patient.birthDate <= @2022-02-01".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.birthDate <= @2022-03-01".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.birthDate <= @2022-03-01T00:00:00".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
            TestCase {
                path: "1 <= 2".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "1 <= 1".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "2 <= 1".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "'abc' <= 'ABC'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "'abc' <= 'abc'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "'ABC' <= 'abc'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "3 'years' <= 2 'years'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "3 'years' <= 3 'years'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "2 'years' <= 3 'years'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "2 'a' <= 3 'b'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
            TestCase {
                path: "@T10:30:10 <= @T09:29:50".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "@T10:30:10 <= @T10:30:10".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "@T08:30:10 <= @T09:29:50".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "@T10:30:10 <= @T09:29".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
        ];

        run_tests(tests);
    }
}
