use serde::Serialize;
use serde_json::Value;

use crate::{error::FhirpathError, models::ModelDetails, parser::expression::Expression};

use super::{
    data_types::type_info::{Namespace, TypeInfo},
    nodes::resource_node::{PathDetails, ResourceNode},
    CompileResult,
};

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum ReflectionType {
    SimpleTypeInfo(SimpleTypeInfo),
}

#[derive(Clone, Debug, Serialize)]
pub struct TypeSpecifier {
    namespace: Namespace,
    name: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleTypeInfo {
    namespace: Namespace,
    name: String,
    base_type: Option<TypeSpecifier>,
}

impl TryFrom<&Value> for SimpleTypeInfo {
    type Error = FhirpathError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let type_info = TypeInfo::try_from(value)?;

        if let Some(namespace) = type_info.namespace {
            Ok(SimpleTypeInfo {
                namespace,
                name: type_info.type_name,
                base_type: Some(TypeSpecifier {
                    namespace: Namespace::System,
                    name: "Any".to_string(),
                }),
            })
        } else {
            Err(FhirpathError::CompileError {
                msg: "No namespace for type".to_string(),
            })
        }
    }
}

pub fn get_reflection_type(
    path_details: &Option<PathDetails>,
    value: &Value,
    model: &Option<ModelDetails>,
) -> Option<ReflectionType> {
    if let None = path_details {
        return Some(ReflectionType::SimpleTypeInfo(
            SimpleTypeInfo::try_from(value).ok()?,
        ));
    }

    todo!();
    // if let Some(model_details) = model {}

    // None
}

pub fn reflection_type<'a>(
    input: &'a ResourceNode<'a>,
    _expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    let reflection_types = input.get_reflection_types();

    Ok(ResourceNode::from_node(
        input,
        serde_json::to_value(reflection_types).map_err(|err| FhirpathError::CompileError {
            msg: format!("Failed to Serialize TypeInfo: {}", err.to_string()),
        })?,
    ))
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::{
        evaluate::{
            test::test::{run_tests, TestCase},
            EvaluateOptions,
        },
        models::{get_model_details, ModelType},
    };

    #[test]
    fn test_type_path() {
        let observation = json!({
            "resourceType": "Observation",
            "valueQuantity": {
                "value": 1,
                "unit": "s"
            },
            "component": [
                {
                    "valueQuantity": {
                        "value": 1,
                        "unit": "s"
                    }
                },
                {
                    "valueString": "abc"
                }
            ]
        });

        let test_cases: Vec<TestCase> = vec![TestCase {
            path: "('John' | 'Mary').type()".to_string(),
            input: observation.clone(),
            expected: json!([
                { "namespace": "System", "name": "String", "baseType": { "namespace": "System", "name": "Any" } },
                { "namespace": "System", "name": "String", "baseType": { "namespace": "System", "name": "Any" } }
            ]),
            options: Some(EvaluateOptions {
                model: Some(get_model_details(ModelType::Stu3).unwrap()),
                vars: None,
                now: None,
            }),
        }];

        run_tests(test_cases);
    }
}
