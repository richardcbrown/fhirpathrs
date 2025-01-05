#[derive(Debug, PartialEq)]
pub enum FhirpathError {
    ParserError { msg: String },
    CompileError { msg: String },
    LexerError { msg: String },
}
