use std::cmp::Ordering;

use serde_json::Value;

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{types::ArithmeticType, utils::get_numbers, CompileResult, Evaluate, ResourceNode};

impl ArithmeticType {
    pub fn gt(&self, other: &ArithmeticType) -> CompileResult<Value> {
        match (self, other) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => {
                let (f1, f2) = get_numbers(num1, num2)?;

                Ok(Value::Bool(f1 > f2))
            }
            (ArithmeticType::DateTime(dt1), ArithmeticType::DateTime(dt2)) => {
                let cmp = dt1.partial_cmp(dt2);

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
                let (f1, f2) = get_numbers(num1, num2)?;

                Ok(Value::Bool(f1 < f2))
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
            _ => todo!(),
        }
    }

    pub fn gte(&self, other: &ArithmeticType) -> CompileResult<Value> {
        match (self, other) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => {
                let (f1, f2) = get_numbers(num1, num2)?;

                Ok(Value::Bool(f1 >= f2))
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

#[cfg(test)]
mod test {
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    use crate::evaluate::compile;

    #[test]
    fn test_evaluate_gt_date_true_path() {
        let compiled = compile(&"Patient.birthDate > @2022-01-01".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "birthDate": "2022-02-01"
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        println!("{:?}", evaluate_result);

        assert_json_eq!(evaluate_result, json!([true]));
    }
}
