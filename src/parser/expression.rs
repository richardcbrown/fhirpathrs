// todo - split_at - panics

use aho_corasick::{AhoCorasick, MatchKind};
use regex::Regex;

use super::{
    identifier::{Identifier, TypeSpecifier},
    invocation::{filter_ignored_data, Invocation, InvocationTerm},
    literal::{LiteralTerm, StringLiteral},
    traits::Parse,
};
use crate::error::FhirpathError;

/**
 * expression
 *   : term                                               #termExpression
 *   | expression '.' invocation                          #invocationExpression
 *   | expression '[' expression ']'                      #indexerExpression
 *   | ('+' | '-') expression                             #polarityExpression
 *   | expression ('*' | '/' | 'div' | 'mod') expression  #multiplicativeExpression
 *   | expression ('+' | '-' | '&') expression            #additiveExpression
 *   | expression '|' expression                          #unionExpression
 *   | expression ('<=' | '<' | '>' | '>=') expression    #inequalityExpression
 *   | expression ('is' | 'as') typeSpecifier             #typeExpression
 *   | expression ('=' | '~' | '!=' | '!~') expression    #equalityExpression
 *   | expression ('in' | 'contains') expression          #membershipExpression
 *   | expression 'and' expression                        #andExpression
 *   | expression ('or' | 'xor') expression               #orExpression
 *   | expression 'implies' expression                    #impliesExpression
 *   //| (IDENTIFIER)? '=>' expression                    #lambdaExpression
 *   ;
 */
#[derive(Debug)]
pub enum Expression {
    TermExpression(Box<TermExpression>),
    InvocationExpression(Box<InvocationExpression>),
    IndexerExpression(Box<IndexerExpression>),
    PolarityExpression(Box<PolarityExpression>),
    MultiplicativeExpression(Box<MultiplicativeExpression>),
    AdditiveExpression(Box<AdditiveExpression>),
    UnionExpression(Box<UnionExpression>),
    InequalityExpression(Box<InequalityExpression>),
    TypeExpression(Box<TypeExpression>),
    EqualityExpression(Box<EqualityExpression>),
    MembershipExpression(Box<MembershipExpression>),
    AndExpression(Box<AndExpression>),
    OrExpression(Box<OrExpression>),
    ImpliesExpression(Box<ImpliesExpression>),
}

// impl Matches for Expression {
//     fn matches(input: &String, cursor: usize) -> bool {
//         TermExpression::matches(input, cursor)
//             || InvocationExpression::matches(input, cursor)
//             || IndexerExpression::matches(input, cursor)
//             || PolarityExpression::matches(input, cursor)
//             || MultiplicativeExpression::matches(input, cursor)
//             || AdditiveExpression::matches(input, cursor)
//             || UnionExpression::matches(input, cursor)
//             || InequalityExpression::matches(input, cursor)
//             || TypeExpression::matches(input, cursor)
//             || EqualityExpression::matches(input, cursor)
//             || MembershipExpression::matches(input, cursor)
//             || AndExpression::matches(input, cursor)
//             || OrExpression::matches(input, cursor)
//             || ImpliesExpression::matches(input, cursor)
//     }
// }

impl Parse for Expression {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        dbg!(input);

        let mut cursor_pos = cursor;

        while cursor_pos < input.len() {
            if let Ok(term_expression) = TermExpression::parse(input, cursor_pos) {
                return Ok(Box::new(Expression::TermExpression(term_expression)));
            } else if let Ok(invocation_expression) = InvocationExpression::parse(input, cursor_pos)
            {
                return Ok(Box::new(Expression::InvocationExpression(
                    invocation_expression,
                )));
            } else if let Ok(indexer_expression) = IndexerExpression::parse(input, cursor_pos) {
                return Ok(Box::new(Expression::IndexerExpression(indexer_expression)));
            } else if let Ok(polarity_expression) = PolarityExpression::parse(input, cursor_pos) {
                return Ok(Box::new(Expression::PolarityExpression(
                    polarity_expression,
                )));
            } else if let Ok(multiplicative_expression) =
                MultiplicativeExpression::parse(input, cursor_pos)
            {
                return Ok(Box::new(Expression::MultiplicativeExpression(
                    multiplicative_expression,
                )));
            } else if let Ok(additive_expresion) = AdditiveExpression::parse(input, cursor_pos) {
                return Ok(Box::new(Expression::AdditiveExpression(additive_expresion)));
            } else if let Ok(union_expression) = UnionExpression::parse(input, cursor_pos) {
                return Ok(Box::new(Expression::UnionExpression(union_expression)));
            } else if let Ok(inequality_expression) = InequalityExpression::parse(input, cursor_pos)
            {
                return Ok(Box::new(Expression::InequalityExpression(
                    inequality_expression,
                )));
            } else if let Ok(type_expression) = TypeExpression::parse(input, cursor_pos) {
                return Ok(Box::new(Expression::TypeExpression(type_expression)));
            } else if let Ok(equality_expression) = EqualityExpression::parse(input, cursor_pos) {
                return Ok(Box::new(Expression::EqualityExpression(
                    equality_expression,
                )));
            } else if let Ok(membership_expression) = MembershipExpression::parse(input, cursor_pos)
            {
                return Ok(Box::new(Expression::MembershipExpression(
                    membership_expression,
                )));
            } else if let Ok(and_expression) = AndExpression::parse(input, cursor_pos) {
                return Ok(Box::new(Expression::AndExpression(and_expression)));
            } else if let Ok(or_expression) = OrExpression::parse(input, cursor_pos) {
                return Ok(Box::new(Expression::OrExpression(or_expression)));
            } else if let Ok(implies_expression) = ImpliesExpression::parse(input, cursor_pos) {
                return Ok(Box::new(Expression::ImpliesExpression(implies_expression)));
            }

            cursor_pos += 1;
        }

        Err(FhirpathError::ParserError {
            msg: "No match".to_string(),
        })
    }
}

#[derive(Debug)]
pub struct ParenthesizedTerm {
    pub children: Vec<Box<Expression>>,
}

static PARENTHESIZED_TERM_REGEX: &str = r"^\((.*)\)$";

// impl Matches for ParenthesizedTerm {
//     fn matches(input: &String, cursor: usize) -> bool {
//         let captures = Regex::captures(&Regex::new(PARENTHESIZED_TERM_REGEX).unwrap(), input);

//         captures
//             .and_then(|capture| {
//                 return Some(Expression::matches(&capture[1].to_string(), 0));
//             })
//             .unwrap_or(false)
//     }
// }

impl Parse for ParenthesizedTerm {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        let captures = Regex::captures(&Regex::new(PARENTHESIZED_TERM_REGEX).unwrap(), input)
            .ok_or_else(|| FhirpathError::ParserError {
                msg: "No match for ParenthesizedTerm".to_string(),
            })?;

        let capture_text = captures[0].to_string();

        let mut children = Vec::<Box<Expression>>::new();

        children.push(Expression::parse(&capture_text, 0)?);

        Ok(Box::new(Self { children }))
    }
}

#[derive(Debug)]
pub enum Term {
    InvocationTerm(Box<InvocationTerm>),
    LiteralTerm(Box<LiteralTerm>),
    ExternalConstantTerm(Box<ExternalConstantTerm>),
    ParenthesizedTerm(Box<ParenthesizedTerm>),
}

// impl Matches for Term {
//     fn matches(input: &String, cursor: usize) -> bool {
//         return InvocationTerm::matches(input, cursor)
//             || LiteralTerm::matches(input, cursor)
//             || ExternalConstantTerm::matches(input, cursor)
//             || ParenthesizedTerm::matches(input, cursor);
//     }
// }

impl Parse for Term {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        if let Ok(invocation_term) = InvocationTerm::parse(input, cursor) {
            return Ok(Box::new(Term::InvocationTerm(invocation_term)));
        } else if let Ok(literal_term) = LiteralTerm::parse(input, cursor) {
            return Ok(Box::new(Term::LiteralTerm(literal_term)));
        } else if let Ok(constant_term) = ExternalConstantTerm::parse(input, cursor) {
            return Ok(Box::new(Term::ExternalConstantTerm(constant_term)));
        } else if let Ok(paranthesized_term) = ParenthesizedTerm::parse(input, cursor) {
            return Ok(Box::new(Term::ParenthesizedTerm(paranthesized_term)));
        }

        Err(FhirpathError::ParserError {
            msg: "Failed to match Term".to_string(),
        })
    }
}

#[derive(Debug)]
pub struct TermExpression {
    pub children: Vec<Box<Term>>,
}

// impl Matches for TermExpression {
//     fn matches(input: &String, cursor: usize) -> bool {
//         Term::matches(input, cursor)
//     }
// }

impl Parse for TermExpression {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        let mut children = Vec::<Box<Term>>::new();

        let term = Term::parse(input, cursor)?;

        children.push(term);

        Ok(Box::new(Self { children }))
    }
}

#[derive(Debug)]
pub enum ExpressionAndInvocation {
    Expression(Expression),
    Invocation(Invocation),
}

#[derive(Debug)]
pub struct InvocationExpression {
    pub children: Vec<Box<ExpressionAndInvocation>>,
}

// fn backtrace_find_dot(input: &String) -> Option<SplitResult> {
//     let mut lparam_count = 0;
//     let mut rparam_count = 0;

//     let mut lchars: Vec<char> = vec![];
//     let mut rchars: Vec<char> = vec![];

//     let mut found_dot: bool = false;

//     let reversed_chars = input.chars().rev();

//     for current_char in reversed_chars {
//         if found_dot {
//             lchars.push(current_char);
//             continue;
//         }

//         if current_char.eq(&')') {
//             rparam_count += 1;
//         }

//         if current_char.eq(&'(') {
//             lparam_count += 1;
//         }

//         if current_char.eq(&'.') && lparam_count.eq(&rparam_count) {
//             found_dot = true;
//             continue;
//         }

//         rchars.push(current_char);
//     }

//     if !found_dot {
//         return None;
//     }

//     return Some(SplitResult {
//         match_string: ".".to_string(),
//         first_segment: lchars.into_iter().rev().collect(),
//         second_segment: rchars.into_iter().rev().collect(),
//     });
// }

// impl Matches for InvocationExpression {
//     fn matches(input: &String, cursor: usize) -> bool {
//         // let split_result = backtrace_find_dot(input);

//         if input.chars().nth(cursor) != '.' {
//             return false;
//         }

//         match split_result {
//             Some(result) => {
//                 Expression::matches(&result.first_segment, 0)
//                     && Invocation::matches(&result.second_segment, 0)
//             }
//             None => false,
//         }
//     }
// }

impl Parse for InvocationExpression {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        let is_dot = input
            .chars()
            .nth(cursor)
            .and_then(|dot| Some(dot == '.'))
            .unwrap_or(false);

        if !is_dot {
            return Err(FhirpathError::ParserError {
                msg: "Invalid InvocationExpression".to_string(),
            });
        }

        let (first, second) = input.split_at(cursor);

        let mut children = Vec::<Box<ExpressionAndInvocation>>::new();

        let expression = Expression::parse(&first.to_string(), 0)?;
        let invocation = Invocation::parse(&second.replacen(".", "", 1).to_string(), 0)?;

        children.push(Box::new(ExpressionAndInvocation::Expression(*expression)));
        children.push(Box::new(ExpressionAndInvocation::Invocation(*invocation)));

        Ok(Box::new(Self { children }))
    }
}

#[derive(Debug)]
pub struct IndexerExpression {
    pub children: Vec<Box<Expression>>,
}

static INDEX_EXPRESSION_REGEX: &str = r"^([^\[\]]*)\[(\d+)\]$";

// impl Matches for IndexerExpression {
//     fn matches(input: &String, cursor: usize) -> bool {
//         let captures = Regex::captures(&Regex::new(INDEX_EXPRESSION_REGEX).unwrap(), input);

//         captures
//             .and_then(|capture| {
//                 return Some(
//                     Expression::matches(&capture[1].to_string(), 0)
//                         && Expression::matches(&capture[2].to_string(), 0),
//                 );
//             })
//             .unwrap_or(false)
//     }
// }

impl Parse for IndexerExpression {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        let captures = Regex::captures(&Regex::new(INDEX_EXPRESSION_REGEX).unwrap(), input);

        let capture = captures.ok_or(FhirpathError::ParserError {
            msg: "Failed to parse IndexerExpression".to_string(),
        })?;

        let mut children = Vec::<Box<Expression>>::new();

        children.push(Expression::parse(&capture[1].to_string(), 0)?);
        children.push(Expression::parse(&capture[2].to_string(), 0)?);

        Ok(Box::new(Self { children }))
    }
}

#[derive(Debug)]
pub struct PolarityExpression {
    pub text: String,
    pub children: Vec<Box<Expression>>,
}

// impl Matches for PolarityExpression {
//     fn matches(input: &String, cursor: usize) -> bool {
//         let mut chars = input.chars();

//         let first_char = chars.next();

//         match first_char {
//             Some('+') => return Expression::matches(&chars.as_str().to_string(), 0),
//             Some('-') => return Expression::matches(&chars.as_str().to_string(), 0),
//             _ => false,
//         }
//     }
// }

impl Parse for PolarityExpression {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        let mut chars = input.chars();

        let operator = chars
            .next()
            .clone()
            .and_then(|c| Some(c.to_string()))
            .ok_or(FhirpathError::ParserError {
                msg: "PolarityExpression contained no first character".to_string(),
            })?;

        if !operator.eq("+") && !operator.eq("-") {
            return Err(FhirpathError::ParserError {
                msg: "No match for PolarityExpression".to_string(),
            });
        }

        let expression: String = chars.collect();

        let mut children = Vec::<Box<Expression>>::new();

        children.push(Box::new(*Expression::parse(&expression, 0)?));

        Ok(Box::new(Self {
            children,
            text: operator,
        }))
    }
}

#[derive(Debug)]
pub struct SplitResult {
    match_string: String,
    first_segment: String,
    second_segment: String,
}

fn split_at_string(input: &String, match_strings: &[&str]) -> Option<SplitResult> {
    let ac = AhoCorasick::builder()
        .match_kind(MatchKind::Standard)
        .build(match_strings)
        .unwrap();

    let mut matches = vec![];

    for mat in ac.find_iter(input) {
        matches.push((mat.pattern(), mat.start(), mat.end()));
    }

    let last_match = matches.last();

    match last_match {
        Some((pattern, start, end)) => {
            let first_segment = &input[..*start];
            let second_segment = &input[*end..];

            return Some(SplitResult {
                first_segment: first_segment.to_string(),
                second_segment: second_segment.to_string(),
                match_string: match_strings[pattern.as_usize()].to_string(),
            });
        }
        None => None,
    }
}

static MULTIPLICATIVE_TERMS: [&str; 4] = ["*", "/", " div ", " mod "];

// fn match_terms(input: &String, match_strings: &[&str]) -> bool {
//     match split_at_string(input, &match_strings) {
//         Some(split_result) => {
//             let first = filter_ignored_data(&split_result.first_segment);
//             let second = filter_ignored_data(&split_result.second_segment);

//             match (first, second) {
//                 (Ok(first_segment), Ok(second_segment)) => {
//                     Expression::parse(&first_segment, 0).is_ok()
//                         && Expression::parse(&second_segment, 0).is_ok()
//                 }
//                 _ => false,
//             }
//         }
//         None => false,
//     }
// }

#[derive(Debug)]
struct OpParsedTerms {
    children: Vec<Box<Expression>>,
    op: String,
}

fn parse_terms(
    input: &String,
    match_strings: &[&str],
) -> super::traits::ParseResult<OpParsedTerms> {
    match split_at_string(input, &match_strings) {
        Some(split_result) => {
            let mut children: Vec<Box<Expression>> = Vec::<Box<Expression>>::new();

            let first = filter_ignored_data(&split_result.first_segment)?;
            let second = filter_ignored_data(&split_result.second_segment)?;

            children.push(Expression::parse(&first, 0)?);
            children.push(Expression::parse(&second, 0)?);

            Ok(OpParsedTerms {
                children,
                op: split_result.match_string,
            })
        }
        None => Err(FhirpathError::ParserError {
            msg: "Failed to parse terms".to_string(),
        }),
    }
}

#[derive(Debug)]
pub struct MultiplicativeExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

// impl Matches for MultiplicativeExpression {
//     fn matches(input: &String, cursor: usize) -> bool {
//         match_terms(input, &MULTIPLICATIVE_TERMS)
//     }
// }

impl Parse for MultiplicativeExpression {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        parse_terms(input, &MULTIPLICATIVE_TERMS)
            .and_then(|opt| {
                Ok(Box::new(Self {
                    children: opt.children,
                    op: opt.op,
                }))
            })
            .or_else(|_e| {
                Err(FhirpathError::ParserError {
                    msg: "Failed to match MultiplicativeExpression".to_string(),
                })
            })
    }
}

static ADDITIVE_TERMS: [&str; 3] = ["+", "-", "&"];

#[derive(Debug)]
pub struct AdditiveExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

// impl Matches for AdditiveExpression {
//     fn matches(input: &String, cursor: usize) -> bool {
//         match_terms(input, &ADDITIVE_TERMS)
//     }
// }

impl Parse for AdditiveExpression {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        parse_terms(input, &ADDITIVE_TERMS)
            .and_then(|opt| {
                Ok(Box::new(Self {
                    children: opt.children,
                    op: opt.op,
                }))
            })
            .or_else(|_e| {
                Err(FhirpathError::ParserError {
                    msg: "Failed to match AdditiveExpression".to_string(),
                })
            })
    }
}

static UNION_TERMS: [&str; 1] = ["|"];

#[derive(Debug)]
pub struct UnionExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

// impl Matches for UnionExpression {
//     fn matches(input: &String, cursor: usize) -> bool {
//         match_terms(input, &UNION_TERMS)
//     }
// }

impl Parse for UnionExpression {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        parse_terms(input, &UNION_TERMS)
            .and_then(|opt| {
                Ok(Box::new(Self {
                    children: opt.children,
                    op: opt.op,
                }))
            })
            .or_else(|_e| {
                Err(FhirpathError::ParserError {
                    msg: "Failed to match UnionExpression".to_string(),
                })
            })
    }
}

static INEQUALITY_TERMS: [&str; 4] = ["<=", "<", ">", ">="];

#[derive(Debug)]
pub struct InequalityExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

// impl Matches for InequalityExpression {
//     fn matches(input: &String, cursor: usize) -> bool {
//         match_terms(input, &INEQUALITY_TERMS)
//     }
// }

impl Parse for InequalityExpression {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        parse_terms(input, &INEQUALITY_TERMS)
            .and_then(|opt| {
                Ok(Box::new(Self {
                    children: opt.children,
                    op: opt.op,
                }))
            })
            .or_else(|_e| {
                Err(FhirpathError::ParserError {
                    msg: "Failed to match InequalityExpression".to_string(),
                })
            })
    }
}

static EQUALITY_TERMS: [&str; 4] = ["=", "~", "!=", "!~"];

#[derive(Debug)]
pub struct EqualityExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

// impl Matches for EqualityExpression {
//     fn matches(input: &String, cursor: usize) -> bool {
//         match_terms(input, &EQUALITY_TERMS)
//     }
// }

impl Parse for EqualityExpression {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        parse_terms(input, &EQUALITY_TERMS)
            .and_then(|opt| {
                Ok(Box::new(Self {
                    children: opt.children,
                    op: opt.op,
                }))
            })
            .or_else(|_e| {
                Err(FhirpathError::ParserError {
                    msg: "Failed to match EqualityExpression".to_string(),
                })
            })
    }
}

static MEMBERSHIP_TERMS: [&str; 2] = [" in ", " contains "];

#[derive(Debug)]
pub struct MembershipExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

// impl Matches for MembershipExpression {
//     fn matches(input: &String, cursor: usize) -> bool {
//         match_terms(input, &MEMBERSHIP_TERMS)
//     }
// }

impl Parse for MembershipExpression {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        parse_terms(input, &MEMBERSHIP_TERMS)
            .and_then(|opt| {
                Ok(Box::new(Self {
                    children: opt.children,
                    op: opt.op,
                }))
            })
            .or_else(|_e| {
                Err(FhirpathError::ParserError {
                    msg: "Failed to match MembershipExpression".to_string(),
                })
            })
    }
}

static AND_TERMS: [&str; 1] = [" and "];

#[derive(Debug)]
pub struct AndExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

// impl Matches for AndExpression {
//     fn matches(input: &String, cursor: usize) -> bool {
//         match_terms(input, &AND_TERMS)
//     }
// }

impl Parse for AndExpression {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        parse_terms(input, &AND_TERMS)
            .and_then(|opt| {
                Ok(Box::new(Self {
                    children: opt.children,
                    op: opt.op,
                }))
            })
            .or_else(|_e| {
                Err(FhirpathError::ParserError {
                    msg: "Failed to match AndExpression".to_string(),
                })
            })
    }
}

static OR_TERMS: [&str; 2] = [" or ", " xor "];

#[derive(Debug)]
pub struct OrExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

// impl Matches for OrExpression {
//     fn matches(input: &String, cursor: usize) -> bool {
//         match_terms(input, &OR_TERMS)
//     }
// }

impl Parse for OrExpression {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        parse_terms(input, &OR_TERMS)
            .and_then(|opt| {
                Ok(Box::new(Self {
                    children: opt.children,
                    op: opt.op,
                }))
            })
            .or_else(|_e| {
                Err(FhirpathError::ParserError {
                    msg: "Failed to match OrExpression".to_string(),
                })
            })
    }
}

static IMPLIES_TERMS: [&str; 1] = [" implies "];

#[derive(Debug)]
pub struct ImpliesExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

// impl Matches for ImpliesExpression {
//     fn matches(input: &String, cursor: usize) -> bool {
//         match_terms(input, &IMPLIES_TERMS)
//     }
// }

impl Parse for ImpliesExpression {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        parse_terms(input, &IMPLIES_TERMS)
            .and_then(|opt| {
                Ok(Box::new(Self {
                    children: opt.children,
                    op: opt.op,
                }))
            })
            .or_else(|_e| {
                Err(FhirpathError::ParserError {
                    msg: "Failed to match ImpliesExpression".to_string(),
                })
            })
    }
}

#[derive(Debug)]
pub enum ExpressionAndTypeSpecifier {
    Expression(Box<Expression>),
    TypeSpecifier(Box<TypeSpecifier>),
}

static TYPE_TERMS: [&str; 2] = [" is ", " as "];

#[derive(Debug)]
pub struct TypeExpression {
    pub children: Vec<Box<ExpressionAndTypeSpecifier>>,
}

fn parse_type_expression(
    input: &String,
    match_strings: &[&str],
) -> super::traits::ParseResult<Vec<Box<ExpressionAndTypeSpecifier>>> {
    match split_at_string(input, &match_strings) {
        Some(split_result) => {
            let mut children: Vec<Box<ExpressionAndTypeSpecifier>> =
                Vec::<Box<ExpressionAndTypeSpecifier>>::new();

            children.push(Box::new(ExpressionAndTypeSpecifier::Expression(
                Expression::parse(&split_result.first_segment, 0)?,
            )));
            children.push(Box::new(ExpressionAndTypeSpecifier::TypeSpecifier(
                TypeSpecifier::parse(&split_result.second_segment, 0)?,
            )));

            Ok(children)
        }
        None => Err(FhirpathError::ParserError {
            msg: "Failed to parse terms".to_string(),
        }),
    }
}

// impl Matches for TypeExpression {
//     fn matches(input: &String, cursor: usize) -> bool {
//         match split_at_string(input, &TYPE_TERMS) {
//             Some(split_result) => {
//                 Expression::matches(&split_result.first_segment, 0)
//                     && TypeSpecifier::matches(&split_result.second_segment, 0)
//             }
//             None => false,
//         }
//     }
// }

impl Parse for TypeExpression {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        parse_type_expression(input, &TYPE_TERMS)
            .and_then(|children| Ok(Box::new(Self { children })))
            .or_else(|_e| {
                Err(FhirpathError::ParserError {
                    msg: "Failed to match TypeExpression".to_string(),
                })
            })
    }
}

#[derive(Debug)]
pub enum IdentifierOrStringLiteral {
    Identifier(Box<Identifier>),
    StringLiteral(Box<StringLiteral>),
}

#[derive(Debug)]
pub struct ExternalConstantTerm {
    pub children: Vec<Box<IdentifierOrStringLiteral>>,
}

// impl Matches for ExternalConstantTerm {
//     fn matches(input: &String, cursor: usize) -> bool {
//         let mut chars = input.chars();

//         let first_char = chars.next();

//         match first_char {
//             Some('%') => {
//                 return Identifier::matches(&chars.as_str().to_string(), 0)
//                     || StringLiteral::matches(&chars.as_str().to_string(), 0)
//             }
//             _ => false,
//         }
//     }
// }

impl Parse for ExternalConstantTerm {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        let identifier_or_string: String = input.chars().skip(1).collect();

        let mut children = Vec::<Box<IdentifierOrStringLiteral>>::new();

        if let Ok(identifier) = Identifier::parse(&identifier_or_string, cursor) {
            children.push(Box::new(IdentifierOrStringLiteral::Identifier(identifier)));
        } else if let Ok(string_literal) = StringLiteral::parse(&identifier_or_string, cursor) {
            children.push(Box::new(IdentifierOrStringLiteral::StringLiteral(
                string_literal,
            )));
        } else {
            return Err(FhirpathError::ParserError {
                msg: "Failed to parse ExternalConstantTerm".to_string(),
            });
        }

        Ok(Box::new(Self { children }))
    }
}
