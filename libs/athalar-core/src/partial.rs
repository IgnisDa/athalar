use crate::{config_atom::AthalarConfigAtom, AthalarConfigKind};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};

#[derive(Debug)]
enum AthalarPartialError {
    ParseError,
    /// thrown when `kind` is other than `Variable`
    KindError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AthalarPartial {
    /// The path to this partial relative to the current directory
    #[serde(default)]
    source: Option<PathBuf>,
}

impl AthalarPartial {
    /// The directory in which this partial will be found, relative to partial directory
    pub fn source(&self) -> &PathBuf {
        self.source.as_ref().unwrap()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct AthalarPartialData {
    /// The type of partial
    #[serde(default = "AthalarConfigKind::variable")]
    kind: AthalarConfigKind,
    /// The actual data in the file
    config: Vec<AthalarConfigAtom>,
}

impl FromStr for AthalarPartialData {
    type Err = AthalarPartialError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let apd = serde_yaml::from_str::<Self>(s).map_err(|_| AthalarPartialError::ParseError)?;
        if matches!(apd.kind, AthalarConfigKind::Generator) {
            return Err(AthalarPartialError::KindError);
        }
        Ok(apd)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_kind_gets_default_value() {
        let s = r#"
        config:
          - name: MAIL_PORT
            validators:
              - !Port
        "#;
        let apd = AthalarPartialData::from_str(s).unwrap();
        assert_eq!(apd.kind, AthalarConfigKind::Variable);
    }

    #[test]
    fn specifying_kind_as_variable_sets_correct_value() {
        let s = r#"
        kind: !Variable
        config:
          - name: MAIL_PORT
        "#;
        let apd = AthalarPartialData::from_str(s).unwrap();
        assert_eq!(apd.kind, AthalarConfigKind::Variable);
    }

    #[test]
    #[should_panic]
    fn specifying_generator_kind_throws_error() {
        let s = r#"
        kind: !Generator
        config:
          - name: MAIL_PORT
            validators:
              - !Port
        "#;
        AthalarPartialData::from_str(s).unwrap();
    }
}
