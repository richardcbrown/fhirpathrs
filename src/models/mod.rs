use std::collections::{HashMap, HashSet};

mod stu3;
mod r4;

#[derive(Clone, Debug)]
pub enum ModelType {
    Stu3,
    R4,
    R5,
}

#[derive(Clone, Debug)]
pub struct ModelDetails {
    pub choice_type_paths: HashMap<String, Vec<String>>,
    pub path_to_type: HashMap<String, String>,
    pub paths_defined_elsewhere: HashMap<String, String>,
    pub type_to_parent: HashMap<String, String>,
    pub available_types: HashSet<String>,
    pub path_cardinality: HashMap<String, String>,
}

#[derive(Debug)]
pub enum ModelError {
    FileLoadFail { msg: String },
    ParseFail { msg: String },
}

type ModelResult<T> = Result<T, ModelError>;

pub fn get_model_details(model_type: ModelType) -> ModelResult<ModelDetails> {
    match model_type {
        ModelType::Stu3 => {
            let type_to_parent = stu3::type_to_parent::type_to_parent();
            let mut available_types: HashSet<String> = HashSet::new();

            type_to_parent.iter().for_each(|(key, value)| {
                available_types.insert(key.to_string());
                available_types.insert(value.to_string());
            });

            Ok(ModelDetails {
                choice_type_paths: stu3::choice_type_paths::choice_type_paths(),
                path_to_type: stu3::path_to_type::path_to_type(),
                paths_defined_elsewhere: stu3::paths_defined_elsewhere::paths_defined_elsewhere(),
                path_cardinality: HashMap::new(),
                type_to_parent,
                available_types,
            })
        },
        ModelType::R4 => {
            let type_to_parent = r4::type_to_parent::type_to_parent();
            let mut available_types: HashSet<String> = HashSet::new();

            type_to_parent.iter().for_each(|(key, value)| {
                available_types.insert(key.to_string());
                available_types.insert(value.to_string());
            });

            Ok(ModelDetails {
                choice_type_paths: r4::choice_type_paths::choice_type_paths(),
                path_to_type: r4::path_to_type::path_to_type(),
                paths_defined_elsewhere: r4::paths_defined_elsewhere::paths_defined_elsewhere(),
                path_cardinality: r4::path_cardinality::path_cardinality(),
                type_to_parent,
                available_types,
            })
        }
        _ => todo!(),
    }
}
