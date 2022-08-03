use crate::{atom::AthalarAtom, config::AthalarConfigKind};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Contains information about a discovered partial in the project.
#[derive(Debug, PartialEq, Builder, Clone)]
pub struct AthalarPartial {
    /// The path to this partial relative to the current directory
    source: PathBuf,

    /// The actual data that is in this generator file
    data: AthalarPartialData,
}

impl AthalarPartial {
    /// The directory in which this partial will be found, relative to partial directory
    pub fn source(&self) -> &PathBuf {
        &self.source
    }
}

#[derive(Debug, PartialEq, Builder, Clone)]
#[builder(derive(Debug, Serialize, Deserialize))]
pub struct AthalarPartialData {
    /// The type of partial
    #[builder(default = "AthalarConfigKind::Variable")]
    pub kind: AthalarConfigKind,
    /// The actual data in the file
    #[builder(setter(into, strip_option), default)]
    pub config: Vec<AthalarAtom>,
}

impl AthalarPartialData {
    pub fn partial_from_yaml_string(yaml_string: &str) -> Self {
        serde_yaml::from_str::<AthalarPartialDataBuilder>(yaml_string)
            .unwrap()
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_kind_gets_default_value() {
        let apd = AthalarPartialDataBuilder::default().build().unwrap();
        assert_eq!(apd.kind, AthalarConfigKind::Variable);
    }

    #[test]
    fn no_validators() {
        let apd = AthalarPartialDataBuilder::default()
            .config(vec![AthalarAtom::default()])
            .build()
            .unwrap();
        assert_eq!(apd.config.len(), 1);
        assert_eq!(apd.config[0].validators.len(), 0);
    }

    #[test]
    fn specifying_kind_as_variable_sets_correct_value() {
        let apd = AthalarPartialDataBuilder::default()
            .config(vec![AthalarAtom::default()])
            .build()
            .unwrap();
        assert_eq!(apd.kind, AthalarConfigKind::Variable);
    }
}
