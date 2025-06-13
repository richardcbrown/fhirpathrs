use super::{FhirContext, PathDetails};

pub fn determine_fhir_type(
    path: &String,
    child_property: &String,
    context: &FhirContext,
) -> PathDetails {
    let mut child_path = format!("{}.{}", path.clone(), child_property);
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

    PathDetails {
        path: child_path,
        fhir_type,
        extensible: child_property.starts_with("_"),
    }
}
