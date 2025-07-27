use serde_json::{json, Value};

use crate::{
    error::FhirpathError,
    evaluate::{
        fhir_type::determine_fhir_type,
        nodes::resource_node::{PathDetails, ResourceNode},
        EvaluateResult, Evaluate, Text,
    },
    parser::invocation::MemberInvocation,
};
use crate::evaluate::fhir_type::Path;
use crate::evaluate::nodes::utils::capitalise;

fn expand_choice_values<'a, 'b>(input: &'a ResourceNode<'a, 'b>, property: &String) -> Vec<String> {
    // if there is no model there's nothing to expand
    let model = &input.context.model;

    if model.is_none() {
        return vec![property.clone()];
    }

    // if there's no path there's nothing to expand
    let path = input.path.as_ref().and_then(|p| Some(p.clone()));

    if path.is_none() {
        return vec![property.clone()];
    }

    let choice_path = path.and_then(|p| Some(vec![p, property.clone()].join(".")));

    let choice_elements = model
        .as_ref()
        .and_then(|model_details| model_details.choice_type_paths.get(&choice_path?).clone())
        .and_then(|choice_items| {
            Some(
                choice_items
                    .into_iter()
                    .map(|ci| vec![property.to_string(), capitalise(ci)].join(""))
                    .collect::<Vec<String>>(),
            )
        });

    match choice_elements {
        Some(ce) => ce.to_vec(),
        None => vec![property.clone()],
    }
}

fn expand_primitive_values(keys: Vec<String>) -> Vec<String> {
    keys.into_iter().fold(vec![], |mut acc, item| {
        let mut expanded = vec![item.clone()];

        expanded.push(format!("_{}", &item));
        acc.append(&mut expanded);
        acc
    })
}

impl Evaluate for MemberInvocation {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> EvaluateResult<ResourceNode<'a, 'b>> {
        if input.is_empty()? {
            return Ok(ResourceNode::from_node(input, json!([])));
        }

        let key_node = self
            .children
            .first()
            .ok_or(FhirpathError::EvaluateError {
                msg: "MemberInvocation has no child node".to_string(),
            })?
            .evaluate(input)?;

        if !key_node.is_single()? {
            return Err(FhirpathError::EvaluateError {
                msg: "Could not determine property to invoke".to_string(),
            });
        }

        let key = key_node.get_single()?;

        let key_value = match key {
            Value::String(str) => str,
            _ => "".to_string(),
        };

        // check if we're already invoking the extensible property
        // of a primitive FHIR value
        let is_extensible_key = key_value.starts_with("_");

        let input_data = input.get_array()?;

        let node_resource_type = input_data.first().and_then(|item| item.get("resourceType"));

        // MemberInvocation is resourceType, so return whole resource
        if node_resource_type.is_some_and(|resource_type| resource_type.eq(&key_value)) {
            let resource_data = Value::Array(input_data.clone());

            let path_details = determine_fhir_type(
                input_data.first(),
                Some(Path {
                    path: key_value.clone(),
                    child_property: None
                }),
                input.context,
                false
            );

            let mut node = ResourceNode::from_node(input, resource_data);

            if let Some(pd) = path_details {
                node.path = Some(pd.path.clone());

                node.fhir_types = vec![Some(pd)];
            }

            return Ok(node);
        }

        // if the path leads to a choice (e.g. value[x]) expand the
        // choice properties
        let choice_values = expand_choice_values(input, &key_value);

        // also expand primitive values
        let expanded = expand_primitive_values(choice_values);

        // Else look for a child property of the resource that matches the key
        let keys_values: Vec<(String, Vec<Value>)> =
            expanded.iter().fold(vec![], |mut acc, prop| {
                let prop_vals: Vec<Value> = input_data
                    .to_owned()
                    .into_iter()
                    .filter_map(|item| item.get(&prop.to_string()).cloned())
                    .collect();

                if prop_vals.len() != 0 {
                    let flat_vals = prop_vals.into_iter().fold(vec![], |mut acc, item| {
                        match item {
                            Value::Array(mut arr) => acc.append(&mut arr),
                            _ => acc.push(item.clone()),
                        }
                        acc
                    });

                    acc.push((prop.to_string(), flat_vals));
                }

                acc
            });

        let keys: Vec<String> = keys_values.iter().map(|kv| kv.0.clone()).collect();
        let values: Vec<Vec<Value>> = keys_values.iter().map(|kv| kv.1.clone()).collect();

        let flattened_values: Vec<Value> = values.iter().fold(vec![], |mut acc, val| {
            acc.append(&mut val.clone());
            acc
        });

        let path_details = match &input.path {
            Some(path) => {
                let path_info = Some(Path {
                    path: path.clone(),
                    child_property: Some(key_value.clone()),
                });

                determine_fhir_type(
                    flattened_values.first(),
                    path_info,
                    input.context,
                    is_extensible_key
                )
            },
            None => None,
        };

        let mut node = ResourceNode::from_node(input, Value::Array(flattened_values));

        if let Some(pd) = path_details {
            node.path = Some(pd.path);
        }

        let type_details: Vec<Option<PathDetails>> = keys
            .iter()
            .map(|key| {
                let path = Some(Path {
                    path: input.path.clone()?,
                    child_property: Some(key.clone()),
                });

                determine_fhir_type(None, path, input.context, is_extensible_key)
            })
            .collect();

        let mut fhir_types = vec![];

        for (pos, detail) in type_details.into_iter().enumerate() {
            fhir_types.append(&mut vec![detail; values[pos].len()]);
        }

        node.fhir_types = fhir_types;

        Ok(node)
    }
}

impl Text for MemberInvocation {
    fn text(&self) -> EvaluateResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<EvaluateResult<Vec<String>>>()?
            .join("."))
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::evaluate::{
        test::test::{run_tests, TestCase},
    };
    use crate::evaluate::test::test::Expected;

    #[test]
    fn evaluate_extensible_path() {
        let patient = json!({
          "resourceType": "Patient",
          "id": "example",
          "gender": "male",
          "birthDate": "2022",
          "_birthDate": {
            "extension": [
              {
                "url": "http://hl7.org/fhir/StructureDefinition/patient-birthTime",
                "valueDateTime": "1974-12-25T14:35:45-05:00"
              }
            ]
          }
        });

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                path: "Patient.birthDate".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!(["2022"])),
            },
            TestCase {
                path: "Patient.birthDate.extension".to_string(),
                input: patient.clone(),
                options: None,
                expected: Expected::Value(json!([{
                    "url": "http://hl7.org/fhir/StructureDefinition/patient-birthTime",
                    "valueDateTime": "1974-12-25T14:35:45-05:00"
                  }])),
            },
            TestCase {
                path: "Patient._birthDate".to_string(),
                input: patient.clone(),
                expected: Expected::Value(json!([{
                    "extension": [{
                    "url": "http://hl7.org/fhir/StructureDefinition/patient-birthTime",
                    "valueDateTime": "1974-12-25T14:35:45-05:00"
                  }]}])),
                options: None,
            },
        ];

        run_tests(test_cases);
    }
}