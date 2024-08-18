use regex::Regex;

use super::traits::Matches;

enum Identifier {
    LiteralIdentifier(LiteralIdentifier),
}

pub struct LiteralIdentifier {}

static IDENTIFIER_REGEX: &str = "([A-Za-z] | '_')([A-Za-z0-9] | '_')*";

impl Matches for LiteralIdentifier {
    fn matches(input: &String) -> bool {
        Regex::is_match(&Regex::new(IDENTIFIER_REGEX).unwrap(), input)
    }
}

pub struct LiteralDelimitedIdentifier {}

pub struct LiteralAs {}

pub struct LiteralIs {}

pub struct LiteralContains {}

pub struct LiteralIn {}
