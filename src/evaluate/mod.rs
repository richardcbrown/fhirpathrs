mod aggregate;
mod collections;
mod combining;
mod comparison;
mod conversion;
mod equality;
mod existence;
mod fhir_type;
mod filtering;
mod invocation_table;
mod logic;
mod math;
mod navigation;
mod nodes;
mod strings;
mod subsetting;
mod target;
mod test;
mod types;
mod utils;

use std::collections::HashMap;

use nodes::resource_node::{FhirContext, PathDetails, ResourceNode};
use serde_json::Value;

use crate::error::FhirpathError;
use crate::models::ModelDetails;
use crate::parser::expression::EntireExpression;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub fhirpath);

pub type CompileResult<T> = std::result::Result<T, FhirpathError>;

pub struct CompiledPath {
    expression: Box<EntireExpression>,
}

pub trait Evaluate {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>>;
}

pub trait Text {
    fn text(&self) -> CompileResult<String>;
}

#[derive(Clone)]
pub struct EvaluateOptions {
    model: Option<ModelDetails>,
    vars: Option<HashMap<String, Value>>,
}

impl CompiledPath {
    fn evaluate(&self, resource: Value, options: Option<EvaluateOptions>) -> CompileResult<Value> {
        let opts = options.unwrap_or(EvaluateOptions {
            model: None,
            vars: None,
        });

        let mut vars = HashMap::<String, Value>::new();

        vars.insert(
            "ucum".to_string(),
            Value::String("http://unitsofmeasure.org".to_string()),
        );

        vars.insert("resource".to_string(), resource.clone());
        vars.insert("rootResource".to_string(), resource.clone());

        if let Some(custom_vars) = opts.vars {
            vars.extend(custom_vars);
        }

        let context = FhirContext {
            model: opts.model,
            vars,
        };

        let node = ResourceNode::new(
            &resource,
            None,
            resource.clone(),
            &context,
            None,
            vec![],
            None,
        );

        let evaluate_result = self.expression.evaluate(&node)?;

        dbg!(evaluate_result.path);

        Ok(evaluate_result.data)
    }
}

pub fn compile(path: &String) -> CompileResult<CompiledPath> {
    Ok(CompiledPath {
        expression: Box::new(fhirpath::EntireExpressionParser::new().parse(path).unwrap()),
    })
}

#[cfg(test)]
mod tests {
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    use crate::models::{get_model_details, ModelType};

    use super::*;

    #[test]
    fn evaluates_basic_path() {
        let compiled = compile(&"Patient".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient"
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "resourceType": "Patient"
            }])
        );
    }

    #[test]
    fn evaluate_name_path() {
        let compiled = compile(&"Patient.name".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [{
                "use": "usual",
                "given": ["test"]
            }]
        });

        let evaluate_result = compiled
            .evaluate(
                patient,
                Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                }),
            )
            .unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "usual",
                "given": ["test"]
            }])
        );
    }

    #[test]
    fn evaluate_name_given_path() {
        let compiled = compile(&"Patient.name.given".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [{
                "use": "usual",
                "given": ["test", "test2"]
            }]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        println!("{:?}", evaluate_result);

        assert_json_eq!(evaluate_result, json!(["test", "test2"]));
    }

    #[test]
    fn evaluate_where_path_simple() {
        let compiled = compile(&"Patient.name.where(use = 'usual')".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [{
                "use": "usual",
                "given": ["test"]
            }]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "usual",
                "given": ["test"]
            }])
        );
    }

    #[test]
    fn evaluate_where_path() {
        let compiled = compile(&"Patient.name.where(use = 'usual').given".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [{
                "use": "usual",
                "given": ["test"]
            }]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["test"]));
    }

    #[test]
    fn evaluate_index_path() {
        let compiled = compile(&"Patient.name[0]".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [{
                "use": "usual",
                "given": ["test"]
            }]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "usual",
                "given": ["test"]
            }])
        );
    }

    #[test]
    fn evaluate_index_invocation_path() {
        let compiled = compile(&"Patient.name[0].family".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [{
                "use": "usual",
                "given": ["test"],
                "family": "test"
            }]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["test"]));
    }

    #[test]
    fn evaluate_complex_index_path() {
        let compiled = compile(&"Patient.name.where(use = 'usual').given[1]".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [{
                "use": "usual",
                "given": ["test", "test1"]
            }]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["test1"]));
    }

    #[test]
    fn evaluate_inequality_path() {
        let compiled = compile(&"Patient.name.where(use != 'usual').given".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"]
                },
                {
                    "use": "official",
                    "given": ["test1"]
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["test1"]));
    }

    #[test]
    fn evaluate_indexof_path() {
        let compiled =
            compile(&"Patient.name.where(family.indexOf('test') = -1)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "official",
                "given": ["test1"],
                "family": "abc"
            }])
        );
    }

    #[test]
    fn evaluate_substring_path() {
        let compiled =
            compile(&"Patient.name.where(family.substring(1) = 'est')".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "usual",
                "given": ["test"],
                "family": "test"
            }])
        );
    }

    #[test]
    fn evaluate_startswith_path() {
        let compiled =
            compile(&"Patient.name.where(family.startsWith('tes'))".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "usual",
                "given": ["test"],
                "family": "test"
            }])
        );
    }

    #[test]
    fn evaluate_endswith_path() {
        let compiled = compile(&"Patient.name.where(family.endsWith('bc'))".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "official",
                "given": ["test1"],
                "family": "abc"
            }])
        );
    }

    #[test]
    fn evaluate_contains_path() {
        let compiled = compile(&"Patient.name.where(family.contains('b'))".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "official",
                "given": ["test1"],
                "family": "abc"
            }])
        );
    }

    #[test]
    fn evaluate_upper_path() {
        let compiled = compile(&"Patient.name[0].family.upper()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["TEST"]));
    }

    #[test]
    fn evaluate_lower_path() {
        let compiled = compile(&"Patient.name[0].family.lower()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "TEST"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["test"]));
    }

    #[test]
    fn evaluate_replace_path() {
        let compiled = compile(&"Patient.name.family.replace('es', '')".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["tt", "abc"]));
    }

    #[test]
    fn evaluate_matches_path() {
        let compiled = compile(&"Patient.name[0].family.matches('tes')".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_replace_matches_path() {
        let compiled =
            compile(&"Patient.name.family.replaceMatches('es', '')".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["tt", "abc"]));
    }

    #[test]
    fn evaluate_length_path() {
        let compiled = compile(&"Patient.name.family.length()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([4, 3]));
    }

    #[test]
    fn evaluate_to_chars_path() {
        let compiled = compile(&"Patient.name.family.toChars()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([['t', 'e', 's', 't'], ['a', 'b', 'c']])
        );
    }

    #[test]
    fn evaluate_select_path() {
        let compiled =
            compile(&"Patient.name.select(given[0] + ' ' + family)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["test test", "test1 abc"]));
    }

    #[test]
    fn evaluate_single_path() {
        let compiled = compile(&"Patient.name.single()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "usual",
                "given": ["test"],
                "family": "test"
            }])
        );
    }

    #[test]
    fn evaluate_single_path_no_values() {
        let compiled = compile(&"Patient.name.single()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": []
        });

        let evaluate_result = compiled.evaluate(patient, None);

        assert_eq!(
            evaluate_result,
            Err(FhirpathError::CompileError {
                msg: "Expected array with single element but had 0".to_string()
            })
        )
    }

    #[test]
    fn evaluate_single_path_multiple_values() {
        let compiled = compile(&"Patient.name.single()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None);

        assert_eq!(
            evaluate_result,
            Err(FhirpathError::CompileError {
                msg: "Expected array with single element but had 2".to_string()
            })
        )
    }

    #[test]
    fn evaluate_first_path() {
        let compiled = compile(&"Patient.name.first()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "usual",
                "given": ["test"],
                "family": "test"
            }])
        );
    }

    #[test]
    fn evaluate_first_empty_path() {
        let compiled = compile(&"Patient.name.first()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": []
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_last_path() {
        let compiled = compile(&"Patient.name.last()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "official",
                "given": ["test1"],
                "family": "abc"
            }])
        );
    }

    #[test]
    fn evaluate_last_empty_path() {
        let compiled = compile(&"Patient.name.last()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": []
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_tail_path() {
        let compiled = compile(&"Patient.name.tail()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                },
                {
                    "use": "official",
                    "given": ["test2"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "official",
                "given": ["test1"],
                "family": "abc"
            },
            {
                "use": "official",
                "given": ["test2"],
                "family": "abc"
            }])
        );
    }

    #[test]
    fn evaluate_tail_empty_path() {
        let compiled = compile(&"Patient.name.last()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": []
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_skip_path() {
        let compiled = compile(&"Patient.name.skip(1)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                },
                {
                    "use": "official",
                    "given": ["test2"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "official",
                "given": ["test1"],
                "family": "abc"
            },
            {
                "use": "official",
                "given": ["test2"],
                "family": "abc"
            }])
        );
    }

    #[test]
    fn evaluate_skip_empty_path() {
        let compiled = compile(&"Patient.name.skip(1)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": []
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_take_path() {
        let compiled = compile(&"Patient.name.take(2)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                },
                {
                    "use": "official",
                    "given": ["test2"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "usual",
                "given": ["test"],
                "family": "test"
            },
            {
                "use": "official",
                "given": ["test1"],
                "family": "abc"
            }])
        );
    }

    #[test]
    fn evaluate_take_empty_path() {
        let compiled = compile(&"Patient.name.take(1)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": []
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_intersect_path() {
        let compiled = compile(&"Patient.a.intersect(Patient.b)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2,3],
            "b": [1,2]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([1, 2]));
    }

    #[test]
    fn evaluate_exclude_path() {
        let compiled = compile(&"Patient.a.exclude(Patient.b)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2,3],
            "b": [1,2]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([3]));
    }

    #[test]
    fn evaluate_union_path() {
        let compiled = compile(&"Patient.a.union(Patient.b)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2],
            "b": [1,2,3]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([1, 2, 3]));
    }

    #[test]
    fn evaluate_union_symbol_path() {
        let compiled = compile(&"Patient.a | Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2],
            "b": [1,2,3]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([1, 2, 3]));
    }

    #[test]
    fn evaluate_combine_path() {
        let compiled = compile(&"Patient.a.combine(Patient.b)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2],
            "b": [1,2,3]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([1, 2, 1, 2, 3]));
    }

    #[test]
    fn evaluate_iif_path() {
        let compiled = compile(&"iif(Patient.c, Patient.a, Patient.b)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2],
            "b": [1,2,3],
            "c": true
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([1, 2]));
    }

    #[test]
    fn evaluate_iif_path_default_else() {
        let compiled = compile(&"iif(Patient.c, Patient.a)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_toboolean_true_path() {
        let compiled = compile(&"Patient.a[0].toBoolean()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_toboolean_false_path() {
        let compiled = compile(&"Patient.a[1].toBoolean()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,0],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([false]));
    }

    #[test]
    fn evaluate_toboolean_empty_path() {
        let compiled = compile(&"Patient.b[2].toBoolean()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,0],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_convertstoboolean_true_path() {
        let compiled = compile(&"Patient.a[0].convertsToBoolean()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_convertstoboolean_false_path() {
        let compiled = compile(&"Patient.b[2].convertsToBoolean()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,0],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([false]));
    }

    #[test]
    fn evaluate_empty_true_path() {
        let compiled = compile(&"Patient.a.empty()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_empty_false_path() {
        let compiled = compile(&"Patient.b.empty()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([false]));
    }

    #[test]
    fn evaluate_exists_true_path() {
        let compiled = compile(&"Patient.b.exists()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_exists_false_path() {
        let compiled = compile(&"Patient.a.exists()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([false]));
    }

    #[test]
    fn evaluate_exists_criteria_true_path() {
        let compiled = compile(&"Patient.name.exists(family.endsWith('bc'))".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_exists_criteria_false_path() {
        let compiled = compile(&"Patient.name.exists(family.endsWith('bb'))".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([false]));
    }

    #[test]
    fn evaluate_all_criteria_true_path() {
        let compiled = compile(&"Patient.name.all(family.endsWith('bc'))".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "tebc"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_all_criteria_empty_path() {
        let compiled = compile(&"Patient.name.all(family.endsWith('bc'))".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": []
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_all_criteria_false_path() {
        let compiled = compile(&"Patient.name.all(family.endsWith('bc'))".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([false]));
    }

    #[test]
    fn evaluate_alltrue_true_path() {
        let compiled = compile(&"Patient.b.allTrue()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "b": [1, "1", 1.0, "1.0", "y", "yes", true]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_alltrue_false_path() {
        let compiled = compile(&"Patient.b.allTrue()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "b": [1, "1", 1.0, "1.0", "y", "yes", true, false]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([false]));
    }

    #[test]
    fn evaluate_variable_basic_path() {
        let compiled = compile(&"Patient.b.where(value = %ucum).value".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "b": [
                { "value": "1" },
                { "value": "2" },
                { "value": "http://unitsofmeasure.org"}
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["http://unitsofmeasure.org"]));
    }

    #[test]
    fn evaluate_variable_resource_path() {
        let compiled = compile(&"%resource.b.where(value = %ucum).value".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "b": [
                { "value": "1" },
                { "value": "2" },
                { "value": "http://unitsofmeasure.org"}
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["http://unitsofmeasure.org"]));
    }
}
