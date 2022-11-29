//! Common utilities that can be used to work with athalar projects.

use crate::{
    constants::ATHALAR_CONFIG_FILE,
    generator::{AthalarGenerator, AthalarGeneratorBuilder, AthalarGeneratorData},
    partial::{AthalarPartial, AthalarPartialBuilder, AthalarPartialData},
    Athalar, AthalarConfig,
};
use glob::glob;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf, MAIN_SEPARATOR},
};
use uuid::Uuid;

/// Changes path to name, eg: `src/generators/backend.ath.yaml` to `backend`.
pub fn get_name_from_path(path: &Path) -> String {
    path.to_str()
        .unwrap()
        .to_string()
        .split(MAIN_SEPARATOR)
        .last()
        .unwrap()
        .split('.')
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .to_string()
}

fn get_file_source_and_contents(dir: &Path) -> Vec<(PathBuf, String)> {
    let glob_pattern = dir
        .join("*.ath.yaml")
        .into_os_string()
        .into_string()
        .unwrap();
    glob(&glob_pattern)
        .expect("Failed to read glob pattern")
        .flatten()
        .map(|p| {
            let yaml_string = fs::read_to_string(&p).unwrap();
            (p, yaml_string)
        })
        .collect()
}

/// Load all the partials from a given path using globs
pub fn load_partials(dir: &Path) -> Vec<AthalarPartial> {
    get_file_source_and_contents(dir)
        .into_iter()
        .map(|(path, contents)| {
            let apd = AthalarPartialData::partial_from_yaml_string(&contents);
            AthalarPartialBuilder::default()
                .source(path)
                .data(apd)
                .build()
                .unwrap()
        })
        .collect()
}

/// Load all the generators from a given path using globs
pub fn load_generators(dir: &Path) -> Vec<AthalarGenerator> {
    get_file_source_and_contents(dir)
        .into_iter()
        .map(|(path, contents)| {
            let apd = AthalarGeneratorData::partial_from_yaml_string(&contents);
            AthalarGeneratorBuilder::default()
                .source(path)
                .data(apd)
                .build()
                .unwrap()
        })
        .collect()
}

pub(crate) fn get_uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn from_path(path: String) -> Result<Athalar, Box<dyn Error>> {
    let project_path = PathBuf::from(&path).join(ATHALAR_CONFIG_FILE);
    let config_file_contents = match fs::read_to_string(&project_path) {
        Ok(x) => x,
        Err(_) => {
            return Err(format!("Config file does not exist at: {:?}", &project_path).into());
        }
    };
    let config = AthalarConfig::from_str_and_source(&config_file_contents, &path).unwrap();
    let athalar = Athalar::from_config(config);
    Ok(athalar)
}
