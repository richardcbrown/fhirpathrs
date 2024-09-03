#[derive(Debug)]
pub enum FhirpathError {
    ParserError { msg: String },
    CompileError { msg: String },
}
