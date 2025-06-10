#[cfg(test)]
pub mod test {
    use assert_json_diff::assert_json_eq;
    use serde_json::Value;

    use crate::evaluate::{compile, EvaluateOptions};

    pub struct TestCase<'a> {
        pub path: String,
        pub input: Value,
        pub options: Option<EvaluateOptions<'a>>,
        pub expected: Value,
    }

    pub fn run_tests(tests: Vec<TestCase>) {
        tests.into_iter().for_each(|test| {
            println!("Test");
            println!("{}", &test.path);

            let compiled = compile(&test.path).unwrap();

            println!("{:?}", compiled.expression);

            let evaluate_result = compiled.evaluate(test.input, test.options).unwrap();

            println!("{:?}", evaluate_result);

            assert_json_eq!(evaluate_result, test.expected);
        });
    }
}
