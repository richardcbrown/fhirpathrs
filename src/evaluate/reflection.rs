use serde::{Serialize, Serializer};
use serde_json::Value;

use crate::{error::FhirpathError, models::ModelDetails, parser::expression::Expression};

use super::{
    data_types::type_info::{Namespace, TypeInfo},
    nodes::resource_node::{PathDetails, ResourceNode},
    EvaluateResult,
};

#[derive(Clone, Debug)]
enum Cardinality {
    Single,
    Multiple,
}

impl TryFrom<&str> for Cardinality {
    type Error = FhirpathError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "1" => Ok(Cardinality::Single),
            "*" => Ok(Cardinality::Multiple),
            _ => Err(FhirpathError::EvaluateError { msg: format!("unknown cardinality: {}", value) })
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum ReflectionType {
    SimpleTypeInfo(SimpleTypeInfo),
    ListTypeInfo(ListTypeInfo),
    ClassInfo(ClassInfo),
    TupleTypeInfo(TupleTypeInfo)
}

#[derive(Clone, Debug)]
pub struct TypeSpecifier {
    namespace: Namespace,
    name: String,
    cardinality: Cardinality
}

impl Serialize for TypeSpecifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted: String = match self.cardinality {
            Cardinality::Single => format!("{}.{}", self.namespace, self.name),
            Cardinality::Multiple => format!("List<{}.{}>", self.namespace, self.name),
        };

        serializer.serialize_str(formatted.as_str())
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleTypeInfo {
    namespace: Namespace,
    name: String,
    base_type: Option<TypeSpecifier>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTypeInfo {
    element_type: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassInfoElement {
    name: String,
    r#type: Option<TypeSpecifier>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TupleTypeInfo {
    element: Vec<TupleTypeInfoElement>
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TupleTypeInfoElement {
    name: String,
    r#type: Option<TypeSpecifier>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassInfo {
    namespace: Namespace,
    name: String,
    base_type: Option<TypeSpecifier>,
    element: Vec<ClassInfoElement>
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
                    cardinality: Cardinality::Single
                }),
            })
        } else {
            Err(FhirpathError::EvaluateError {
                msg: "No namespace for type".to_string(),
            })
        }
    }
}

fn get_class_reflection_type(
    value: &Value,
    fhir_type: &String,
    model: &ModelDetails,
) -> Option<ReflectionType> {
    let obj = match value {
        Value::Object(obj) => obj,
        _ => return None,
    };

    let element: Vec<ClassInfoElement> = obj.keys().filter_map(|elem| {
        let elem_path = format!("{}.{}", fhir_type, elem);

        let elem_type = model.path_to_type.get(elem_path.as_str())?;
        let cardinality = model.path_cardinality.get(elem_path.as_str())?;

        Some(ClassInfoElement {
            name: elem.clone(),
            r#type: Some(TypeSpecifier {
                name: elem_type.clone(),
                namespace: Namespace::Fhir,
                cardinality: Cardinality::try_from(cardinality.as_str()).ok()?
            }),
        })
    }).collect();

    let base_type = model.type_to_parent.get(fhir_type).and_then(|bt| {
        Some(TypeSpecifier { namespace: Namespace::Fhir, name: bt.to_string(), cardinality: Cardinality::Single })
    });

    Some(ReflectionType::ClassInfo(ClassInfo {
        namespace: Namespace::Fhir,
        base_type,
        name: fhir_type.clone(),
        element,
    }))
}

fn get_tuple_reflection_type(
    value: &Value,
    path: &String,
    model: &ModelDetails,
) -> Option<ReflectionType> {
    let obj = match value {
        Value::Object(obj) => obj,
        _ => return None,
    };

    let element: Vec<TupleTypeInfoElement> = obj.keys().filter_map(|elem| {
        let elem_path = format!("{}.{}", path, elem);

        let elem_type = model.path_to_type.get(elem_path.as_str())?;
        let cardinality = model.path_cardinality.get(elem_path.as_str())?;

        Some(TupleTypeInfoElement {
            name: elem.clone(),
            r#type: Some(TypeSpecifier {
                name: elem_type.clone(),
                namespace: Namespace::Fhir,
                cardinality: Cardinality::try_from(cardinality.as_str()).ok()?
            }),
        })
    }).collect();

    Some(ReflectionType::TupleTypeInfo(TupleTypeInfo {
        element
    }))
}

pub fn get_reflection_type(
    path_details: &Option<PathDetails>,
    value: &Value,
    model: &Option<ModelDetails>,
) -> Option<ReflectionType> {
    if let (Some(pd), Some(model_details)) = (path_details, model) {
        let fhir_type = model_details.path_to_type.get(&pd.path)?;

        match fhir_type.as_str() {
            "BackboneElement" =>  get_tuple_reflection_type(
                value,
                &pd.path,
                model_details,
            ),
            _ => get_class_reflection_type(
                value,
                fhir_type,
                model_details,
            ),
        }
    } else {
        return Some(ReflectionType::SimpleTypeInfo(
            SimpleTypeInfo::try_from(value).ok()?,
        ));
    }
}

pub fn reflection_type<'a, 'b>(
    input: &'a ResourceNode<'a, 'b>,
    _expressions: &Vec<Box<Expression>>,
) -> EvaluateResult<ResourceNode<'a, 'b>> {
    let reflection_types = input.get_reflection_types();

    Ok(ResourceNode::from_node(
        input,
        serde_json::to_value(reflection_types).map_err(|err| FhirpathError::EvaluateError {
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
    use crate::evaluate::test::test::Expected;

    #[test]
    fn test_type_path() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "('John' | 'Mary').type()".to_string(),
                input: json!({}),
                expected: Expected::Value(json!([
                    { "namespace": "System", "name": "String", "baseType": "System.Any" },
                    { "namespace": "System", "name": "String", "baseType": "System.Any" }
                ])),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
            TestCase {
                path: "Patient.address.type()".to_string(),
                input: json!({ "resourceType": "Patient", "address": [{ "text": "abc" }] }),
                expected: Expected::Value(json!([
                    {
                        "namespace": "FHIR",
                        "name": "Address",
                        "baseType": "FHIR.Element",
                        "element": [
                            {
                                "name": "text",
                                "type": "FHIR.string"
                            }
                        ]
                    }
                ])),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::R4).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
            TestCase {
                path: "Patient.maritalStatus.type()".to_string(),
                input: json!({ "resourceType": "Patient", "maritalStatus": { "coding": [{ "system": "system", "code": "code" }], "text": "text" } }),
                expected: Expected::Value(json!([
                    {
                        "namespace": "FHIR",
                        "name": "CodeableConcept",
                        "baseType": "FHIR.Element",
                        "element": [
                           { "name": "coding", "type": "List<FHIR.Coding>" },
                           { "name": "text", "type": "FHIR.string" }
                        ]
                    }
                ])),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::R4).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            },
            TestCase {
                path: "Patient.contact.single().type()".to_string(),
                input: json!({ "resourceType": "Patient", "contact": [{ "gender": "male" }]}),
                expected: Expected::Value(json!([
                    {
                        "element": [
                           { "name": "gender", "type": "FHIR.code" },
                        ]
                    }
                ])),
                options: Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::R4).unwrap()),
                    vars: None,
                    now: None,
                    trace_function: None,
                }),
            }
        ];

        run_tests(test_cases);
    }
}
