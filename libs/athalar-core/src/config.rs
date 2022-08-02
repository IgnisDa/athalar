use derive_builder::Builder;
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AthalarConfigVersion {
    One = 1,
}

/// The container for configuring the Athalar instance.
#[derive(Debug, PartialEq, Builder, Clone)]
pub struct AthalarConfig {
    version: AthalarConfigVersion,

    source: PathBuf,

    #[builder(default = "PathBuf::from(\"partials\")")]
    pub partials: PathBuf,

    #[builder(default = "PathBuf::from(\"generators\")")]
    pub generators: PathBuf,
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
    #[should_panic]
    fn panic_on_no_version() {
        AthalarConfigBuilder::default().build().unwrap();
    }

    #[test]
    #[should_panic]
    fn panic_on_no_source() {
        AthalarConfigBuilder::default()
            .version(AthalarConfigVersion::One)
            .build()
            .unwrap();
    }
}
