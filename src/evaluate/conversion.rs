use serde_json::{json, Number, Value};

use crate::{error::FhirpathError, parser::expression::Expression};

use super::{
    data_types::{date::Date, date_time::DateTime, quantity::Quantity, time::Time},
    utils::{
        get_array_from_expression, get_boolean_from_expression, try_convert_to_boolean,
        try_convert_to_decimal,
    },
    CompileResult, ResourceNode,
};

pub fn iif<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if expressions.len() < 2 || expressions.len() > 3 {
        return Err(FhirpathError::CompileError {
            msg: "iif expects 2-3 Expressions".to_string(),
        });
    }

    let mut exp_iter = expressions.iter();

    let first = exp_iter.next().ok_or_else(|| FhirpathError::CompileError {
        msg: "Missing iif condition".to_string(),
    })?;

    let second = exp_iter.next().ok_or_else(|| FhirpathError::CompileError {
        msg: "Missing iif true result".to_string(),
    })?;

    let third = exp_iter.next();

    let condition = get_boolean_from_expression(input, first)?;

    let output = match condition {
        true => get_array_from_expression(input, second)?,
        false => match third {
            Some(exp) => get_array_from_expression(input, exp)?,
            None => vec![],
        },
    };

    Ok(ResourceNode::from_node(input, json!(output)))
}

pub fn to_boolean<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let result = input.get_single()?;

    let bool_result: Vec<Value> = match try_convert_to_boolean(&result) {
        Some(val) => vec![Value::Bool(val)],
        None => vec![],
    };

    Ok(ResourceNode::from_node(input, json!(bool_result)))
}

pub fn converts_to_boolean<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let result = input.get_single()?;

    let converts_bool: bool = match try_convert_to_boolean(&result) {
        Some(_val) => true,
        None => false,
    };

    Ok(ResourceNode::from_node(input, json!(converts_bool)))
}

pub fn to_integer<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(vec![])));
    }

    let single = input.get_single()?;

    let dec_result = try_convert_to_decimal(&single);

    let value = dec_result
        .and_then(|dr| {
            if dr.is_integer() {
                return Some(dr);
            }

            return None;
        })
        .and_then(|dr| Some(Value::Number(Number::from_i128(dr.try_into().ok()?)?)))
        .unwrap_or(Value::Array(vec![]));

    Ok(ResourceNode::from_node(input, value))
}

pub fn converts_to_integer<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Bool(false)));
    }

    let single = input.get_single()?;

    let dec_result = try_convert_to_decimal(&single);

    let value = dec_result
        .and_then(|dr| {
            if dr.is_integer() {
                return Some(dr);
            }

            return None;
        })
        .and_then(|dr| Some(Value::Number(Number::from_i128(dr.try_into().ok()?)?)))
        .and_then(|_| Some(Value::Bool(true)))
        .unwrap_or(Value::Bool(false));

    Ok(ResourceNode::from_node(input, value))
}

pub fn to_date<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(vec![])));
    }

    let single = input.get_single()?;

    let date_result = DateTime::try_from(&single)
        .ok()
        .and_then(|dt| Some(Value::String(dt.date.to_date_string())))
        .unwrap_or(Value::Array(vec![]));

    Ok(ResourceNode::from_node(input, date_result))
}

pub fn converts_to_date<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Bool(false)));
    }

    let single = input.get_single()?;

    let date_result = DateTime::try_from(&single)
        .ok()
        .and_then(|_| Some(Value::Bool(true)))
        .unwrap_or(Value::Bool(false));

    Ok(ResourceNode::from_node(input, date_result))
}

pub fn to_datetime<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(vec![])));
    }

    let single = input.get_single()?;

    let datetime_result = DateTime::try_from(&single)
        .ok()
        .and_then(|dt| Some(Value::String(dt.to_string())))
        .unwrap_or(Value::Array(vec![]));

    Ok(ResourceNode::from_node(input, datetime_result))
}

pub fn converts_to_datetime<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Bool(false)));
    }

    let single = input.get_single()?;

    let datetime_result = DateTime::try_from(&single)
        .ok()
        .and_then(|_| Some(Value::Bool(true)))
        .unwrap_or(Value::Bool(false));

    Ok(ResourceNode::from_node(input, datetime_result))
}

pub fn to_decimal<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(vec![])));
    }

    let single = input.get_single()?;

    let dec_result = try_convert_to_decimal(&single);

    let value = dec_result
        .and_then(|dec| Some(Value::Number(Number::from_f64(dec.try_into().ok()?)?)))
        .unwrap_or(Value::Array(vec![]));

    Ok(ResourceNode::from_node(input, value))
}

pub fn converts_to_decimal<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Bool(false)));
    }

    let single = input.get_single()?;

    let dec_result = try_convert_to_decimal(&single);

    let value = dec_result
        .and_then(|dec| Some(Value::Number(Number::from_f64(dec.try_into().ok()?)?)))
        .and_then(|_| Some(Value::Bool(true)))
        .unwrap_or(Value::Bool(false));

    Ok(ResourceNode::from_node(input, value))
}

pub fn to_string<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(vec![])));
    }

    let single = input.get_single()?;

    let value = match &single {
        Value::String(_) => single,
        Value::Bool(bool) => Value::String(bool.to_string()),
        Value::Number(num) => Value::String(num.to_string()),
        Value::Object(_) => Quantity::try_from(&single)
            .and_then(|q| Ok(Value::String(q.to_string())))
            .unwrap_or(Value::Array(vec![])),
        _ => Value::Array(vec![]),
    };

    Ok(ResourceNode::from_node(input, value))
}

pub fn converts_to_string<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(vec![])));
    }

    let single = input.get_single()?;

    let value = match &single {
        Value::String(_) => Value::Bool(true),
        Value::Bool(_) => Value::Bool(true),
        Value::Number(_) => Value::Bool(true),
        Value::Object(_) => Quantity::try_from(&single)
            .and_then(|_| Ok(Value::Bool(true)))
            .unwrap_or(Value::Bool(false)),
        _ => Value::Bool(false),
    };

    Ok(ResourceNode::from_node(input, value))
}

pub fn to_time<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(vec![])));
    }

    let single = input.get_single()?;

    let date_result = Time::try_from(&single)
        .ok()
        .and_then(|time| Some(Value::String(time.to_time_string())))
        .unwrap_or(Value::Array(vec![]));

    Ok(ResourceNode::from_node(input, date_result))
}

pub fn converts_to_time<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Bool(false)));
    }

    let single = input.get_single()?;

    let date_result = Time::try_from(&single)
        .ok()
        .and_then(|_| Some(Value::Bool(true)))
        .unwrap_or(Value::Bool(false));

    Ok(ResourceNode::from_node(input, date_result))
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::evaluate::test::test::{run_tests, TestCase};

    #[test]
    fn test_evaluate_to_integer_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "1.0",
            "b": true,
            "c": 1.1
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.toInteger()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([1]),
            },
            TestCase {
                path: "Patient.b.toInteger()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([1]),
            },
            TestCase {
                path: "Patient.c.toInteger()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_evaluate_converts_to_integer_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "1.0",
            "b": true,
            "c": 1.1
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.convertsToInteger()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.b.convertsToInteger()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.c.convertsToInteger()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_evaluate_to_date_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "@2022",
            "b": "@2022-01-01",
            "c": 1.1,
            "d": "@2022-01-01T01:01:01"
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.toDate()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!(["@2022"]),
            },
            TestCase {
                path: "Patient.b.toDate()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!(["@2022-01-01"]),
            },
            TestCase {
                path: "Patient.c.toDate()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
            TestCase {
                path: "Patient.d.toDate()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!(["@2022-01-01"]),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_evaluate_converts_to_date_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "@2022",
            "b": "@2022-01-01",
            "c": 1.1,
            "d": "@2022-01-01T01:01:01"
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.convertsToDate()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.b.convertsToDate()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.c.convertsToDate()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "Patient.d.convertsToDate()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_evaluate_to_datetime_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "@2022",
            "b": "@2022-01-01",
            "c": 1.1,
            "d": "@2022-01-01T01:01:01.999+01:00"
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.toDateTime()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!(["@2022"]),
            },
            TestCase {
                path: "Patient.b.toDateTime()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!(["@2022-01-01"]),
            },
            TestCase {
                path: "Patient.c.toDateTime()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
            TestCase {
                path: "Patient.d.toDateTime()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!(["@2022-01-01T00:01:01.999"]),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_evaluate_converts_to_datetime_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "@2022",
            "b": "@2022-01-01",
            "c": 1.1,
            "d": "@2022-01-01T01:01:01+01:00"
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.convertsToDateTime()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.b.convertsToDateTime()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.c.convertsToDateTime()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "Patient.d.convertsToDateTime()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_evaluate_to_decimal_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "1.111",
            "b": true,
            "c": 1,
            "d": "a"
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.toDecimal()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([1.111]),
            },
            TestCase {
                path: "Patient.b.toDecimal()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([1.0]),
            },
            TestCase {
                path: "Patient.c.toDecimal()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([1.0]),
            },
            TestCase {
                path: "Patient.d.toDecimal()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_evaluate_converts_to_decimal_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "1.111",
            "b": true,
            "c": 1,
            "d": "a"
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.convertsToDecimal()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.b.convertsToDecimal()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.c.convertsToDecimal()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.d.convertsToDecimal()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_evaluate_to_string_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "1.0",
            "b": true,
            "c": 1.1,
            "d": null,
            "e": "@2022-01-02",
            "f": {
                "value": 1,
                "unit": "year"
            },
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.toString()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!(["1.0"]),
            },
            TestCase {
                path: "Patient.b.toString()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!(["true"]),
            },
            TestCase {
                path: "Patient.c.toString()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!(["1.1"]),
            },
            TestCase {
                path: "Patient.d.toString()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
            TestCase {
                path: "Patient.e.toString()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!(["@2022-01-02"]),
            },
            TestCase {
                path: "Patient.f.toString()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!(["1 'year'"]),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_evaluate_converts_to_string_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "1.0",
            "b": true,
            "c": 1.1,
            "d": null,
            "e": "@2022-01-02",
            "f": {
                "value": 1,
                "unit": "year"
            },
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.convertsToString()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.b.convertsToString()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.c.convertsToString()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.d.convertsToString()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "Patient.e.convertsToString()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.f.convertsToString()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_evaluate_to_time_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "@T12",
            "b": "@T12:20",
            "c": 1.1,
            "d": "@T12:20:01.1",
            "e": "12:20:01.1"
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.toTime()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!(["@T12"]),
            },
            TestCase {
                path: "Patient.b.toTime()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!(["@T12:20"]),
            },
            TestCase {
                path: "Patient.c.toTime()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([]),
            },
            TestCase {
                path: "Patient.d.toTime()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!(["@T12:20:01.1"]),
            },
            TestCase {
                path: "Patient.e.toTime()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!(["@T12:20:01.1"]),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_evaluate_converts_to_time_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "@T12",
            "b": "@T12:20",
            "c": 1.1,
            "d": "@T12:20:01.1",
            "e": "12:20:01.1"
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.convertsToTime()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.b.convertsToTime()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.c.convertsToTime()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
            TestCase {
                path: "Patient.d.convertsToTime()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.e.convertsToTime()".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
        ];

        run_tests(tests);
    }
}
