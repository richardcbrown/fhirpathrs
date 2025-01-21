use std::{collections::HashMap, fs};

use serde::de::DeserializeOwned;
use serde_json::from_str;

pub enum ModelType {
    Stu3,
    R4,
    R5,
}

impl ModelType {
    fn get_dir(&self) -> &str {
        match self {
            ModelType::Stu3 => "stu3",
            ModelType::R4 => "r4",
            ModelType::R5 => "r5",
        }
    }
}

pub struct ModelDetails {
    pub model_type: ModelType,
    pub choice_type_paths: HashMap<String, Vec<String>>,
    pub path_to_type: HashMap<String, String>,
    pub paths_defined_elsewhere: HashMap<String, String>,
    pub type_to_parent: HashMap<String, String>,
}

pub enum ModelError {
    FileLoadFail { msg: String },
    ParseFail { msg: String },
}

type ModelResult<T> = Result<T, ModelError>;

fn parse_file<T>(path: &str, file: &str) -> ModelResult<T>
where
    T: DeserializeOwned,
{
    let file_path = format!("./{}/{}", path, file);

    fs::read_to_string(&file_path)
        .map_err(|err| ModelError::FileLoadFail {
            msg: format!("Failed to load {}: {}", file_path, err.to_string()),
        })
        .and_then(|file| {
            from_str::<T>(&file).map_err(|err| ModelError::ParseFail {
                msg: format!("Failed to parse {}: {}", file_path, err.to_string()),
            })
        })
}

pub fn get_model_details(model_type: ModelType) -> ModelResult<ModelDetails> {
    let model_dir = model_type.get_dir();

    let choice_type_paths =
        parse_file::<HashMap<String, Vec<String>>>(model_dir, "choice_type_paths.json")?;
    let path_to_type = parse_file::<HashMap<String, String>>(model_dir, "path_to_type.json")?;
    let paths_defined_elsewhere =
        parse_file::<HashMap<String, String>>(model_dir, "paths_defined_elsewhere.json")?;
    let type_to_parent = parse_file::<HashMap<String, String>>(model_dir, "type_to_parent.json")?;

    Ok(ModelDetails {
        model_type,
        choice_type_paths,
        path_to_type,
        paths_defined_elsewhere,
        type_to_parent,
    })
}
