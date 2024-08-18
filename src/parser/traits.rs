use super::error::FhirpathError;

pub trait Matches {
    fn matches(input: &String) -> bool;
}

pub type ParseResult<T> = std::result::Result<T, FhirpathError>;

pub trait Parse {
    fn parse(input: &String) -> ParseResult<Box<Self>>;
}
