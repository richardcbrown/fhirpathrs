use serde_json::{json, Number, Value};

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    types::ArithmeticType,
    utils::{get_f64_from_expression, get_usize_from_expression},
    CompileResult, Evaluate, ResourceNode,
};

fn get_number(num: &Number) -> CompileResult<f64> {
    num.as_f64().ok_or_else(|| FhirpathError::CompileError {
        msg: "Could not convert to f64".to_string(),
    })
}

fn get_numbers(num1: &Number, num2: &Number) -> CompileResult<(f64, f64)> {
    let f1 = get_number(num1)?;
    let f2 = get_number(num2)?;

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

    pub fn exp(&self) -> CompileResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                let f1 = get_number(num)?;

                Ok(Value::Number(Number::from_f64(f1.exp()).ok_or_else(
                    || FhirpathError::CompileError {
                        msg: "Could not convert to f64".to_string(),
                    },
                )?))
            }
            _ => todo!(),
        }
    }

    pub fn abs(&self) -> CompileResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                let f1 = get_number(num)?;

                Ok(Value::Number(Number::from_f64(f1.abs()).ok_or_else(
                    || FhirpathError::CompileError {
                        msg: "Could not convert to f64".to_string(),
                    },
                )?))
            }
            _ => todo!(),
        }
    }

    pub fn ceiling(&self) -> CompileResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                let f1 = get_number(num)?;

                Ok(Value::Number(
                    Number::from_i128(f1.ceil() as i128).ok_or_else(|| {
                        FhirpathError::CompileError {
                            msg: "Could not convert to f64".to_string(),
                        }
                    })?,
                ))
            }
            _ => todo!(),
        }
    }

    pub fn floor(&self) -> CompileResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                let f1 = get_number(num)?;

                Ok(Value::Number(
                    Number::from_i128(f1.floor() as i128).ok_or_else(|| {
                        FhirpathError::CompileError {
                            msg: "Could not convert to f64".to_string(),
                        }
                    })?,
                ))
            }
            _ => todo!(),
        }
    }

    pub fn ln(&self) -> CompileResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                let f1 = get_number(num)?;

                Ok(Value::Number(Number::from_f64(f1.ln()).ok_or_else(
                    || FhirpathError::CompileError {
                        msg: "Could not convert from f64".to_string(),
                    },
                )?))
            }
            _ => todo!(),
        }
    }

    pub fn log(&self, base: f64) -> CompileResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                let f1 = get_number(num)?;

                Ok(Value::Number(Number::from_f64(f1.log(base)).ok_or_else(
                    || FhirpathError::CompileError {
                        msg: "Could not convert from f64".to_string(),
                    },
                )?))
            }
            _ => todo!(),
        }
    }

    pub fn power(&self, exponent: f64) -> CompileResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                let f1 = get_number(num)?;

                // @todo - Integer representation

                Ok(Value::Number(
                    Number::from_f64(f1.powf(exponent)).ok_or_else(|| {
                        FhirpathError::CompileError {
                            msg: "Could not convert from f64".to_string(),
                        }
                    })?,
                ))
            }
            _ => todo!(),
        }
    }

    pub fn round(&self, precision: usize) -> CompileResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                let f1 = get_number(num)?;

                let rounded = match precision {
                    0 => Ok(Number::from_i128(f1.round() as i128).ok_or_else(|| {
                        FhirpathError::CompileError {
                            msg: "Could not convert from f64".to_string(),
                        }
                    })?),
                    _ => {
                        let round_string: String = format!("{:.1$}", f1, precision);

                        let num = Number::from_f64(round_string.parse::<f64>().map_err(|_| {
                            FhirpathError::CompileError {
                                msg: format!("Could not parse {} as f64", round_string),
                            }
                        })?)
                        .ok_or_else(|| FhirpathError::CompileError {
                            msg: "Could not convert from f64".to_string(),
                        })?;

                        Ok(num)
                    }
                }?;

                Ok(Value::Number(rounded))
            }
            _ => todo!(),
        }
    }

    pub fn sqrt(&self) -> CompileResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                let f1 = get_number(num)?;

                let root = f1.sqrt();

                if root.is_nan() {
                    return Ok(Value::Array(vec![]));
                }

                Ok(Value::Number(Number::from_f64(root).ok_or_else(|| {
                    FhirpathError::CompileError {
                        msg: "Could not convert from f64".to_string(),
                    }
                })?))
            }
            _ => todo!(),
        }
    }

    pub fn truncate(&self) -> CompileResult<Value> {
        match self {
            ArithmeticType::Number(num) => {
                let f1 = get_number(num)?;

                Ok(Value::Number(
                    Number::from_i128(f1.trunc() as i128).ok_or_else(|| {
                        FhirpathError::CompileError {
                            msg: "Could not convert from f64".to_string(),
                        }
                    })?,
                ))
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

pub fn abs<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.abs(),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

pub fn ceiling<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.ceiling(),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

pub fn exp<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.exp(),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

pub fn floor<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.floor(),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

pub fn ln<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.ln(),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

pub fn log<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() != 1 {
        return Err(FhirpathError::CompileError {
            msg: "log expects exactly one expression".to_string(),
        });
    }

    let first = get_f64_from_expression(input, &expressions[0])?;

    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.log(first),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

pub fn power<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() != 1 {
        return Err(FhirpathError::CompileError {
            msg: "log expects exactly one expression".to_string(),
        });
    }

    let first = get_f64_from_expression(input, &expressions[0])?;

    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.power(first),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

pub fn round<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() > 1 {
        return Err(FhirpathError::CompileError {
            msg: "log expects zero or one expression".to_string(),
        });
    }

    let first = match expressions.first() {
        Some(exp) => get_usize_from_expression(input, &exp),
        None => Ok(0),
    }?;

    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.round(first),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

pub fn sqrt<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.sqrt(),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
}

pub fn truncate<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let data = &input.get_single_or_empty()?;

    let value = match data {
        Some(val) => ArithmeticType::try_from(val)?.truncate(),
        None => Ok(Value::Array(vec![])),
    }?;

    Ok(ResourceNode::from_node(input, value))
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

    #[test]
    fn evaluate_abs_path() {
        let compiled = compile(&"Patient.a.abs()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": -1.1
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([1.1]));
    }

    #[test]
    fn evaluate_abs_empty_path() {
        let compiled = compile(&"Patient.a.abs()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": []
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_ceiling_path() {
        let compiled = compile(&"Patient.a.ceiling()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 1.1
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([2]));
    }

    #[test]
    fn evaluate_ceiling_empty_path() {
        let compiled = compile(&"Patient.a.ceiling()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": []
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_exp_path() {
        let compiled = compile(&"Patient.a.exp()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 2
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([7.38905609893065]));
    }

    #[test]
    fn evaluate_exp_empty_path() {
        let compiled = compile(&"Patient.a.exp()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": []
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_floor_path() {
        let compiled = compile(&"Patient.a.floor()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 1.1
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([1]));
    }

    #[test]
    fn evaluate_floor_empty_path() {
        let compiled = compile(&"Patient.a.floor()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": []
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_ln_path() {
        let compiled = compile(&"Patient.a.ln()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 1.1
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([0.09531017980432493]));
    }

    #[test]
    fn evaluate_ln_empty_path() {
        let compiled = compile(&"Patient.a.ln()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": []
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_log_path() {
        let compiled = compile(&"Patient.a.log(2.0)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 1.1
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([0.13750352374993502]));
    }

    #[test]
    fn evaluate_log_empty_path() {
        let compiled = compile(&"Patient.a.log(2.0)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": []
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_power_path() {
        let compiled = compile(&"Patient.a.power(2.0)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 2
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([4.0]));
    }

    #[test]
    fn evaluate_power_empty_path() {
        let compiled = compile(&"Patient.a.power(2.0)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": []
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_round_path() {
        let compiled = compile(&"Patient.a.round(2)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 2.111111
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([2.11]));
    }

    #[test]
    fn evaluate_round_default_path() {
        let compiled = compile(&"Patient.a.round()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 2.111111
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([2]));
    }

    #[test]
    fn evaluate_round_empty_path() {
        let compiled = compile(&"Patient.a.round(2)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": []
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_sqrt_path() {
        let compiled = compile(&"Patient.a.sqrt()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 4
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([2.0]));
    }

    #[test]
    fn evaluate_sqrt_empty_path() {
        let compiled = compile(&"Patient.a.sqrt()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": []
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_truncate_path() {
        let compiled = compile(&"Patient.a.truncate()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": 4.53
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([4]));
    }

    #[test]
    fn evaluate_truncate_empty_path() {
        let compiled = compile(&"Patient.a.truncate()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": []
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }
}
