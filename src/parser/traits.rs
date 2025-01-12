use crate::{error::FhirpathError, lexer::tokens::Token};

// pub trait Matches {
//     fn matches(input: &String, cursor: usize) -> bool;
// }

pub type ParseResult<T> = std::result::Result<T, FhirpathError>;

#[derive(Debug)]
pub struct ParseDetails<T> {
    pub position: usize,
    pub value: T,
}

pub trait Parse {
    fn parse(input: &Vec<Token>, cursor: usize) -> ParseResult<ParseDetails<Box<Self>>>;
}
