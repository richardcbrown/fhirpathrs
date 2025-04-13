use serde_json::Value;

use crate::parser::{
    expression::{Expression, Term, TermExpression},
    identifier::{Identifier, LiteralIdentifier},
    invocation::{FunctionInvocation, IdentifierAndParamList, Invocation, InvocationTerm},
};

use super::{filtering::repeat, nodes::resource_node::ResourceNode, CompileResult};

pub fn children<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let children = input
        .get_array()?
        .iter()
        .try_fold(vec![], |mut acc, item| {
            match item {
                Value::Object(obj) => {
                    let mut values: Vec<Value> = obj.values().map(|item| item.to_owned()).collect();

                    acc.append(&mut values);
                }
                _ => {}
            }

            Ok(acc)
        })?;

    Ok(ResourceNode::from_node(input, Value::Array(children)))
}

pub fn descendants<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let children_expression = Expression::TermExpression(Box::new(TermExpression {
        children: vec![Box::new(Term::InvocationTerm(Box::new(InvocationTerm {
            children: vec![Box::new(Invocation::FunctionInvocation(Box::new(
                FunctionInvocation {
                    children: vec![Box::new(IdentifierAndParamList::Identifier(Box::new(
                        Identifier::LiteralIdentifier(Box::new(LiteralIdentifier {
                            text: "children".to_string(),
                        })),
                    )))],
                },
            )))],
        })))],
    }));

    repeat(input, &vec![Box::new(children_expression)])
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::evaluate::test::test::{run_tests, TestCase};

    #[test]
    fn evaluate_children_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": 2,
            "b": 6
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.children()".to_string(),
                input: patient.clone(),
                expected: json!([2, 6, "Patient"]),
                options: None,
            },
            TestCase {
                path: "Patient.children()".to_string(),
                input: json!({
                  "resourceType": "Patient",
                  "a": [2, 3],
                  "b": 6
                }),
                expected: json!([[2, 3], 6, "Patient"]),
                options: None,
            },
        ];

        run_tests(test_cases);
    }

    #[test]
    fn evaluate_descendants_path() {
        let patient = json!({
            "resourceType": "Patient",
            "a": 2,
            "b": 6
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.descendants()".to_string(),
                input: patient.clone(),
                expected: json!([2, 6, "Patient"]),
                options: None,
            },
            TestCase {
                path: "Patient.descendants()".to_string(),
                input: json!({
                  "resourceType": "Patient",
                  "a": [2, 3],
                  "b": 6
                }),
                expected: json!([[2, 3], 6, "Patient"]),
                options: None,
            },
            TestCase {
                path: "Patient.descendants()".to_string(),
                input: json!({
                  "resourceType": "Patient",
                  "c": { "e": { "f": { "g": 2 } } }
                }),
                expected: json!([{ "e": { "f": { "g": 2 } } }, "Patient", { "f": { "g": 2 } }, { "g": 2 }, 2]),
                options: None,
            },
            TestCase {
                path: "Patient.descendants()".to_string(),
                input: json!({
                  "resourceType": "Patient",
                  "a": [2, 3],
                  "b": 6,
                  "c": { "d": 1, "e": { "f": 2 } }
                }),
                expected: json!([[2, 3], 6, { "d": 1, "e": { "f": 2 } }, "Patient", 1, { "f": 2 }, 2]),
                options: None,
            },
        ];

        run_tests(test_cases);
    }
}
