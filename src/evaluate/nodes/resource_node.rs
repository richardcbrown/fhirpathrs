use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::{json, Value};

use crate::{
    error::FhirpathError,
    evaluate::{
        data_types::type_info::{TypeDetails, TypeInfo},
        reflection::{get_reflection_type, ReflectionType},
        CompileResult,
    },
    models::ModelDetails,
};

#[derive(Clone, Debug)]
pub struct FhirContext {
    pub model: Option<ModelDetails>,
    pub vars: HashMap<String, Value>,
    pub now: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct PathDetails {
    pub path: String,
    pub fhir_type: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ResourceNode<'a> {
    pub data_root: &'a Value,
    pub parent_node: Option<Box<&'a ResourceNode<'a>>>,
    pub data: Value,
    pub context: &'a FhirContext,
    pub path: Option<String>,
    pub fhir_types: Vec<Option<PathDetails>>,
    pub reflection_types: Vec<Option<ReflectionType>>,
    pub resource_context: Option<ResourceContext>,
}

#[derive(Clone, Debug)]
pub struct ResourceContext {
    pub index: Option<usize>,
    pub total: Option<Value>,
}

impl<'a> ResourceNode<'a> {
    pub fn new(
        data_root: &'a Value,
        parent_node: Option<Box<&'a ResourceNode<'a>>>,
        data: Value,
        context: &'a FhirContext,
        path: Option<String>,
        fhir_types: Vec<Option<PathDetails>>,
        resource_context: Option<ResourceContext>,
        reflection_types: Vec<Option<ReflectionType>>,
    ) -> Self {
        let node_data = match data {
            Value::Array(array) => json!(array),
            Value::Bool(boolean) => json!([boolean]),
            Value::Number(num) => json!([num]),
            Value::Object(obj) => json!([obj]),
            Value::Null => json!([]),
            Value::String(string) => json!([string]),
        };

        Self {
            data_root,
            parent_node,
            data: node_data,
            context,
            path,
            fhir_types,
            resource_context,
            reflection_types,
        }
    }

    pub fn from_node(node: &'a ResourceNode, data: Value) -> Self {
        Self::new(
            node.data_root,
            Some(Box::new(node)),
            data,
            node.context,
            node.path.clone(),
            node.fhir_types.clone(),
            None,
            node.reflection_types.clone(),
        )
    }

    pub fn is_empty(&self) -> CompileResult<bool> {
        match &self.data {
            Value::Array(array) => Ok(array.len() == 0),
            _ => Err(FhirpathError::CompileError {
                msg: "Data must be a Value::Array".to_string(),
            }),
        }
    }

    pub fn is_single(&self) -> CompileResult<bool> {
        match &self.data {
            Value::Array(array) => Ok(array.len() == 1),
            _ => Err(FhirpathError::CompileError {
                msg: "Data must be a Value::Array".to_string(),
            }),
        }
    }

    pub fn get_single(&self) -> CompileResult<Value> {
        if !self.is_single()? {
            return Err(FhirpathError::CompileError {
                msg: "Expected single value for node".to_string(),
            });
        }

        match &self.data {
            Value::Array(array) => {
                let first = array.first().ok_or_else(|| FhirpathError::CompileError {
                    msg: "Expected single value for node".to_string(),
                })?;

                Ok(first.clone())
            }
            _ => Err(FhirpathError::CompileError {
                msg: "Data must be a Value::Array".to_string(),
            }),
        }
    }

    pub fn get_single_or_empty(&self) -> CompileResult<Option<Value>> {
        match &self.data {
            Value::Array(array) => Ok(array.first().cloned()),
            _ => Err(FhirpathError::CompileError {
                msg: "Data must be a Value::Array".to_string(),
            }),
        }
    }

    pub fn get_array(&self) -> CompileResult<&Vec<Value>> {
        match &self.data {
            Value::Array(array) => Ok(array),
            _ => Err(FhirpathError::CompileError {
                msg: "Data must be a Value::Array".to_string(),
            }),
        }
    }

    pub fn get_var(&self, var_name: &String) -> Option<Value> {
        self.context.vars.get(var_name).cloned()
    }

    pub fn get_type_info(&self) -> Vec<Option<TypeInfo>> {
        self.fhir_types
            .iter()
            .map(|pd| {
                pd.as_ref().and_then(|path_details| {
                    TypeInfo::try_from(&TypeDetails {
                        fhir_type: path_details.fhir_type.clone(),
                        model: &self.context.model,
                    })
                    .ok()
                })
            })
            .collect()
    }

    pub fn get_reflection_types(&self) -> Vec<Option<ReflectionType>> {
        self.get_array()
            .unwrap_or(&vec![])
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let pd = &self.fhir_types.iter().nth(index).unwrap_or(&None);

                get_reflection_type(*pd, value, &self.context.model)
            })
            .collect()
    }

    pub fn get_index(&self) -> Option<usize> {
        let index = self.resource_context.as_ref().and_then(|rc| rc.index);

        index.clone()
    }

    pub fn get_total(&self) -> Option<Value> {
        self.resource_context
            .as_ref()
            .and_then(|rc| rc.total.clone())
    }
}
