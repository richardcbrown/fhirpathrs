use super::{
    equality::{equal, not_equal},
    strings::{index_of, substring},
    where_check::where_function,
    CompileResult, ResourceNode,
};
use crate::parser::expression::Expression;
use std::collections::HashMap;

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

    map.insert("substring".to_string(), substring);

    map.insert("indexOf".to_string(), index_of);

    map.insert("!=".to_string(), not_equal);

    map.insert("=".to_string(), equal);

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
                    text: "a".to_string(),
                })))],
            })))],
        };

        let expressions = vec![Box::new(Expression::EqualityExpression(Box::new(
            EqualityExpression {
                op: "=".to_string(),
                children: vec![
                    Box::new(Expression::TermExpression(Box::new(term_expr1))),
                    Box::new(Expression::TermExpression(Box::new(term_expr2))),
                ],
            },
        )))];

        let result = where_function(&node, &expressions).unwrap();

        assert_eq!(result.data, Some(json!([{ "use": "a" }])));
    }
}
