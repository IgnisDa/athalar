//! Common utilities that can be used to work with athalar projects.

use crate::{
    config::{AthalarConfig, AthalarConfigBuilder},
    constants::ATHALAR_CONFIG_FILE,
    generator::{AthalarGenerator, AthalarGeneratorBuilder, AthalarGeneratorData},
    partial::{AthalarPartial, AthalarPartialBuilder, AthalarPartialData},
};
use glob::glob;
use std::{
    fs,
    path::{Path, PathBuf},
};

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

/// Load the athalar configuration from the given path
pub fn load_config(dir: &Path) -> AthalarConfig {
    let config_path = dir.join(ATHALAR_CONFIG_FILE);
    let s = fs::read_to_string(config_path).unwrap();
    let acb = toml::from_str::<AthalarConfigBuilder>(&s).unwrap();
    acb.build().unwrap()
}
