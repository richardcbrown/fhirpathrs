// todo - split_at - panics

use aho_corasick::{AhoCorasick, MatchKind};
use regex::Regex;

use super::{
    identifier::{Identifier, TypeSpecifier},
    invocation::{filter_ignored_data, Invocation, InvocationTerm},
    literal::{LiteralTerm, StringLiteral},
    traits::{Matches, Parse, ParseResult},
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

impl Matches for Expression {
    fn matches(input: &String) -> bool {
        TermExpression::matches(input)
            || InvocationExpression::matches(input)
            || IndexerExpression::matches(input)
            || PolarityExpression::matches(input)
            || MultiplicativeExpression::matches(input)
            || AdditiveExpression::matches(input)
            || UnionExpression::matches(input)
            || InequalityExpression::matches(input)
            || TypeExpression::matches(input)
            || EqualityExpression::matches(input)
            || MembershipExpression::matches(input)
            || AndExpression::matches(input)
            || OrExpression::matches(input)
            || ImpliesExpression::matches(input)
    }
}

impl Parse for Expression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        dbg!(input);

        if TermExpression::matches(input) {
            return Ok(Box::new(Expression::TermExpression(TermExpression::parse(
                input,
            )?)));
        } else if InvocationExpression::matches(input) {
            return Ok(Box::new(Expression::InvocationExpression(
                InvocationExpression::parse(input)?,
            )));
        } else if IndexerExpression::matches(input) {
            return Ok(Box::new(Expression::IndexerExpression(
                IndexerExpression::parse(input)?,
            )));
        } else if PolarityExpression::matches(input) {
            return Ok(Box::new(Expression::PolarityExpression(
                PolarityExpression::parse(input)?,
            )));
        } else if MultiplicativeExpression::matches(input) {
            return Ok(Box::new(Expression::MultiplicativeExpression(
                MultiplicativeExpression::parse(input)?,
            )));
        } else if AdditiveExpression::matches(input) {
            return Ok(Box::new(Expression::AdditiveExpression(
                AdditiveExpression::parse(input)?,
            )));
        } else if UnionExpression::matches(input) {
            return Ok(Box::new(Expression::UnionExpression(
                UnionExpression::parse(input)?,
            )));
        } else if InequalityExpression::matches(input) {
            return Ok(Box::new(Expression::InequalityExpression(
                InequalityExpression::parse(input)?,
            )));
        } else if TypeExpression::matches(input) {
            return Ok(Box::new(Expression::TypeExpression(TypeExpression::parse(
                input,
            )?)));
        } else if EqualityExpression::matches(input) {
            return Ok(Box::new(Expression::EqualityExpression(
                EqualityExpression::parse(input)?,
            )));
        } else if MembershipExpression::matches(input) {
            return Ok(Box::new(Expression::MembershipExpression(
                MembershipExpression::parse(input)?,
            )));
        } else if AndExpression::matches(input) {
            return Ok(Box::new(Expression::AndExpression(AndExpression::parse(
                input,
            )?)));
        } else if OrExpression::matches(input) {
            return Ok(Box::new(Expression::OrExpression(OrExpression::parse(
                input,
            )?)));
        } else if ImpliesExpression::matches(input) {
            return Ok(Box::new(Expression::ImpliesExpression(
                ImpliesExpression::parse(input)?,
            )));
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

impl Matches for ParenthesizedTerm {
    fn matches(input: &String) -> bool {
        let captures = Regex::captures(&Regex::new(PARENTHESIZED_TERM_REGEX).unwrap(), input);

        captures
            .and_then(|capture| {
                return Some(Expression::matches(&capture[1].to_string()));
            })
            .unwrap_or(false)
    }
}

impl Parse for ParenthesizedTerm {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        let capture_text = Regex::captures(&Regex::new(PARENTHESIZED_TERM_REGEX).unwrap(), input)
            .unwrap()[0]
            .to_string();

        let mut children = Vec::<Box<Expression>>::new();

        children.push(Expression::parse(&capture_text)?);

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

impl Matches for Term {
    fn matches(input: &String) -> bool {
        return InvocationTerm::matches(input)
            || LiteralTerm::matches(input)
            || ExternalConstantTerm::matches(input)
            || ParenthesizedTerm::matches(input);
    }
}

impl Parse for Term {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        if InvocationTerm::matches(input) {
            return Ok(Box::new(Term::InvocationTerm(InvocationTerm::parse(
                input,
            )?)));
        } else if LiteralTerm::matches(input) {
            return Ok(Box::new(Term::LiteralTerm(LiteralTerm::parse(input)?)));
        } else if ExternalConstantTerm::matches(input) {
            return Ok(Box::new(Term::ExternalConstantTerm(
                ExternalConstantTerm::parse(input)?,
            )));
        } else if ParenthesizedTerm::matches(input) {
            return Ok(Box::new(Term::ParenthesizedTerm(ParenthesizedTerm::parse(
                input,
            )?)));
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

impl Matches for TermExpression {
    fn matches(input: &String) -> bool {
        Term::matches(input)
    }
}

impl Parse for TermExpression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        let mut children = Vec::<Box<Term>>::new();

        let term = Term::parse(input)?;

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

fn backtrace_find_dot(input: &String) -> Option<SplitResult> {
    let mut lparam_count = 0;
    let mut rparam_count = 0;

    let mut lchars: Vec<char> = vec![];
    let mut rchars: Vec<char> = vec![];

    let mut found_dot: bool = false;

    let reversed_chars = input.chars().rev();

    for current_char in reversed_chars {
        if found_dot {
            lchars.push(current_char);
            continue;
        }

        if current_char.eq(&')') {
            rparam_count += 1;
        }

        if current_char.eq(&'(') {
            lparam_count += 1;
        }

        if current_char.eq(&'.') && lparam_count.eq(&rparam_count) {
            found_dot = true;
            continue;
        }

        rchars.push(current_char);
    }

    if !found_dot {
        return None;
    }

    return Some(SplitResult {
        match_string: ".".to_string(),
        first_segment: lchars.into_iter().rev().collect(),
        second_segment: rchars.into_iter().rev().collect(),
    });
}

impl Matches for InvocationExpression {
    fn matches(input: &String) -> bool {
        let split_result = backtrace_find_dot(input);

        match split_result {
            Some(result) => {
                Expression::matches(&result.first_segment)
                    && Invocation::matches(&result.second_segment)
            }
            None => false,
        }
    }
}

impl Parse for InvocationExpression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        let split_result = backtrace_find_dot(input);

        match split_result {
            Some(result) => {
                let mut children = Vec::<Box<ExpressionAndInvocation>>::new();

                let expression = Expression::parse(&result.first_segment.to_string())?;
                let invocation = Invocation::parse(&result.second_segment.to_string())?;

                children.push(Box::new(ExpressionAndInvocation::Expression(*expression)));
                children.push(Box::new(ExpressionAndInvocation::Invocation(*invocation)));

                Ok(Box::new(Self { children }))
            }
            None => Err(FhirpathError::ParserError {
                msg: "Invalid InvocationExpression".to_string(),
            }),
        }
    }
}

#[derive(Debug)]
pub struct IndexerExpression {
    pub children: Vec<Box<Expression>>,
}

static INDEX_EXPRESSION_REGEX: &str = r"^([^\[\]]*)\[(\d+)\]$";

impl Matches for IndexerExpression {
    fn matches(input: &String) -> bool {
        let captures = Regex::captures(&Regex::new(INDEX_EXPRESSION_REGEX).unwrap(), input);

        captures
            .and_then(|capture| {
                return Some(
                    Expression::matches(&capture[1].to_string())
                        && Expression::matches(&capture[2].to_string()),
                );
            })
            .unwrap_or(false)
    }
}

impl Parse for IndexerExpression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        let captures = Regex::captures(&Regex::new(INDEX_EXPRESSION_REGEX).unwrap(), input);

        let capture = captures.ok_or(FhirpathError::ParserError {
            msg: "Failed to parse IndexerExpression".to_string(),
        })?;

        let mut children = Vec::<Box<Expression>>::new();

        children.push(Expression::parse(&capture[1].to_string())?);
        children.push(Expression::parse(&capture[2].to_string())?);

        Ok(Box::new(Self { children }))
    }
}

#[derive(Debug)]
pub struct PolarityExpression {
    pub text: String,
    pub children: Vec<Box<Expression>>,
}

impl Matches for PolarityExpression {
    fn matches(input: &String) -> bool {
        let mut chars = input.chars();

        let first_char = chars.next();

        match first_char {
            Some('+') => return Expression::matches(&chars.as_str().to_string()),
            Some('-') => return Expression::matches(&chars.as_str().to_string()),
            _ => false,
        }
    }
}

impl Parse for PolarityExpression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        let mut chars = input.chars();

        let operator = chars
            .next()
            .clone()
            .and_then(|c| Some(c.to_string()))
            .ok_or(FhirpathError::ParserError {
                msg: "PolarityExpression contained no first character".to_string(),
            })?;
        let expression: String = chars.collect();

        let mut children = Vec::<Box<Expression>>::new();

        children.push(Box::new(*Expression::parse(&expression)?));

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

static MULTIPLICATIVE_TERMS: [&str; 4] = ["*", "/", "div", "mod"];

fn match_terms(input: &String, match_strings: &[&str]) -> bool {
    match split_at_string(input, &match_strings) {
        Some(split_result) => {
            let first = filter_ignored_data(&split_result.first_segment);
            let second = filter_ignored_data(&split_result.second_segment);

            match (first, second) {
                (Ok(first_segment), Ok(second_segment)) => {
                    Expression::matches(&first_segment) && Expression::matches(&second_segment)
                }
                _ => false,
            }
        }
        None => false,
    }
}

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

            children.push(Expression::parse(&first)?);
            children.push(Expression::parse(&split_result.second_segment)?);

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

impl Matches for MultiplicativeExpression {
    fn matches(input: &String) -> bool {
        match_terms(input, &MULTIPLICATIVE_TERMS)
    }
}

impl Parse for MultiplicativeExpression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
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

impl Matches for AdditiveExpression {
    fn matches(input: &String) -> bool {
        match_terms(input, &ADDITIVE_TERMS)
    }
}

impl Parse for AdditiveExpression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
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

impl Matches for UnionExpression {
    fn matches(input: &String) -> bool {
        match_terms(input, &UNION_TERMS)
    }
}

impl Parse for UnionExpression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
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

impl Matches for InequalityExpression {
    fn matches(input: &String) -> bool {
        match_terms(input, &INEQUALITY_TERMS)
    }
}

impl Parse for InequalityExpression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
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

impl Matches for EqualityExpression {
    fn matches(input: &String) -> bool {
        match_terms(input, &EQUALITY_TERMS)
    }
}

impl Parse for EqualityExpression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
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

static MEMBERSHIP_TERMS: [&str; 2] = ["in", "contains"];

#[derive(Debug)]
pub struct MembershipExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

impl Matches for MembershipExpression {
    fn matches(input: &String) -> bool {
        match_terms(input, &MEMBERSHIP_TERMS)
    }
}

impl Parse for MembershipExpression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
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

static AND_TERMS: [&str; 1] = ["and"];

#[derive(Debug)]
pub struct AndExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

impl Matches for AndExpression {
    fn matches(input: &String) -> bool {
        match_terms(input, &AND_TERMS)
    }
}

impl Parse for AndExpression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
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

static OR_TERMS: [&str; 2] = ["or", "xor"];

#[derive(Debug)]
pub struct OrExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

impl Matches for OrExpression {
    fn matches(input: &String) -> bool {
        match_terms(input, &OR_TERMS)
    }
}

impl Parse for OrExpression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
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

static IMPLIES_TERMS: [&str; 1] = ["implies"];

#[derive(Debug)]
pub struct ImpliesExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

impl Matches for ImpliesExpression {
    fn matches(input: &String) -> bool {
        match_terms(input, &IMPLIES_TERMS)
    }
}

impl Parse for ImpliesExpression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
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

static TYPE_TERMS: [&str; 2] = ["is", "as"];

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
                Expression::parse(&split_result.first_segment)?,
            )));
            children.push(Box::new(ExpressionAndTypeSpecifier::TypeSpecifier(
                TypeSpecifier::parse(&split_result.second_segment)?,
            )));

            Ok(children)
        }
        None => Err(FhirpathError::ParserError {
            msg: "Failed to parse terms".to_string(),
        }),
    }
}

impl Matches for TypeExpression {
    fn matches(input: &String) -> bool {
        match split_at_string(input, &TYPE_TERMS) {
            Some(split_result) => {
                Expression::matches(&split_result.first_segment)
                    && TypeSpecifier::matches(&split_result.second_segment)
            }
            None => false,
        }
    }
}

impl Parse for TypeExpression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
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

impl Matches for ExternalConstantTerm {
    fn matches(input: &String) -> bool {
        let mut chars = input.chars();

        let first_char = chars.next();

        match first_char {
            Some('%') => {
                return Identifier::matches(&chars.as_str().to_string())
                    || StringLiteral::matches(&chars.as_str().to_string())
            }
            _ => false,
        }
    }
}

impl Parse for ExternalConstantTerm {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        let identifier_or_string: String = input.chars().skip(1).collect();

        let mut children = Vec::<Box<IdentifierOrStringLiteral>>::new();

        if Identifier::matches(&identifier_or_string) {
            children.push(Box::new(IdentifierOrStringLiteral::Identifier(
                Identifier::parse(&identifier_or_string)?,
            )));
        } else if StringLiteral::matches(&identifier_or_string) {
            children.push(Box::new(IdentifierOrStringLiteral::StringLiteral(
                StringLiteral::parse(&identifier_or_string)?,
            )));
        } else {
            return Err(FhirpathError::ParserError {
                msg: "Failed to parse ExternalConstantTerm".to_string(),
            });
        }

        Ok(Box::new(Self { children }))
    }
}
