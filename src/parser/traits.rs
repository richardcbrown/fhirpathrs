use crate::error::FhirpathError;

// pub trait Matches {
//     fn matches(input: &String, cursor: usize) -> bool;
// }

pub type ParseResult<T> = std::result::Result<T, FhirpathError>;

pub trait Parse {
    fn parse(input: &String, cursor: usize) -> ParseResult<Box<Self>>;
}
