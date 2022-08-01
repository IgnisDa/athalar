use std::path::PathBuf;

/// The container for validation errors when creating [AthalarConfig]
#[derive(Debug, PartialEq)]
pub struct AthalarConfigValidationError {
    /// version related errors
    version: Option<String>,
    /// source related errors
    source: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum AthalarConfigVersion {
    One = 1,
}

/// The different errors raised when creating [AthalarConfig]
#[derive(Debug, PartialEq)]
pub enum AthalarConfigError {
    ParseError,
    ValidationError(AthalarConfigValidationError),
}

/// The container for configuring the Athalar instance.
#[derive(Debug, PartialEq)]
pub struct AthalarConfig {
    version: AthalarConfigVersion,
    source: Option<PathBuf>,
    partials: Option<PathBuf>,
    generators: Option<PathBuf>,
}

impl AthalarConfig {
    /// The version of schema to use for this project
    pub fn version(&self) -> u8 {
        self.version
    }

    /// The directory in which the source configuration will be found, relative to current directory
    pub fn source(&self) -> &PathBuf {
        self.source.as_ref().unwrap()
    }

    // This is a `config dir`
    /// The directory where partials will be found. It contains [Self::source] inside it.
    pub fn partials(&self) -> &PathBuf {
        self.partials.as_ref().unwrap()
    }

    // This is a `config dir`
    /// The directory where generators will be found. It contains [Self::source] inside it.
    pub fn generators(&self) -> &PathBuf {
        self.generators.as_ref().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_correct_toml() {
        let s = r#"
        version = 1
        source  = "other_source"
        "#;
        let ac = AthalarConfig::from_str(s).unwrap();
        assert_eq!(ac.version, 1);
        assert_eq!(ac.source.unwrap(), PathBuf::from("other_source"));
    }

    #[test]
    fn accessors_return_correct_values() {
        let s = r#"
        version = 1
        source  = "other_source"
        "#;
        let ac = AthalarConfig::from_str(s).unwrap();
        assert_eq!(ac.source(), &PathBuf::from("other_source"));
        assert_eq!(
            ac.partials(),
            &PathBuf::from("other_source").join("partials")
        );
    }

    #[test]
    fn gets_correct_value_of_config_dirs() {
        let s = "version = 1";
        let ac = AthalarConfig::from_str(s).unwrap();
        assert_eq!(ac.partials(), &PathBuf::from("src").join("partials"));
        assert_eq!(ac.generators(), &PathBuf::from("src").join("generators"));
    }

    #[test]
    fn gets_correct_value_of_partials() {
        let s = r#"
        version = 1
        partials  = "some_dir"
        "#;
        let ac = AthalarConfig::from_str(s);
        assert_eq!(
            ac.unwrap().partials(),
            &PathBuf::from("src").join("some_dir")
        );
    }

    #[test]
    fn gets_correct_value_of_generators() {
        let s = r#"
        version = 1
        generators  = "some_gen"
        "#;
        let ac = AthalarConfig::from_str(s);
        assert_eq!(
            ac.unwrap().generators(),
            &PathBuf::from("src").join("some_gen")
        );
    }

    #[test]
    fn errors_parse_error_on_incomplete_toml() {
        let s = "";
        let ac = AthalarConfig::from_str(s);
        assert_eq!(ac, Err(AthalarConfigError::ParseError));
    }

    #[test]
    fn validation_error_on_incorrect_version() {
        let s = r#"version = 2"#;
        let ac = AthalarConfig::from_str(s);
        assert!(matches!(
            ac,
            Err(AthalarConfigError::ValidationError { .. })
        ));
    }
}
