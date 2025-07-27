use serde_json::Value;
use crate::evaluate::data_types::type_info::{system_try_from_value, SystemType};
use super::{FhirContext, PathDetails};

pub struct Path {
    pub path: String,
    pub child_property: Option<String>,
}

pub fn determine_fhir_type(
    value: Option<&Value>,
    path: Option<Path>,
    context: &FhirContext,
    is_extensible_key: bool,
) -> Option<PathDetails> {
    if let Some(path) = path {
        // if path has no child property, and the path points to the
        // resource type of the resource
        if let Some(resource_type) = value.and_then(|val| val.get("resourceType")).and_then(Value::as_str) {
            if resource_type == path.path && path.child_property.is_none() {
                return Some(PathDetails {
                    path: resource_type.to_string(),
                    fhir_type: Some(resource_type.to_string()),
                    extensible: false
                });
            }
        }

        if let Some(child_property) = path.child_property {
            let mut child_path = format!("{}.{}", path.path.clone(), child_property);
            let mut fhir_type: Option<String> = None;

            if let Some(model) = &context.model {
                if let Some(def_path) = model.paths_defined_elsewhere.get(&child_path) {
                    child_path = def_path.to_string();
                }

                fhir_type = model
                    .path_to_type
                    .get(&child_path)
                    .and_then(|t| Some(format!("FHIR.{}", t)));
            };

            return Some(PathDetails {
                path: child_path,
                fhir_type,
                extensible: !is_extensible_key && child_property.starts_with("_"),
            });
        }
    }

    // otherwise try and determine a system type based in the input Value
    if let Ok(system_type) = system_try_from_value(value?) {
        return Some(PathDetails {
            path: "".to_string(),
            fhir_type: Some(format!("System.{}", system_type.to_string())),
            extensible: false
        })
    }

    None
}
