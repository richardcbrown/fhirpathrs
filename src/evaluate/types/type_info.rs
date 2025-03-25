use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{error::FhirpathError, evaluate::ResourceNode, models::ModelDetails};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SystemType {
    Integer,
    Decimal,
    Date,
    Time,
    DateTime,
    Boolean,
    String,
    Quantity,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Fhir {
    pub name: String,
}

impl<'a> TryFrom<&NameAndModel<'a>> for Fhir {
    type Error = FhirpathError;

    fn try_from(value: &NameAndModel) -> Result<Self, Self::Error> {
        let model: &ModelDetails =
            value
                .model
                .as_ref()
                .ok_or_else(|| FhirpathError::CompileError {
                    msg: "Cannot infer FHIR TypeInfo without Model".to_string(),
                })?;

        if !model.available_types.contains(&value.name) {
            return Err(FhirpathError::CompileError {
                msg: format!("No FHIR type named {} in model", value.name),
            });
        }

        Ok(Fhir {
            name: value.name.clone(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct System {
    pub name: SystemType,
}

impl<'a> TryFrom<&NameAndModel<'a>> for System {
    type Error = FhirpathError;

    fn try_from(value: &NameAndModel) -> Result<Self, Self::Error> {
        match value.name.as_str() {
            "Integer" => Ok(System {
                name: SystemType::Integer,
            }),
            "Decimal" => Ok(System {
                name: SystemType::Decimal,
            }),
            "Date" => Ok(System {
                name: SystemType::Date,
            }),
            "Time" => Ok(System {
                name: SystemType::Time,
            }),
            "DateTime" => Ok(System {
                name: SystemType::DateTime,
            }),
            "Boolean" => Ok(System {
                name: SystemType::Boolean,
            }),
            "String" => Ok(System {
                name: SystemType::String,
            }),
            "Quantity" => Ok(System {
                name: SystemType::Quantity,
            }),
            _ => Err(FhirpathError::CompileError {
                msg: format!("Invalid System Type {}", value.name),
            }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TypeInfo {
    Fhir(Fhir),
    System(System),
}

struct NameAndModel<'a> {
    pub name: String,
    pub model: &'a Option<ModelDetails>,
}

impl<'a> TryFrom<&ResourceNode<'a>> for TypeInfo {
    type Error = FhirpathError;

    fn try_from(value: &ResourceNode<'a>) -> Result<Self, Self::Error> {
        match &value.data {
            Value::String(string_value) => Ok(TypeInfo::try_from(&TypeDetails {
                fhir_type: Some(string_value.to_string()),
                model: &value.context.model,
            })?),
            _ => Err(FhirpathError::CompileError {
                msg: "ResourceNode data was not a String".to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&TypeDetails<'a>> for TypeInfo {
    type Error = FhirpathError;

    fn try_from(value: &TypeDetails) -> Result<Self, Self::Error> {
        let type_specifier =
            value
                .fhir_type
                .clone()
                .ok_or_else(|| FhirpathError::CompileError {
                    msg: "No type information".to_string(),
                })?;

        let components: Vec<&str> = type_specifier.split('.').collect();

        match components.len() {
            1 => {
                let type_name = components
                    .first()
                    .ok_or_else(|| FhirpathError::CompileError {
                        msg: format!("Invalid TypeInfo {}", type_specifier),
                    })?;

                let name_model = NameAndModel {
                    name: type_name.to_string(),
                    model: &value.model,
                };

                if let Ok(system) = System::try_from(&name_model) {
                    return Ok(TypeInfo::System(system));
                }

                if let Ok(fhir) = Fhir::try_from(&name_model) {
                    return Ok(TypeInfo::Fhir(fhir));
                }

                Err(FhirpathError::CompileError {
                    msg: format!("Invalid TypeInfo {}", type_specifier),
                })
            }
            2 => {
                let namespace = components
                    .first()
                    .ok_or_else(|| FhirpathError::CompileError {
                        msg: format!("Invalid TypeInfo {}", type_specifier),
                    })?;

                let type_name =
                    components
                        .iter()
                        .nth(1)
                        .ok_or_else(|| FhirpathError::CompileError {
                            msg: format!("Invalid TypeInfo {}", type_specifier),
                        })?;

                match *namespace {
                    "FHIR" => Ok(TypeInfo::Fhir(Fhir::try_from(&NameAndModel {
                        name: type_name.to_string(),
                        model: &value.model,
                    })?)),
                    "System" => Ok(TypeInfo::System(System::try_from(&NameAndModel {
                        name: type_name.to_string(),
                        model: &value.model,
                    })?)),
                    _ => Err(FhirpathError::CompileError {
                        msg: format!("Invalid namespace {}", namespace),
                    }),
                }
            }
            _ => Err(FhirpathError::CompileError {
                msg: format!("Invalid TypeInfo {}", type_specifier),
            }),
        }
    }
}

pub struct TypeDetails<'a> {
    pub fhir_type: Option<String>,
    pub model: &'a Option<ModelDetails>,
}
