use std::collections::HashMap;

mod stu3;

#[derive(Clone)]
pub enum ModelType {
    Stu3,
    R4,
    R5,
}

#[derive(Clone)]
pub struct ModelDetails {
    pub model_type: ModelType,
    pub choice_type_paths: HashMap<String, Vec<String>>,
    pub path_to_type: HashMap<String, String>,
    pub paths_defined_elsewhere: HashMap<String, String>,
    pub type_to_parent: HashMap<String, String>,
}

#[derive(Debug)]
pub enum ModelError {
    FileLoadFail { msg: String },
    ParseFail { msg: String },
}

type ModelResult<T> = Result<T, ModelError>;

pub fn get_model_details(model_type: ModelType) -> ModelResult<ModelDetails> {
    match model_type {
        ModelType::Stu3 => Ok(ModelDetails {
            model_type,
            choice_type_paths: stu3::choice_type_paths::choice_type_paths(),
            path_to_type: stu3::path_to_type::path_to_type(),
            paths_defined_elsewhere: stu3::paths_defined_elsewhere::paths_defined_elsewhere(),
            type_to_parent: stu3::type_to_parent::type_to_parent(),
        }),
        _ => todo!(),
    }
}
