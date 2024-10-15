use super::{CompileResult, Evaluate, ResourceNode};
use crate::{error::FhirpathError, parser::expression::Expression};
use serde_json::Value;
use std::collections::HashMap;

fn evaluate_array_expression(array: &Vec<Value>, expr: &Expression) -> Vec<Value> {
    let results: Vec<Value> = array
        .iter()
        .filter_map(|item| {
            let node = ResourceNode {
                data: Some(item.to_owned()),
                parent_node: None,
            };

            expr.evaluate(&node)
                .ok()
                .and_then(|result| result.data)
                .and_then(|value| {
                    if let Value::Bool(bool) = value {
                        if bool {
                            return Some(item.to_owned());
                        }
                    }

                    None
                })
        })
        .collect();

    results
}

fn where_function<'a>(
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    expressions
        .first()
        .and_then(|expr| {
            let data = input.data.as_ref().and_then(|val| match val {
                Value::Array(array) => {
                    let results: Vec<Value> = evaluate_array_expression(array, expr);

                    Some(Value::Array(results))
                }
                _ => None,
            });

            Some(ResourceNode {
                parent_node: Some(Box::new(input)),
                data,
            })
        })
        .ok_or(FhirpathError::CompileError {
            msg: "where_function requires single expression argument".to_string(),
        })
}

pub fn invocation_table<'a>() -> HashMap<
    String,
    fn(
        input: &'a ResourceNode<'a>,
        expressions: &Vec<Box<Expression>>,
    ) -> CompileResult<ResourceNode<'a>>,
> {
    let mut map = HashMap::<
        String,
        fn(
            input: &'a ResourceNode<'a>,
            expressions: &Vec<Box<Expression>>,
        ) -> CompileResult<ResourceNode<'a>>,
    >::new();

    map.insert("where".to_string(), where_function);

    map
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        compile::ResourceNode,
        parser::{
            expression::{EqualityExpression, Expression, Term, TermExpression},
            identifier::{Identifier, LiteralIdentifier},
            invocation::{Invocation, InvocationTerm, MemberInvocation},
            literal::{Literal, LiteralTerm, StringLiteral},
        },
    };

    use super::where_function;

    #[test]
    fn test_where_function() {
        let node = ResourceNode {
            data: Some(json!([
              { "use": "a" },
              { "use": "b" },
              { "use": "c" }
            ])),
            parent_node: None,
        };

        let term_expr1 = TermExpression {
            children: vec![Box::new(Term::InvocationTerm(Box::new(InvocationTerm {
                children: vec![Box::new(Invocation::MemberInvocation(Box::new(
                    MemberInvocation {
                        children: vec![Box::new(Identifier::LiteralIdentifier(Box::new(
                            LiteralIdentifier {
                                text: "use".to_string(),
                            },
                        )))],
                    },
                )))],
            })))],
        };

        let term_expr2 = TermExpression {
            children: vec![Box::new(Term::LiteralTerm(Box::new(LiteralTerm {
                children: vec![Box::new(Literal::StringLiteral(Box::new(StringLiteral {
                    text: "'a'".to_string(),
                })))],
            })))],
        };

        let expressions = vec![Box::new(Expression::EqualityExpression(Box::new(
            EqualityExpression {
                children: vec![
                    Box::new(Expression::TermExpression(Box::new(term_expr1))),
                    Box::new(Expression::TermExpression(Box::new(term_expr2))),
                ],
            },
        )))];

        let result = where_function(&node, &expressions).unwrap();
    }
}
