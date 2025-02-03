use serde_json::{json, Number, Value};

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{utils::get_string, CompileResult, Evaluate, ResourceNode};

struct Quantity {}

enum ArithmeticType {
    Number(Number),
    Quantity(Quantity),
}

impl TryFrom<&Value> for ArithmeticType {
    type Error = FhirpathError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(num) => Ok(ArithmeticType::Number(num.clone())),
            Value::String(string_num) => {
                let num = string_num
                    .parse::<f64>()
                    .map_err(|_| FhirpathError::CompileError {
                        msg: "String was not a number".to_string(),
                    })?;

                let num_value =
                    Number::from_f64(num).ok_or_else(|| FhirpathError::CompileError {
                        msg: "Failed to convert f64 to Number".to_string(),
                    })?;

                Ok(ArithmeticType::Number(num_value))
            }
            _ => todo!(),
        }
    }
}

fn get_numbers(num1: &Number, num2: &Number) -> CompileResult<(f64, f64)> {
    let f1 = num1.as_f64().ok_or_else(|| FhirpathError::CompileError {
        msg: "Could not convert to f64".to_string(),
    })?;
    let f2 = num2.as_f64().ok_or_else(|| FhirpathError::CompileError {
        msg: "Could not convert to f64".to_string(),
    })?;

    Ok((f1, f2))
}

impl ArithmeticType {
    pub fn mul(&self, other: &ArithmeticType) -> CompileResult<Value> {
        match (self, other) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => {
                let (f1, f2) = get_numbers(num1, num2)?;

                Ok(Value::Number(Number::from_f64(f1 * f2).ok_or_else(
                    || FhirpathError::CompileError {
                        msg: "Could not convert to f64".to_string(),
                    },
                )?))
            }
            _ => todo!(),
        }
    }

    pub fn add(&self, other: &ArithmeticType) -> CompileResult<Value> {
        match (self, other) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => {
                let (f1, f2) = get_numbers(num1, num2)?;

                Ok(Value::Number(Number::from_f64(f1 + f2).ok_or_else(
                    || FhirpathError::CompileError {
                        msg: "Could not convert to f64".to_string(),
                    },
                )?))
            }
            _ => todo!(),
        }
    }

    pub fn sub(&self, other: &ArithmeticType) -> CompileResult<Value> {
        match (self, other) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => {
                let (f1, f2) = get_numbers(num1, num2)?;

                Ok(Value::Number(Number::from_f64(f1 - f2).ok_or_else(
                    || FhirpathError::CompileError {
                        msg: "Could not convert to f64".to_string(),
                    },
                )?))
            }
            _ => todo!(),
        }
    }

    pub fn div(&self, other: &ArithmeticType) -> CompileResult<Value> {
        match (self, other) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => {
                let (f1, f2) = get_numbers(num1, num2)?;

                if f2 == 0.0 {
                    return Ok(Value::Array(vec![]));
                }

                Ok(Value::Number(
                    Number::from_f64(f1.div_euclid(f2)).ok_or_else(|| {
                        FhirpathError::CompileError {
                            msg: "Could not convert to f64".to_string(),
                        }
                    })?,
                ))
            }
            _ => todo!(),
        }
    }

    pub fn rem(&self, other: &ArithmeticType) -> CompileResult<Value> {
        match (self, other) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => {
                let (f1, f2) = get_numbers(num1, num2)?;

                if f2 == 0.0 {
                    return Ok(Value::Array(vec![]));
                }

                Ok(Value::Number(
                    Number::from_f64(f1.rem_euclid(f2)).ok_or_else(|| {
                        FhirpathError::CompileError {
                            msg: "Could not convert to f64".to_string(),
                        }
                    })?,
                ))
            }
            _ => todo!(),
        }
    }

    pub fn divide(&self, other: &ArithmeticType) -> CompileResult<Value> {
        match (self, other) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => {
                let (f1, f2) = get_numbers(num1, num2)?;

                if f2 == 0.0 {
                    return Ok(Value::Array(vec![]));
                }

                Ok(Value::Number(Number::from_f64(f1 / f2).ok_or_else(
                    || FhirpathError::CompileError {
                        msg: "Could not convert to f64".to_string(),
                    },
                )?))
            }
            _ => todo!(),
        }
    }
}

pub fn add<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::CompileError {
            msg: "add expects exactly two expressions".to_string(),
        });
    }

    let first = expressions[0].evaluate(input)?.get_single()?;
    let second = expressions[1].evaluate(input)?.get_single()?;

    if let Ok(result) = ArithmeticType::try_from(&first)?.add(&ArithmeticType::try_from(&second)?) {
        return Ok(ResourceNode::from_node(input, result));
    }

    if let (Value::String(mut first_string), Value::String(second_string)) = (first, second) {
        first_string.push_str(second_string.as_str());

        return Ok(ResourceNode::from_node(input, json!(first_string)));
    }

    Err(FhirpathError::CompileError {
        msg: "add operator not supported for types".to_string(),
    })
}

pub fn mul<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::CompileError {
            msg: "mul expects exactly two expressions".to_string(),
        });
    }

    let first = ArithmeticType::try_from(&expressions[0].evaluate(input)?.get_single()?)?;

    let second = ArithmeticType::try_from(&expressions[1].evaluate(input)?.get_single()?)?;

    Ok(ResourceNode::from_node(input, first.mul(&second)?))
}

pub fn sub<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::CompileError {
            msg: "sub expects exactly two expressions".to_string(),
        });
    }

    let first = ArithmeticType::try_from(&expressions[0].evaluate(input)?.get_single()?)?;

    let second = ArithmeticType::try_from(&expressions[1].evaluate(input)?.get_single()?)?;

    Ok(ResourceNode::from_node(input, first.sub(&second)?))
}

pub fn div<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::CompileError {
            msg: "sub expects exactly two expressions".to_string(),
        });
    }

    let first = ArithmeticType::try_from(&expressions[0].evaluate(input)?.get_single()?)?;

    let second = ArithmeticType::try_from(&expressions[1].evaluate(input)?.get_single()?)?;

    Ok(ResourceNode::from_node(input, first.div(&second)?))
}

pub fn rem<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::CompileError {
            msg: "sub expects exactly two expressions".to_string(),
        });
    }

    let first = ArithmeticType::try_from(&expressions[0].evaluate(input)?.get_single()?)?;

    let second = ArithmeticType::try_from(&expressions[1].evaluate(input)?.get_single()?)?;

    Ok(ResourceNode::from_node(input, first.rem(&second)?))
}

pub fn divide<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::CompileError {
            msg: "sub expects exactly two expressions".to_string(),
        });
    }

    let first = ArithmeticType::try_from(&expressions[0].evaluate(input)?.get_single()?)?;

    let second = ArithmeticType::try_from(&expressions[1].evaluate(input)?.get_single()?)?;

    Ok(ResourceNode::from_node(input, first.divide(&second)?))
}

pub fn amp<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() != 2 {
        return Err(FhirpathError::CompileError {
            msg: "sub expects exactly two expressions".to_string(),
        });
    }

    let first = expressions[0]
        .evaluate(input)?
        .get_single_or_empty()?
        .unwrap_or(Value::String("".to_string()));

    let second = expressions[1]
        .evaluate(input)?
        .get_single_or_empty()?
        .unwrap_or(Value::String("".to_string()));

    if let (Value::String(mut first_string), Value::String(second_string)) = (first, second) {
        first_string.push_str(second_string.as_str());

        return Ok(ResourceNode::from_node(input, json!(first_string)));
    }

    Err(FhirpathError::CompileError {
        msg: "add operator not supported for types".to_string(),
    })
}

#[cfg(test)]
mod test {
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    use crate::evaluate::compile;

    #[test]
    fn evaluate_add_path() {
        let compiled = compile(&"Patient.a + Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 2,
            "b": 6
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([8.0]));
    }

    #[test]
    fn evaluate_sub_path() {
        let compiled = compile(&"Patient.a - Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 2,
            "b": 6
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([-4.0]));
    }

    #[test]
    fn evaluate_mul_path() {
        let compiled = compile(&"Patient.a * Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 2,
            "b": 6
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([12.0]));
    }

    #[test]
    fn evaluate_div_path() {
        let compiled = compile(&"Patient.a div Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 5,
            "b": 2
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([2.0]));
    }

    #[test]
    fn evaluate_div_path_2() {
        let compiled = compile(&"Patient.a div Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 5.5,
            "b": 0.7
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([7.0]));
    }

    #[test]
    fn evaluate_mod_path() {
        let compiled = compile(&"Patient.a mod Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 5,
            "b": 2
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([1.0]));
    }

    #[test]
    fn evaluate_mod_path_2() {
        let compiled = compile(&"Patient.a mod Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 5.5,
            "b": 0.7
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        // @todo - precision
        assert_json_eq!(evaluate_result, json!([0.6000000000000003]));
    }

    #[test]
    fn evaluate_divide_path() {
        let compiled = compile(&"Patient.a / Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 5,
            "b": 2
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([2.5]));
    }

    #[test]
    fn evaluate_divide_path_2() {
        let compiled = compile(&"Patient.a / Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 5.5,
            "b": 0.7
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([7.857142857142858]));
    }

    #[test]
    fn evaluate_amp_path() {
        let compiled = compile(&"Patient.a & Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": "a",
            "b": "b"
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["ab"]));
    }

    #[test]
    fn evaluate_amp_path_2() {
        let compiled = compile(&"Patient.a & Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": "a",
            "b": []
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["a"]));
    }
}
