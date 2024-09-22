use super::traits::{Matches, Parse, ParseResult};
use crate::error::FhirpathError;
use regex::Regex;

pub enum Identifier {
    LiteralIdentifier(Box<LiteralIdentifier>),
    LiteralDelimitedIdentifier(Box<LiteralDelimitedIdentifier>),
}

impl Matches for Identifier {
    fn matches(input: &String) -> bool {
        return LiteralIdentifier::matches(input);
    }
}

impl Parse for Identifier {
    fn parse(input: &String) -> ParseResult<Box<Self>> {
        if LiteralIdentifier::matches(input) {
            return Ok(Box::new(Identifier::LiteralIdentifier(
                LiteralIdentifier::parse(input)?,
            )));
        }

        Err(FhirpathError::ParserError {
            msg: "Failed to match Identifier".to_string(),
        })
    }
}

pub struct LiteralIdentifier {
    pub text: String,
}

static IDENTIFIER_REGEX: &str = "([A-Za-z]|'_')([A-Za-z0-9]|'_')*";

impl Matches for LiteralIdentifier {
    fn matches(input: &String) -> bool {
        Regex::is_match(&Regex::new(IDENTIFIER_REGEX).unwrap(), input)
    }
}

impl Parse for LiteralIdentifier {
    fn parse(input: &String) -> ParseResult<Box<Self>> {
        let capture_text =
            Regex::captures(&Regex::new(IDENTIFIER_REGEX).unwrap(), input).unwrap()[0].to_string();

        Ok(Box::new(Self { text: capture_text }))
    }
}

static DELIMITED_IDENTIFIER_REGEX: &str = "`(ESC | ~[\\`])*`";

pub struct LiteralDelimitedIdentifier {}

pub struct LiteralAs {}

pub struct LiteralIs {}

pub struct LiteralContains {}

pub struct LiteralIn {}

pub enum TypeSpecifier {
    QualifiedIdentifier(QualifiedIdentifier),
}

pub enum QualifiedIdentifier {
    LiteralQualifiedIdentifier(LiteralQualifiedIdentifier),
}

pub struct LiteralQualifiedIdentifier {
    pub text: String,
}

impl Matches for LiteralQualifiedIdentifier {
    fn matches(input: &String) -> bool {
        let identifiers: Vec<&str> = input.split('.').collect();

        identifiers
            .iter()
            .all(|&identifier| Identifier::matches(&identifier.to_string()))
    }
}

impl Parse for LiteralQualifiedIdentifier {
    fn parse(input: &String) -> ParseResult<Box<Self>> {
        Ok(Box::new(Self {
            text: input.to_owned(),
        }))
    }
}
