use super::{
    aggregate::aggregate,
    collections::{collection_contains, in_collection},
    combining::{combine, union},
    comparison::{gt, gte, lt, lte},
    conversion::{
        converts_to_boolean, converts_to_date, converts_to_datetime, converts_to_decimal,
        converts_to_integer, converts_to_string, converts_to_time, iif, to_boolean, to_date,
        to_datetime, to_decimal, to_integer, to_string, to_time,
    },
    equality::{equal, equivalent, not_equal, not_equivalent},
    existence::{
        all, all_false, all_true, any_false, any_true, count, distinct, empty, exists, is_distinct,
        subset_of, superset_of,
    },
    filtering::{of_type, repeat, select, where_function},
    logic::{and, implies, not, or, xor},
    math::{
        abs, add, amp, ceiling, div, divide, exp, floor, ln, log, mul, power, rem, round, sqrt,
        sub, truncate,
    },
    navigation::{children, descendants},
    reflection::reflection_type,
    strings::{
        contains, ends_with, index_of, length, lower, matches, replace, replace_matches,
        starts_with, substring, to_chars, upper,
    },
    subsetting::{exclude, first, intersect, last, single, skip, tail, take},
    target::Target,
    types::{as_fn, is},
    utility_functions::{now, time_of_day, today},
    EvaluateResult, ResourceNode,
};
use crate::parser::expression::Expression;
use std::collections::HashMap;
use crate::evaluate::trace::trace;

pub fn invocation_table<'a, 'b>() -> HashMap<
    String,
    fn(
        input: &'a ResourceNode<'a, 'b>,
        expressions: &Vec<Box<Expression>>,
    ) -> EvaluateResult<ResourceNode<'a, 'b>>,
> {
    let mut map = HashMap::<
        String,
        fn(
            input: &'a ResourceNode<'a, 'b>,
            expressions: &Vec<Box<Expression>>,
        ) -> EvaluateResult<ResourceNode<'a, 'b>>,
    >::new();

    map.insert("empty".to_string(), empty);

    map.insert("exists".to_string(), exists);

    map.insert("all".to_string(), all);

    map.insert("allTrue".to_string(), all_true);

    map.insert("anyTrue".to_string(), any_true);

    map.insert("allFalse".to_string(), all_false);

    map.insert("anyFalse".to_string(), any_false);

    let subset_of_root = |input: &'a ResourceNode<'a, 'b>, expressions: &Vec<Box<Expression>>| {
        subset_of(input, expressions, Target::AnyAtRoot)
    };

    map.insert("subsetOf".to_string(), subset_of_root);

    let superset_of_root = |input: &'a ResourceNode<'a, 'b>, expressions: &Vec<Box<Expression>>| {
        superset_of(input, expressions, Target::AnyAtRoot)
    };

    map.insert("supersetOf".to_string(), superset_of_root);

    map.insert("count".to_string(), count);

    map.insert("distinct".to_string(), distinct);

    map.insert("isDistinct".to_string(), is_distinct);

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

    map.insert("!~".to_string(), not_equivalent);

    map.insert("~".to_string(), equivalent);

    map.insert("+".to_string(), add);

    map.insert("-".to_string(), sub);

    map.insert("div".to_string(), div);

    map.insert("mod".to_string(), rem);

    map.insert("/".to_string(), divide);

    map.insert("*".to_string(), mul);

    map.insert("&".to_string(), amp);

    map.insert("abs".to_string(), abs);

    map.insert("ceiling".to_string(), ceiling);

    map.insert("exp".to_string(), exp);

    map.insert("floor".to_string(), floor);

    map.insert("ln".to_string(), ln);

    map.insert("log".to_string(), log);

    map.insert("power".to_string(), power);

    map.insert("round".to_string(), round);

    map.insert("sqrt".to_string(), sqrt);

    map.insert("truncate".to_string(), truncate);

    map.insert("children".to_string(), children);

    map.insert("now".to_string(), now);

    map.insert("today".to_string(), today);

    map.insert("timeOfDay".to_string(), time_of_day);

    map.insert(">".to_string(), gt);

    map.insert(">=".to_string(), gte);

    map.insert("<".to_string(), lt);

    map.insert("<=".to_string(), lte);

    map.insert("is".to_string(), is);

    map.insert("as".to_string(), as_fn);

    map.insert("in".to_string(), in_collection);

    map.insert("collection_contains".to_string(), collection_contains);

    map.insert("where".to_string(), where_function);

    map.insert("select".to_string(), select);

    map.insert("repeat".to_string(), repeat);

    map.insert("descendants".to_string(), descendants);

    map.insert("ofType".to_string(), of_type);

    map.insert("single".to_string(), single);

    map.insert("first".to_string(), first);

    map.insert("last".to_string(), last);

    map.insert("tail".to_string(), tail);

    map.insert("skip".to_string(), skip);

    map.insert("take".to_string(), take);

    map.insert("intersect".to_string(), intersect);

    map.insert("exclude".to_string(), exclude);

    let union_root = |input: &'a ResourceNode<'a, 'b>, expressions: &Vec<Box<Expression>>| {
        union(input, expressions, Target::AnyAtRoot)
    };

    map.insert("union".to_string(), union_root);

    let union_expr = |input: &'a ResourceNode<'a, 'b>, expressions: &Vec<Box<Expression>>| {
        union(input, expressions, Target::Expr)
    };

    map.insert("|".to_string(), union_expr);

    map.insert("combine".to_string(), combine);

    map.insert("iif".to_string(), iif);

    map.insert("toBoolean".to_string(), to_boolean);

    map.insert("convertsToBoolean".to_string(), converts_to_boolean);

    map.insert("toInteger".to_string(), to_integer);

    map.insert("convertsToInteger".to_string(), converts_to_integer);

    map.insert("toDate".to_string(), to_date);

    map.insert("convertsToDate".to_string(), converts_to_date);

    map.insert("toDateTime".to_string(), to_datetime);

    map.insert("convertsToDateTime".to_string(), converts_to_datetime);

    map.insert("toDecimal".to_string(), to_decimal);

    map.insert("convertsToDecimal".to_string(), converts_to_decimal);

    map.insert("toString".to_string(), to_string);

    map.insert("convertsToString".to_string(), converts_to_string);

    map.insert("toTime".to_string(), to_time);

    map.insert("convertsToTime".to_string(), converts_to_time);

    map.insert("and".to_string(), and);

    map.insert("or".to_string(), or);

    map.insert("not".to_string(), not);

    map.insert("xor".to_string(), xor);

    map.insert("implies".to_string(), implies);

    map.insert("aggregate".to_string(), aggregate);

    map.insert("type".to_string(), reflection_type);

    map.insert("trace".to_string(), trace);

    map
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use chrono::Utc;
    use serde_json::{json, Value};

    use crate::{
        evaluate::{FhirContext, ResourceNode},
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
            data_root: &json!([
              { "use": "a" },
              { "use": "b" },
              { "use": "c" }
            ]),
            data: json!([
              { "use": "a" },
              { "use": "b" },
              { "use": "c" }
            ]),
            context: &FhirContext {
                model: None,
                vars: HashMap::<String, Value>::new(),
                now: Utc::now(),
                trace_function: None
            },
            path: None,
            fhir_types: vec![],
            resource_context: None,
            reflection_types: vec![],
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

        assert_eq!(result.data, json!([{ "use": "a" }]));
    }
}
