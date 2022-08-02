use derive_builder::{self, Builder};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AthalarConfigKind {
    Variable,
}

impl AthalarConfigKind {
    fn variable() -> Self {
        Self::Variable
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum AthalarConfigVersion {
    #[serde(rename = "1")]
    One,
}

/// The container for configuring the Athalar instance.
#[derive(Debug, PartialEq, Builder, Clone)]
#[builder(derive(Debug, Serialize, Deserialize))]
pub struct AthalarConfig {
    version: AthalarConfigVersion,

    #[builder(default = "PathBuf::from(\"src\")")]
    source: PathBuf,

    #[builder(default = "PathBuf::from(\"partials\")")]
    partials: PathBuf,

    #[builder(default = "PathBuf::from(\"generators\")")]
    generators: PathBuf,
}

impl AthalarConfig {
    /// The directory where the partials will be located
    pub fn partials(&self) -> PathBuf {
        self.source.join(self.partials.clone())
    }

    /// The directory where the generators will be located
    pub fn generators(&self) -> PathBuf {
        self.source.join(self.generators.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gets_correct_source_and_version() {
        let ac = AthalarConfigBuilder::default()
            .version(AthalarConfigVersion::One)
            .source(PathBuf::from("other_source"))
            .build()
            .unwrap();
        assert_eq!(ac.version, AthalarConfigVersion::One);
        assert_eq!(ac.source, PathBuf::from("other_source"));
    }

    #[test]
    fn gets_correct_value_of_config_dirs() {
        let ac = AthalarConfigBuilder::default()
            .version(AthalarConfigVersion::One)
            .source(PathBuf::from("src"))
            .build()
            .unwrap();
        assert_eq!(ac.partials, PathBuf::from("partials"));
        assert_eq!(ac.generators, PathBuf::from("generators"));
    }

    #[test]
    fn accessors_return_correct_value() {
        let ac = AthalarConfigBuilder::default()
            .version(AthalarConfigVersion::One)
            .source(PathBuf::from("src"))
            .build()
            .unwrap();
        assert_eq!(ac.partials(), PathBuf::from("src").join("partials"));
        assert_eq!(ac.generators(), PathBuf::from("src").join("generators"));
    }

    #[test]
    #[should_panic]
    fn panic_on_no_version() {
        AthalarConfigBuilder::default().build().unwrap();
    }

    #[test]
    fn parses_version_from_toml() {
        let s = r#"version = "1""#;
        let acb = toml::from_str::<AthalarConfigBuilder>(s).unwrap();
        let ac = acb.build().unwrap();
        assert_eq!(ac.version, AthalarConfigVersion::One);
    }

    #[test]
    fn parses_source_from_toml() {
        let s = r#"version = "1""#;
        let acb = toml::from_str::<AthalarConfigBuilder>(s).unwrap();
        let ac = acb.build().unwrap();
        assert_eq!(ac.source, PathBuf::from("src"));
    }
}
