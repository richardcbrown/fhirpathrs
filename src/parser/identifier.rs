use super::traits::{Parse, ParseResult};
use crate::{error::FhirpathError, lexer::tokens::Token};
use regex::Regex;

#[derive(Debug)]
pub enum Identifier {
    LiteralIdentifier(Box<LiteralIdentifier>),
    LiteralDelimitedIdentifier(Box<LiteralDelimitedIdentifier>),
    LiteralAs(Box<LiteralAs>),
    LiteralIs(Box<LiteralIs>),
    LiteralContains(Box<LiteralContains>),
    LiteralIn(Box<LiteralIn>),
}

// impl Matches for Identifier {
//     fn matches(input: &String, cursor: usize) -> bool {
//         return LiteralIdentifier::matches(input, cursor);
//     }
// }

impl Parse for Identifier {
    fn parse(input: &Vec<Token>, cursor: usize) -> ParseResult<Box<Self>> {
        if let Ok(literal_identifier) = LiteralIdentifier::parse(input, cursor) {
            return Ok(Box::new(Identifier::LiteralIdentifier(literal_identifier)));
        }

        Err(FhirpathError::ParserError {
            msg: "Failed to match Identifier".to_string(),
        })
    }
}

#[derive(Debug)]
pub struct LiteralIdentifier {
    pub text: String,
}

static IDENTIFIER_REGEX: &str = "^([A-Za-z]|_)([A-Za-z0-9]|_)*$";

// impl Matches for LiteralIdentifier {
//     fn matches(input: &String, cursor: usize) -> bool {
//         Regex::is_match(&Regex::new(IDENTIFIER_REGEX).unwrap(), input)
//     }
// }

impl Parse for LiteralIdentifier {
    fn parse(input: &Vec<Token>, cursor: usize) -> ParseResult<Box<Self>> {
        let captures =
            Regex::captures(&Regex::new(IDENTIFIER_REGEX).unwrap(), input).ok_or_else(|| {
                FhirpathError::ParserError {
                    msg: "No match for LiteralIdentifier".to_string(),
                }
            })?;

        let capture_text = captures[0].to_string();

        Ok(Box::new(Self { text: capture_text }))
    }
}

static UNICODE_REGEX: &str = "u[0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F]";

static ESC_REGEX: &str = "\\\\([`'\\/fnrt]|UNICODE)";

static DELIMITED_IDENTIFIER_REGEX: &str = "`(ESC|^[\\`])*`";

fn esc_regex() -> String {
    ESC_REGEX.replace("UNICODE", UNICODE_REGEX)
}

fn delimited_identifier_regex() -> String {
    DELIMITED_IDENTIFIER_REGEX.replace("ESC", esc_regex().as_str())
}

#[derive(Debug)]
pub struct LiteralDelimitedIdentifier {}

#[derive(Debug)]
pub struct LiteralAs {}

#[derive(Debug)]
pub struct LiteralIs {}

#[derive(Debug)]
pub struct LiteralContains {}

#[derive(Debug)]
pub struct LiteralIn {}

#[derive(Debug)]
pub enum TypeSpecifier {
    QualifiedIdentifier(Box<QualifiedIdentifier>),
}

// impl Matches for TypeSpecifier {
//     fn matches(input: &String, cursor: usize) -> bool {
//         QualifiedIdentifier::matches(input, cursor)
//     }
// }

impl Parse for TypeSpecifier {
    fn parse(input: &Vec<Token>, cursor: usize) -> ParseResult<Box<Self>> {
        Ok(Box::new(TypeSpecifier::QualifiedIdentifier(
            QualifiedIdentifier::parse(input, cursor)?,
        )))
    }
}

#[derive(Debug)]
pub struct QualifiedIdentifier {
    pub children: Vec<Box<Identifier>>,
}

impl Parse for QualifiedIdentifier {
    fn parse(input: &Vec<Token>, cursor: usize) -> ParseResult<Box<Self>> {
        let mut children = Vec::<Box<Identifier>>::new();

        let identifiers: Vec<&str> = input.split('.').collect();

        for identifier in identifiers.iter() {
            children.push(Identifier::parse(&identifier.to_string(), 0)?);
        }

        Ok(Box::new(Self { children }))
    }
}
