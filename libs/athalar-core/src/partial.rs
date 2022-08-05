use crate::{
    atom::{AthalarAtom, AthalarAtomBuilder},
    config::AthalarConfigKind,
    utils::get_name_from_path,
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Contains information about a discovered partial in the project.
#[derive(Debug, PartialEq, Builder, Clone)]
pub struct AthalarPartial {
    /// A unique ID assigned to this atom, should be used as an identifier
    #[builder(setter(skip), default = "Uuid::new_v4()")]
    pub(crate) id: Uuid,

    /// The name of this partial, based on the file name.
    #[builder(default = "self.get_name()?")]
    pub name: String,

    /// The path to this partial relative to the current directory
    source: PathBuf,

    /// The actual data that is in this generator file
    pub data: AthalarPartialData,
}

impl AthalarPartialBuilder {
    fn get_name(&self) -> Result<String, String> {
        let source = get_name_from_path(&self.source.clone().unwrap());
        Ok(source)
    }
}

#[derive(Debug, PartialEq, Builder, Clone)]
#[builder(derive(Debug, Serialize, Deserialize))]
pub struct AthalarPartialData {
    /// The type of partial
    #[builder(default = "AthalarConfigKind::Variable")]
    pub kind: AthalarConfigKind,

    /// The actual data in the file
    #[builder(field(
        type = "Vec<AthalarAtomBuilder>",
        build = "self.config.iter().map(|c| c.build().unwrap()).collect()"
    ))]
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
            .config(vec![AthalarAtomBuilder::default().name("atom").clone()])
            .build()
            .unwrap();
        assert_eq!(apd.config.len(), 1);
        assert_eq!(apd.config[0].validators.len(), 0);
    }

    #[test]
    fn specifying_kind_as_variable_sets_correct_value() {
        let apd = AthalarPartialDataBuilder::default()
            .config(vec![AthalarAtomBuilder::default().name("atom").clone()])
            .build()
            .unwrap();
        assert_eq!(apd.kind, AthalarConfigKind::Variable);
    }
}
