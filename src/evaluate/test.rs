#[cfg(test)]
pub mod test {
    use assert_json_diff::assert_json_eq;
    use serde_json::Value;
    use crate::error::FhirpathError;
    use crate::evaluate::{compile, EvaluateOptions};

    pub enum Expected {
        Value(Value),
        Error(FhirpathError),
    }

    pub struct TestCase<'a> {
        pub path: String,
        pub input: Value,
        pub options: Option<EvaluateOptions<'a>>,
        pub expected: Expected,
    }

    pub fn run_tests(tests: Vec<TestCase>) {
        tests.into_iter().for_each(|test| {
            println!("Test");
            println!("{}", &test.path);

            let compiled = compile(&test.path).unwrap();

            println!("{:?}", compiled.expression);

            let evaluate_result = compiled.evaluate_single(test.input, test.options);

            println!("{:?}", evaluate_result);

            match test.expected {
                Expected::Value(value) => {
                    let evaluate_value = evaluate_result.unwrap();

                    assert_json_eq!(evaluate_value, value);
                },
                Expected::Error(err) => {
                    let evaluate_error = evaluate_result.err().unwrap();

                    assert_eq!(evaluate_error, err);
                }
            }

        });
    }
}
