use std::fmt::Display;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{error::FhirpathError, evaluate::ResourceNode, models::ModelDetails};
use crate::evaluate::data_types::date_time::DateTime as DateTimeType;
use crate::evaluate::data_types::date_time::Time as TimeType;
use crate::evaluate::data_types::quantity::Quantity;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

impl SystemType {
    pub fn to_string(&self) -> String {
        match self {
            SystemType::Integer => "Integer".to_string(),
            SystemType::Decimal => "Decimal".to_string(),
            SystemType::Date => "Date".to_string(),
            SystemType::Time => "Time".to_string(),
            SystemType::DateTime => "DateTime".to_string(),
            SystemType::Boolean => "Boolean".to_string(),
            SystemType::String => "String".to_string(),
            SystemType::Quantity => "Quantity".to_string(),
        }
    }
}

fn fhir_try_from(value: &NameAndModel) -> Result<TypeInfo, FhirpathError> {
    let model: &ModelDetails = value
        .model
        .as_ref()
        .ok_or_else(|| FhirpathError::EvaluateError {
            msg: "Cannot infer FHIR TypeInfo without Model".to_string(),
        })?;

    if !model.available_types.contains(&value.name) {
        return Err(FhirpathError::EvaluateError {
            msg: format!("No FHIR type named {} in model", value.name),
        });
    }

    Ok(TypeInfo {
        type_name: value.name.clone(),
        namespace: Some(Namespace::Fhir),
    })
}

pub fn system_try_from_value(value: &Value) -> Result<TypeInfo, FhirpathError> {
    let sys_type = match value {
        Value::Number(n) => {
            if n.is_i64() {
                Ok(SystemType::Integer)
            } else {
                Ok(SystemType::Decimal)
            }
        },
        Value::String(s) => {
            if let Ok(dt) = DateTimeType::try_from(s) {
                match dt.time {
                    Some(_) => Ok(SystemType::DateTime),
                    None => Ok(SystemType::Date),
                }
            } else if let Ok(_) = TimeType::try_from(s) {
                Ok(SystemType::Time)
            } else if let Ok(_) = Quantity::try_from(s) {
                Ok(SystemType::Quantity)
            } else {
                Ok(SystemType::String)
            }
        },
        Value::Object(_) => {
            if let Ok(_) = Quantity::try_from(value) {
                Ok(SystemType::Quantity)
            } else {
                Err(FhirpathError::EvaluateError {
                    msg: "Cannot determine system type".to_string()
                })
            }
        },
        Value::Bool(_) => Ok(SystemType::Boolean),
        _ => Err(FhirpathError::EvaluateError {
            msg: "Cannot determine system type".to_string()
        })
    }?;

    Ok(TypeInfo {
        type_name: sys_type.to_string(),
        namespace: Some(Namespace::System),
    })
}

fn system_try_from(value: &NameAndModel) -> Result<TypeInfo, FhirpathError> {
    let result = match value.name.as_str() {
        "Integer" => Ok(SystemType::Integer),
        "Decimal" => Ok(SystemType::Decimal),
        "Date" => Ok(SystemType::Date),
        "Time" => Ok(SystemType::Time),
        "DateTime" => Ok(SystemType::DateTime),
        "Boolean" => Ok(SystemType::Boolean),
        "String" => Ok(SystemType::String),
        "Quantity" => Ok(SystemType::Quantity),
        _ => Err(FhirpathError::EvaluateError {
            msg: format!("Invalid System Type {}", value.name),
        }),
    }?;

    Ok(TypeInfo {
        namespace: Some(Namespace::System),
        type_name: result.to_string(),
    })
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Namespace {
    #[serde(rename = "FHIR")]
    Fhir,
    System,
}

impl Display for Namespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Namespace::Fhir => write!(f, "FHIR"),
            Namespace::System => write!(f, "System"),
        }
    }
}

impl Namespace {
    pub fn to_string(&self) -> String {
        match self {
            Namespace::Fhir => "FHIR".to_string(),
            Namespace::System => "System".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TypeInfo {
    pub namespace: Option<Namespace>,
    pub type_name: String,
}

struct NameAndModel<'a> {
    pub name: String,
    pub model: &'a Option<ModelDetails>,
}

impl<'a, 'b> TryFrom<&ResourceNode<'a, 'b>> for TypeInfo {
    type Error = FhirpathError;

    fn try_from(value: &ResourceNode<'a, 'b>) -> Result<Self, Self::Error> {
        match &value.get_single()? {
            Value::String(string_value) => Ok(TypeInfo::try_from(&TypeDetails {
                // when we convert a String to an Expression we are currently using
                // StringLiteral, the text() implementation wrap this in quotes
                // so remove the quotes, ideally should have a better way of converting the string
                // to an expression
                fhir_type: Some(string_value.to_string().replace("'", "").replace("`", "")),
                model: &value.context.model,
            })?),
            _ => Err(FhirpathError::EvaluateError {
                msg: "ResourceNode data was not a String".to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&Value> for TypeInfo {
    type Error = FhirpathError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        system_try_from_value(value)
    }
}

impl<'a> TryFrom<&TypeDetails<'a>> for TypeInfo {
    type Error = FhirpathError;

    fn try_from(value: &TypeDetails) -> Result<Self, Self::Error> {
        let type_specifier =
            value
                .fhir_type
                .clone()
                .ok_or_else(|| FhirpathError::EvaluateError {
                    msg: "No type information".to_string(),
                })?;

        let components: Vec<&str> = type_specifier.split('.').collect();

        match components.len() {
            1 => {
                let type_name = components
                    .first()
                    .ok_or_else(|| FhirpathError::EvaluateError {
                        msg: format!("Invalid TypeInfo {}", type_specifier),
                    })?;

                let name_model = NameAndModel {
                    name: type_name.to_string(),
                    model: &value.model,
                };

                if let Ok(system) = system_try_from(&name_model) {
                    return Ok(TypeInfo {
                        namespace: None,
                        type_name: system.type_name,
                    });
                }

                if let Ok(fhir) = fhir_try_from(&name_model) {
                    return Ok(TypeInfo {
                        namespace: None,
                        type_name: fhir.type_name,
                    });
                }

                Err(FhirpathError::EvaluateError {
                    msg: format!("Invalid TypeInfo {}", type_specifier),
                })
            }
            2 => {
                let namespace = components
                    .first()
                    .ok_or_else(|| FhirpathError::EvaluateError {
                        msg: format!("Invalid TypeInfo {}", type_specifier),
                    })?;

                let type_name =
                    components
                        .iter()
                        .nth(1)
                        .ok_or_else(|| FhirpathError::EvaluateError {
                            msg: format!("Invalid TypeInfo {}", type_specifier),
                        })?;

                match *namespace {
                    "FHIR" => Ok(fhir_try_from(&NameAndModel {
                        name: type_name.to_string(),
                        model: &value.model,
                    })?),
                    "System" => Ok(system_try_from(&NameAndModel {
                        name: type_name.to_string(),
                        model: &value.model,
                    })?),
                    _ => Err(FhirpathError::EvaluateError {
                        msg: format!("Invalid namespace {}", namespace),
                    }),
                }
            }
            _ => Err(FhirpathError::EvaluateError {
                msg: format!("Invalid TypeInfo {}", type_specifier),
            }),
        }
    }
}

impl PartialEq for TypeInfo {
    fn eq(&self, other: &Self) -> bool {
        if self.namespace.is_none() || other.namespace.is_none() {
            return self.type_name == other.type_name;
        }

        // if this is a FHIR type and the other is a System type
        // they are implicitly equal if the type names are the same
        if let Some(self_ns) = &self.namespace {
            if self_ns.eq(&Namespace::Fhir) {
                return self.type_name == other.type_name;
            }
        }

        self.namespace == other.namespace && self.type_name == other.type_name
    }
}

pub struct TypeDetails<'a> {
    pub fhir_type: Option<String>,
    pub model: &'a Option<ModelDetails>,
}
