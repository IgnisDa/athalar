use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};
use validator::{Validate, ValidationError, ValidationErrors};

fn get_error_message(err: &&Vec<ValidationError>) -> String {
    err.get(0)
        .unwrap()
        .message
        .clone()
        .map(|p| p.to_string())
        .unwrap()
}

fn get_version_and_source_errors(errs: ValidationErrors) -> (Option<String>, Option<String>) {
    let field_errors = errs.field_errors();
    let version = field_errors.get("version").map(get_error_message);
    let source = field_errors.get("source").map(get_error_message);
    (version, source)
}

/// The container for validation errors when creating [AthalarConfig]
#[derive(Debug, PartialEq)]
pub struct AthalarConfigValidationError {
    /// version related errors
    version: Option<String>,
    /// source related errors
    source: Option<String>,
}

/// The different errors raised when creating [AthalarConfig]
#[derive(Debug, PartialEq)]
pub enum AthalarConfigError {
    ParseError,
    ValidationError(AthalarConfigValidationError),
}

/// The container for configuring the Athalar instance
#[derive(Serialize, Debug, Deserialize, PartialEq, Validate)]
pub struct AthalarConfig {
    #[validate(range(min = 1, max = 1, message = "Version can only be 1"))]
    version: u8,
    source: Option<PathBuf>,
    partials: Option<PathBuf>,
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

    /// The directory relative to `source` where partials will be found
    pub fn partials(&self) -> &PathBuf {
        self.partials.as_ref().unwrap()
    }
}

impl FromStr for AthalarConfig {
    type Err = AthalarConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ac = match toml::from_str::<Self>(s) {
            Ok(mut x) => {
                x.source = x.source.or_else(|| Some(PathBuf::from("src")));
                x.partials = match x.partials {
                    Some(p) => Some(x.source.clone().unwrap().join(p)),
                    None => Some(x.source.clone().unwrap().join("partials")),
                };
                x
            }
            Err(_) => return Err(AthalarConfigError::ParseError),
        };
        match ac.validate() {
            Ok(_) => Ok(ac),
            Err(p) => {
                let (version, source) = get_version_and_source_errors(p);
                Err(AthalarConfigError::ValidationError(
                    AthalarConfigValidationError { version, source },
                ))
            }
        }
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
