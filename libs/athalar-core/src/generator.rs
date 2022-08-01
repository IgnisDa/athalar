use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, PartialEq)]
enum AthalarGeneratorError {
    ParseError,
}

/// Contains information about a discovered generator in the project.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AthalarGenerator {
    /// The path to this partial relative to the current directory
    #[serde(default)]
    source: Option<PathBuf>,
}

impl AthalarGenerator {
    /// The directory in which this partial will be found, relative to partial directory
    pub fn source(&self) -> &PathBuf {
        self.source.as_ref().unwrap()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum AthalarGeneratorContent {
    IncludePartial(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct AthalarGeneratorData {
    /// The actual data in the file
    config: Vec<AthalarGeneratorContent>,
}

impl FromStr for AthalarGeneratorData {
    type Err = AthalarGeneratorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let apd = serde_yaml::from_str::<Self>(s).map_err(|_| AthalarGeneratorError::ParseError)?;
        Ok(apd)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_config_should_have_correct_length() {
        let s = "config: []";
        let agd = AthalarGeneratorData::from_str(s).unwrap();
        assert_eq!(agd.config.len(), 0);
    }

    #[test]
    fn config_should_have_correct_length() {
        let s = r#"
        config:
          - !IncludePartial mail
        "#;
        let agd = AthalarGeneratorData::from_str(s).unwrap();
        assert_eq!(agd.config.len(), 1);
    }

    #[test]
    fn config_should_have_correct_value_inside_include_partial() {
        let s = r#"
        config:
          - !IncludePartial mail
        "#;
        let agd = AthalarGeneratorData::from_str(s).unwrap();
        match agd.config.get(0).unwrap() {
            AthalarGeneratorContent::IncludePartial(x) => assert_eq!(x, "mail"),
        }
    }

    #[test]
    fn no_config_key_should_return_parse_error() {
        let s = "";
        let agd = AthalarGeneratorData::from_str(s);
        assert_eq!(agd, Err(AthalarGeneratorError::ParseError));
    }
}
