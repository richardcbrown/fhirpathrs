use std::collections::HashMap;

use serde_json::{json, Value};

use crate::{
    error::FhirpathError,
    evaluate::{
        types::type_info::{TypeDetails, TypeInfo},
        CompileResult,
    },
    models::ModelDetails,
};

pub struct FhirContext {
    pub model: Option<ModelDetails>,
    pub vars: HashMap<String, Value>,
}

#[derive(Clone, Debug)]
pub struct PathDetails {
    pub path: String,
    pub fhir_type: Option<String>,
}

#[derive(Clone)]
pub struct ResourceNode<'a> {
    pub data_root: &'a Value,
    pub parent_node: Option<Box<&'a ResourceNode<'a>>>,
    pub data: Value,
    pub context: &'a FhirContext,
    pub path: Option<String>,
    pub fhir_types: Vec<Option<PathDetails>>,
}

impl<'a> ResourceNode<'a> {
    pub fn new(
        data_root: &'a Value,
        parent_node: Option<Box<&'a ResourceNode<'a>>>,
        data: Value,
        context: &'a FhirContext,
        path: Option<String>,
        fhir_types: Vec<Option<PathDetails>>,
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
}
