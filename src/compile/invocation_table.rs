use super::{
    equality::{equal, not_equal},
    filtering::{select, where_function},
    math::add,
    strings::{
        contains, ends_with, index_of, length, lower, matches, replace, replace_matches,
        starts_with, substring, to_chars, upper,
    },
    subsetting::{first, intersect, last, single, skip, tail, take},
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

    map.insert("startsWith".to_string(), starts_with);

    map.insert("endsWith".to_string(), ends_with);

    map.insert("contains".to_string(), contains);

    map.insert("upper".to_string(), upper);

    map.insert("lower".to_string(), lower);

    map.insert("replace".to_string(), replace);

    map.insert("matches".to_string(), matches);

    map.insert("replaceMatches".to_string(), replace_matches);

    map.insert("length".to_string(), length);

    map.insert("toChars".to_string(), to_chars);

    map.insert("!=".to_string(), not_equal);

    map.insert("=".to_string(), equal);

    map.insert("+".to_string(), add);

    map.insert("where".to_string(), where_function);

    map.insert("select".to_string(), select);

    map.insert("single".to_string(), single);

    map.insert("first".to_string(), first);

    map.insert("last".to_string(), last);

    map.insert("tail".to_string(), tail);

    map.insert("skip".to_string(), skip);

    map.insert("take".to_string(), take);

    map.insert("intersect".to_string(), intersect);

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
            data_root: json!([
              { "use": "a" },
              { "use": "b" },
              { "use": "c" }
            ]),
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
