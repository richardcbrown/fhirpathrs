#[derive(Debug, PartialEq)]
pub enum FhirpathError {
    EvaluateError { msg: String },
}
