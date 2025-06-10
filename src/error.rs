#[derive(Debug, PartialEq)]
pub enum FhirpathError {
    ParserError { msg: String },
    EvaluateError { msg: String },
    LexerError { msg: String },
}
