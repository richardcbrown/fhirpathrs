use std::{cmp::Ordering, ops::Deref};

use serde_json::Value;

use crate::{error::FhirpathError, parser::expression::Expression};
use crate::evaluate::data_types::quantity::{CalendarUnit, Quantity, TimeUnit};
use super::{
    data_types::{arithmetic_type::ArithmeticType, utils::implicit_convert},
    strings::normalise,
    EvaluateResult, Evaluate, ResourceNode,
};

impl ArithmeticType {
    fn eq(&self, other: &Self) -> Option<bool> {
        let (first, second) = implicit_convert(self, other);

        match (first, second) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => Some(num1.eq(&num2)),
            (ArithmeticType::DateTime(dt1), ArithmeticType::DateTime(dt2)) => {
                let cmp = dt1.partial_cmp(&dt2);

                match cmp {
                    Some(order) => match order {
                        Ordering::Equal => Some(true),
                        _ => Some(false),
                    },
                    None => None,
                }
            }
            (ArithmeticType::String(s1), ArithmeticType::String(s2)) => Some(s1.eq(&s2)),
            (ArithmeticType::Time(t1), ArithmeticType::Time(t2)) => {
                let cmp = t1.partial_cmp(&t2);

                match cmp {
                    Some(order) => match order {
                        Ordering::Equal => Some(true),
                        _ => Some(false),
                    },
                    None => None,
                }
            }
            (ArithmeticType::Quantity(q1), ArithmeticType::Quantity(q2)) => {
                let cmp = q1.partial_cmp(&q2);

                match cmp {
                    Some(order) => match order {
                        Ordering::Equal => Some(true),
                        _ => Some(false),
                    },
                    // in particular for Quantities, Time and Calendar Units return
                    // true/false rather than empty when being compared
                    None => q1.fhir_eq(&q2)
                }
            }
            _ => Some(false),
        }
    }

    fn eqiv(&self, other: &Self) -> Option<bool> {
        let (first, second) = implicit_convert(self, other);

        match (first, second) {
            (ArithmeticType::Number(num1), ArithmeticType::Number(num2)) => Some(num1.eq(&num2)),
            (ArithmeticType::DateTime(dt1), ArithmeticType::DateTime(dt2)) => {
                let cmp = dt1.partial_cmp(&dt2);

                match cmp {
                    Some(order) => match order {
                        Ordering::Equal => Some(true),
                        _ => Some(false),
                    },
                    // if the compare result is None, the datetimes did not have the same
                    // precision so return false here
                    None => Some(false),
                }
            }
            (ArithmeticType::String(s1), ArithmeticType::String(s2)) => {
                let norm_s1 = normalise(&s1);
                let norm_s2 = normalise(&s2);

                Some(norm_s1.eq(&norm_s2))
            }
            (ArithmeticType::Time(t1), ArithmeticType::Time(t2)) => {
                let cmp = t1.partial_cmp(&t2);

                match cmp {
                    Some(order) => match order {
                        Ordering::Equal => Some(true),
                        _ => Some(false),
                    },
                    // if the compare result is None, the times did not have the same
                    // precision so return false here
                    None => Some(false),
                }
            }
            (ArithmeticType::Quantity(q1), ArithmeticType::Quantity(q2)) => {
                let cmp = q1.partial_cmp(&q2);

                match cmp {
                    Some(order) => match order {
                        Ordering::Equal => Some(true),
                        _ => Some(false),
                    },
                    None => None,
                }
            }
            _ => Some(false),
        }
    }
}

pub fn values_are_equal(first: &Value, second: &Value) -> bool {
    first == second
}

fn value_arrays_are_equal(first: &Vec<Value>, second: &Vec<Value>) -> Option<bool> {
    for (index, first_item) in first.iter().enumerate() {
        let result = second.iter().nth(index).and_then(|second_item| {
            if let (Ok(t1), Ok(t2)) = (
                ArithmeticType::try_from(first_item),
                ArithmeticType::try_from(second_item),
            ) {
                return t1.eq(&t2);
            }

            Some(values_are_equal(first_item, second_item))
        });

        match result {
            Some(matched) => {
                if !matched {
                    return Some(false);
                }
            }
            _ => return None,
        }
    }

    Some(true)
}

fn are_equal(input: &ResourceNode, expressions: &Vec<Box<Expression>>) -> EvaluateResult<Value> {
    if expressions.len() != 2 {
        return Err(FhirpathError::EvaluateError {
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

    let result = value_arrays_are_equal(first_val, second_val);

    match result {
        Some(bool) => Ok(Value::Bool(bool)),
        None => Ok(Value::Array(vec![])),
    }
}

pub fn equal<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a>> {
    let result = are_equal(input, expressions)?;

    Ok(ResourceNode::from_node(input, result))
}

pub fn not_equal<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a>> {
    let result = are_equal(input, expressions)?;

    let inverse = match result {
        Value::Bool(val) => Ok(Value::Bool(!val)),
        _ => Err(FhirpathError::EvaluateError {
            msg: "Invalid Boolean value".to_string(),
        }),
    }?;

    Ok(ResourceNode::from_node(input, inverse))
}

fn values_are_equivalent(first: &Value, second: &Value) -> Option<bool> {
    if let (Ok(t1), Ok(t2)) = (
        ArithmeticType::try_from(first),
        ArithmeticType::try_from(second),
    ) {
        return t1.eqiv(&t2);
    }

    match (first, second) {
        (Value::Array(arr1), Value::Array(arr2)) => value_arrays_are_equivalent(&arr1, &arr2),
        (Value::Object(obj1), Value::Object(obj2)) => {
            let first_object_keys: Vec<&String> = obj1.keys().collect();
            let second_object_keys: Vec<&String> = obj2.keys().collect();

            if first_object_keys.len() != second_object_keys.len() {
                return Some(false);
            }

            if !first_object_keys.eq(&second_object_keys) {
                return Some(false);
            }

            for first_key in first_object_keys.iter() {
                let first_val = obj1.get(*first_key);
                let second_val = obj2.get(*first_key);

                match (first_val, second_val) {
                    (Some(first), Some(second)) => {
                        let equivalent = values_are_equivalent(first, second);

                        match equivalent {
                            Some(equiv) => {
                                if !equiv {
                                    return Some(false);
                                }
                            }
                            None => return None,
                        }
                    }
                    _ => return Some(false),
                }
            }

            Some(true)
        }
        (Value::Bool(b1), Value::Bool(b2)) => Some(b1.eq(b2)),
        (Value::Number(n1), Value::Number(n2)) => Some(n1.eq(n2)),
        (Value::String(s1), Value::String(s2)) => Some(normalise(s1).eq(&normalise(s2))),
        (Value::Null, Value::Null) => Some(true),
        _ => Some(false),
    }
}

fn generate_permutations(arrays: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    if arrays.is_empty() {
        return vec![];
    }
    if arrays.len() == 1 {
        return arrays[0].iter().cloned().map(|x| vec![x]).collect();
    }

    let mut permutations = Vec::new();
    let first_array = &arrays[0];
    let remaining_arrays = &arrays[1..];

    for first_element in first_array {
        let next_permutations = generate_permutations(remaining_arrays.to_vec());

        if next_permutations.is_empty() {
            permutations.push(vec![first_element.clone()]);
            continue;
        }

        for mut permutation in next_permutations {
            permutation.insert(0, first_element.clone());

            permutation.sort();

            permutations.push(permutation);
        }
    }

    permutations
}

fn value_arrays_are_equivalent(first: &Vec<Value>, second: &Vec<Value>) -> Option<bool> {
    if first.len() != second.len() {
        return Some(false);
    }

    if first.len() == 0 && second.len() == 0 {
        return Some(true);
    }

    let mut equivalent_indexes: Vec<Vec<usize>> = vec![];

    for first_item in first.iter() {
        // for each item in first array, find items in the
        // second array that are equivalent and get the
        // indexes
        let equivalent_item_indexes: Vec<usize> = second
            .iter()
            .enumerate()
            .filter_map(|(si, second_item)| {
                let are_equivalent = values_are_equivalent(first_item, &second_item);

                if let Some(equivalent) = are_equivalent {
                    if equivalent {
                        return Some(si);
                    }

                    return None;
                }

                return None;
            })
            .collect();

        // no matches in second array so not equivalent
        if equivalent_item_indexes.is_empty() {
            return Some(false);
        }

        equivalent_indexes.push(equivalent_item_indexes);
    }

    // find permutations of matching indexes
    let permutations = generate_permutations(equivalent_indexes);

    // look for a permutation that has all the indexes
    // in this case we can say we have paired all the items
    // in the first array with an item in the second array
    let target_vec: Vec<usize> = Vec::from_iter(0..first.len());

    let equivalent_arrays = permutations
        .iter()
        .find(|perm| perm.deref().eq(&target_vec))
        .and_then(|_| Some(true))
        .unwrap_or(false);

    Some(equivalent_arrays)
}

fn are_equivalent(
    input: &ResourceNode,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<Value> {
    if expressions.len() != 2 {
        return Err(FhirpathError::EvaluateError {
            msg: "Equivalent function takes exactly 2 expressions".to_string(),
        });
    }

    let first = &expressions[0].evaluate(input)?;
    let second = &expressions[1].evaluate(input)?;

    let first_val = first.get_array()?;
    let second_val = second.get_array()?;

    if first_val.is_empty() && second_val.is_empty() {
        return Ok(Value::Bool(true));
    }

    if first_val.len() != second_val.len() {
        return Ok(Value::Bool(false));
    }

    // equivalent if they are equal
    let arrays_are_equal = value_arrays_are_equal(first_val, second_val);

    if let Some(equality_result) = arrays_are_equal {
        if equality_result {
            return Ok(Value::Bool(equality_result));
        }
    }

    match value_arrays_are_equivalent(first_val, second_val) {
        Some(are_equivalent) => Ok(Value::Bool(are_equivalent)),
        None => Ok(Value::Array(vec![])),
    }
}

pub fn equivalent<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a>> {
    Ok(ResourceNode::from_node(
        input,
        are_equivalent(input, expressions)?,
    ))
}

pub fn not_equivalent<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a>> {
    let equivalent = are_equivalent(input, expressions)?;

    let are_not_equivalent = match equivalent {
        Value::Bool(equiv) => Value::Bool(!equiv),
        _ => Value::Array(vec![]),
    };

    Ok(ResourceNode::from_node(input, are_not_equivalent))
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
            TestCase {
                path: "1 'a' = 1 year".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([false]),
            },
        ];

        run_tests(tests);
    }

    #[test]
    fn test_evaluate_equivalent_path() {
        let patient = json!({
            "resourceType": "Patient",
            "birthDate": "2022-01-01",
            "name": [{ "family": "test", "given": ["test1", "test2"] }, { "family": "test", "given": ["test2", "test1"]  }]
        });

        let tests: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.name[0].given ~ Patient.name[1].given".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.name[0] ~ Patient.name[1]".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "Patient.name[0].given ~ Patient.name[1].given".to_string(),
                input: json!({
                    "resourceType": "Patient",
                    "birthDate": "2022-01-01",
                    "name": [{ "family": "test", "given": ["Test1", "test2"] }, { "family": "test", "given": ["test2", "test1"]  }]
                }),
                options: None,
                expected: json!([true]),
            },
            TestCase {
                path: "'test\n\r\tstring' ~ 'test string'".to_string(),
                input: patient.clone(),
                options: None,
                expected: json!([true]),
            },
        ];

        run_tests(tests);
    }
}
