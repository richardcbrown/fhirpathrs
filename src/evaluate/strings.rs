// http://hl7.org/fhirpath/N1/#string-manipulation

use std::cell::LazyCell;
use regex::Regex;
use serde_json::{json, Number, Value};

use crate::{error::FhirpathError, parser::expression::Expression};
use super::{
    utils::{get_string, get_string_from_expression, get_string_vec, get_usize_from_expression},
    EvaluateResult, Evaluate, ResourceNode,
};

const WHITESPACE_REGEX: LazyCell<Regex> = LazyCell::new(|| Regex::new(r"[\s\n\r\t]+").unwrap());

pub fn normalise(string_val: &String) -> String {
    WHITESPACE_REGEX
        .replace_all(string_val.to_lowercase().as_str(), " ")
        .to_string()
}

pub fn index_of<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(Vec::new())));
    }

    let second = &expressions[0];

    let first_val = input.get_single()?;
    let second_val = second.evaluate(input)?.get_single()?;

    let first_string = get_string(&first_val)?;
    let second_string = get_string(&second_val)?;

    let index: i64 = first_string
        .find(&second_string)
        .and_then(|val| u64::try_from(val).ok())
        .and_then(|val| i64::try_from(val).ok())
        .unwrap_or(-1);

    Ok(ResourceNode::from_node(input, json!(index)))
}

pub fn substring<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let first_expr = expressions.first().ok_or(FhirpathError::EvaluateError {
        msg: "Substring expects at least one expression".to_string(),
    })?;

    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(Vec::new())));
    }

    let string_value = get_string(&input.get_single()?)?;

    let second_expr = expressions.iter().nth(1);

    let first_index = match get_usize_from_expression(input, &first_expr) {
        Ok(index) => index,
        Err(_) => return Ok(ResourceNode::from_node(
            input,
            Value::Array(Vec::new()),
        )),
    };

    let length = second_expr
        .and_then(|se| Some(get_usize_from_expression(input, se)))
        .unwrap_or(Ok(string_value.len()))?;

    let string_chars = string_value.chars();

    if first_index >= string_chars.clone().count() {
        return Ok(ResourceNode::from_node(
            input,
            Value::Array(Vec::new()),
        ))
    }

    let skip_chars = string_chars.skip(first_index);

    if length > skip_chars.clone().count() {
        return Ok(ResourceNode::from_node(
            input,
            Value::String(skip_chars.collect::<String>()),
        ))
    }

    Ok(ResourceNode::from_node(
        input,
        Value::String(skip_chars.take(length).collect::<String>()),
    ))
}

pub fn starts_with<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let first_expr = expressions.first().ok_or(FhirpathError::EvaluateError {
        msg: "StartsWith expects one expression".to_string(),
    })?;

    let match_string = get_string_from_expression(input, first_expr)?;

    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(Vec::new())));
    }

    let string_value = get_string(&input.get_single()?)?;

    let starts_with = string_value.starts_with(&match_string);

    Ok(ResourceNode::from_node(input, json!(starts_with)))
}

pub fn ends_with<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let first_expr = expressions.first().ok_or(FhirpathError::EvaluateError {
        msg: "StartsWith expects one expression".to_string(),
    })?;

    let match_string = get_string_from_expression(input, first_expr)?;

    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(Vec::new())));
    }

    let string_value = get_string(&input.get_single()?)?;

    let ends_with = string_value.ends_with(&match_string);

    Ok(ResourceNode::from_node(input, Value::Bool(ends_with)))
}

pub fn contains<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let first_expr = expressions.first().ok_or(FhirpathError::EvaluateError {
        msg: "StartsWith expects one expression".to_string(),
    })?;

    let match_string = get_string_from_expression(input, first_expr)?;

    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(Vec::new())));
    }

    let string_value = get_string(&input.get_single()?)?;

    let contains = string_value.contains(&match_string);

    Ok(ResourceNode::from_node(input, Value::Bool(contains)))
}

pub fn upper<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(Vec::new())));
    }

    let string_values = get_string(&input.get_single()?)?;

    let replaced = string_values.to_uppercase();

    Ok(ResourceNode::from_node(input, Value::String(replaced)))
}

pub fn lower<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(Vec::new())));
    }

    let string_values = get_string(&input.get_single()?)?;

    let replaced = string_values.to_lowercase();

    Ok(ResourceNode::from_node(input, Value::String(replaced)))
}

pub fn replace<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(Vec::new())));
    }

    let string_values = get_string(&input.get_single()?)?;
    let pattern = get_string_from_expression(input, &expressions[0])?;
    let replacement = get_string_from_expression(input, &expressions[1])?;

    let replaced = string_values.replace(pattern.as_str(), replacement.as_str());

    Ok(ResourceNode::from_node(input, Value::String(replaced)))
}

pub fn matches<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(Vec::new())));
    }

    let string_value = get_string(&input.get_single()?)?;
    let pattern = get_string_from_expression(input, &expressions[0])?;
    let regex = Regex::new(&pattern).map_err(|_| FhirpathError::EvaluateError {
        msg: "Failed to parse Regex".to_string(),
    })?;

    let matches = Regex::is_match(&regex, &string_value);

    Ok(ResourceNode::from_node(input, Value::Bool(matches)))
}

pub fn replace_matches<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(Vec::new())));
    }

    let string_value = get_string(&input.get_single()?)?;
    let pattern = get_string_from_expression(input, &expressions[0])?;
    let replacement = get_string_from_expression(input, &expressions[1])?;
    let regex = Regex::new(&pattern).map_err(|_| FhirpathError::EvaluateError {
        msg: "Failed to parse Regex".to_string(),
    })?;

    let replace_result = regex.replace_all(&string_value, &replacement).to_string();

    Ok(ResourceNode::from_node(input, Value::String(replace_result)))
}

pub fn length<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(Vec::new())));
    }

    let string_value = get_string(&input.get_single()?)?;

    Ok(ResourceNode::from_node(input, Value::Number(Number::from(string_value.len()))))
}

pub fn to_chars<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    if input.is_empty()? {
        return Ok(ResourceNode::from_node(input, Value::Array(Vec::new())));
    }

    let string_value = get_string(&input.get_single()?)?;

    let char_set = string_value
        .chars()
        .map(|c| Value::String(c.to_string()))
        .collect();

    Ok(ResourceNode::from_node(input, Value::Array(char_set)))
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use crate::error::FhirpathError;
    use crate::evaluate::test::test::{run_tests, Expected, TestCase};

    #[test]
    fn test_index_of_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "abcdefg",
            "b": ["abc", "123"],
            "c": []
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.indexOf('bc')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([1])),
            },
            TestCase {
                path: "Patient.a.indexOf('x')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([-1])),
            },
            TestCase {
                path: "Patient.a.indexOf('abcdefg')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([0])),
            },
            TestCase {
                path: "Patient.b.indexOf('abcdefg')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Error(FhirpathError::EvaluateError {
                    msg: "Expected single value for node".to_string(),
                }),
            },
            TestCase {
                path: "Patient.c.indexOf('abcdefg')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([])),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_substring_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "abcdefg",
            "b": ["abc", "123"],
            "c": []
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.substring(3)".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!(["defg"])),
            },
            TestCase {
                path: "Patient.a.substring(1, 2)".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!(["bc"])),
            },
            TestCase {
                path: "Patient.a.substring(6, 2)".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!(["g"])),
            },
            TestCase {
                path: "Patient.a.substring(7, 1)".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([])),
            },
            TestCase {
                path: "Patient.b.substring(6, 2)".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Error(FhirpathError::EvaluateError {
                    msg: "Expected single value for node".to_string(),
                }),
            },
            TestCase {
                path: "Patient.c.substring(0, 1)".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([])),
            },
            TestCase {
                path: "Patient.a.substring(-1, 1)".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([])),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_starts_with_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "abcdefg",
            "b": ["abc", "123"],
            "c": []
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.startsWith('abc')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([true])),
            },
            TestCase {
                path: "Patient.a.startsWith('xyz')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([false])),
            },
            TestCase {
                path: "Patient.a.startsWith('')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([true])),
            },
            TestCase {
                path: "Patient.a.startsWith(1)".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Error(FhirpathError::EvaluateError {
                    msg: "Value was not a String".to_string(),
                })
            },
            TestCase {
                path: "Patient.b.startsWith('abc')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Error(FhirpathError::EvaluateError {
                    msg: "Expected single value for node".to_string(),
                }),
            },
            TestCase {
                path: "Patient.c.startsWith('abc')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([])),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_ends_with_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "abcdefg",
            "b": ["abc", "123"],
            "c": []
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.endsWith('efg')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([true])),
            },
            TestCase {
                path: "Patient.a.endsWith('xyz')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([false])),
            },
            TestCase {
                path: "Patient.a.endsWith('')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([true])),
            },
            TestCase {
                path: "Patient.a.endsWith(1)".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Error(FhirpathError::EvaluateError {
                    msg: "Value was not a String".to_string(),
                })
            },
            TestCase {
                path: "Patient.b.endsWith('efg')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Error(FhirpathError::EvaluateError {
                    msg: "Expected single value for node".to_string(),
                }),
            },
            TestCase {
                path: "Patient.c.endsWith('efg')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([])),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_contains_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "abcdefg",
            "b": ["abc", "123"],
            "c": []
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.contains('cde')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([true])),
            },
            TestCase {
                path: "Patient.a.contains('xyz')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([false])),
            },
            TestCase {
                path: "Patient.a.contains('')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([true])),
            },
            TestCase {
                path: "Patient.a.contains(1)".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Error(FhirpathError::EvaluateError {
                    msg: "Value was not a String".to_string(),
                })
            },
            TestCase {
                path: "Patient.b.contains('cde')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Error(FhirpathError::EvaluateError {
                    msg: "Expected single value for node".to_string(),
                }),
            },
            TestCase {
                path: "Patient.c.contains('cde')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([])),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_upper_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "abcdefg",
            "b": ["abc", "123"],
            "c": [],
            "d": ""
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.upper()".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!(["ABCDEFG"])),
            },
            TestCase {
                path: "Patient.d.upper()".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([""])),
            },
            TestCase {
                path: "Patient.b.upper()".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Error(FhirpathError::EvaluateError {
                    msg: "Expected single value for node".to_string(),
                }),
            },
            TestCase {
                path: "Patient.c.upper()".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([])),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_lower_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "ABCDEFG",
            "b": ["ABC", "123"],
            "c": [],
            "d": ""
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.lower()".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!(["abcdefg"])),
            },
            TestCase {
                path: "Patient.d.lower()".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([""])),
            },
            TestCase {
                path: "Patient.b.lower()".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Error(FhirpathError::EvaluateError {
                    msg: "Expected single value for node".to_string(),
                }),
            },
            TestCase {
                path: "Patient.c.lower()".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([])),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_replace_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "abcdefg",
            "b": ["ABC", "123"],
            "c": [],
            "d": "",
            "f": "abc"
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.replace('cde', '123')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!(["ab123fg"])),
            },
            TestCase {
                path: "Patient.a.replace('cde', '')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!(["abfg"])),
            },
            TestCase {
                path: "Patient.d.replace('cde', '')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([""])),
            },
            TestCase {
                path: "Patient.b.replace('cde', '')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Error(FhirpathError::EvaluateError {
                    msg: "Expected single value for node".to_string(),
                }),
            },
            TestCase {
                path: "Patient.c.replace('cde', '')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([])),
            },
            TestCase {
                path: "Patient.f.replace('', 'x')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!(["xaxbxcx"])),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_matches_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "11/30/1972",
            "b": ["ABC", "123"],
            "c": [],
            "d": "",
            "f": "abc"
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.matches('\\b(?<month>\\d{1,2})/(?<day>\\d{1,2})/(?<year>\\d{2,4})\\b')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([true])),
            },
            TestCase {
                path: "Patient.f.matches('\\b(?<month>\\d{1,2})/(?<day>\\d{1,2})/(?<year>\\d{2,4})\\b')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([false])),
            },
            TestCase {
                path: "Patient.d.matches('\\b(?<month>\\d{1,2})/(?<day>\\d{1,2})/(?<year>\\d{2,4})\\b')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([false])),
            },
            TestCase {
                path: "Patient.b.matches('\\b(?<month>\\d{1,2})/(?<day>\\d{1,2})/(?<year>\\d{2,4})\\b')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Error(FhirpathError::EvaluateError {
                    msg: "Expected single value for node".to_string(),
                }),
            },
            TestCase {
                path: "Patient.c.matches('\\b(?<month>\\d{1,2})/(?<day>\\d{1,2})/(?<year>\\d{2,4})\\b')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([])),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_replace_matches_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "11/30/1972",
            "b": ["ABC", "123"],
            "c": [],
            "d": "",
            "f": "abc"
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.replaceMatches('\\b(?<month>\\d{1,2})/(?<day>\\d{1,2})/(?<year>\\d{2,4})\\b',
       '${day}-${month}-${year}')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!(["30-11-1972"])),
            },
            TestCase {
                path: "Patient.f.replaceMatches('\\b(?<month>\\d{1,2})/(?<day>\\d{1,2})/(?<year>\\d{2,4})\\b',
       '${day}-${month}-${year}')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!(["abc"])),
            },
            TestCase {
                path: "Patient.d.replaceMatches('\\b(?<month>\\d{1,2})/(?<day>\\d{1,2})/(?<year>\\d{2,4})\\b',
       '${day}-${month}-${year}')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([""])),
            },
            TestCase {
                path: "Patient.b.replaceMatches('\\b(?<month>\\d{1,2})/(?<day>\\d{1,2})/(?<year>\\d{2,4})\\b',
       '${day}-${month}-${year}')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Error(FhirpathError::EvaluateError {
                    msg: "Expected single value for node".to_string(),
                }),
            },
            TestCase {
                path: "Patient.c.replaceMatches('\\b(?<month>\\d{1,2})/(?<day>\\d{1,2})/(?<year>\\d{2,4})\\b',
       '${day}-${month}-${year}')".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([])),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_length_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "ABCDEFG",
            "b": ["ABC", "123"],
            "c": [],
            "d": ""
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.length()".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([7])),
            },
            TestCase {
                path: "Patient.d.length()".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([0])),
            },
            TestCase {
                path: "Patient.b.length()".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Error(FhirpathError::EvaluateError {
                    msg: "Expected single value for node".to_string(),
                }),
            },
            TestCase {
                path: "Patient.c.length()".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([])),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_to_chars_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": "ABCDEFG",
            "b": ["ABC", "123"],
            "c": [],
            "d": ""
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.a.toChars()".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!(["A", "B", "C", "D", "E", "F", "G"])),
            },
            TestCase {
                path: "Patient.d.toChars".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([])),
            },
            TestCase {
                path: "Patient.b.toChars()".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Error(FhirpathError::EvaluateError {
                    msg: "Expected single value for node".to_string(),
                }),
            },
            TestCase {
                path: "Patient.c.toChars()".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([])),
            },
        ];

        run_tests(tests);
    }
}