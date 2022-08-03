use crate::binding::AthalarBinding;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Contains information about a discovered generator in the project.
#[derive(Debug, PartialEq, Builder, Clone)]
pub struct AthalarGenerator {
    /// The path to this partial relative to the current directory
    source: PathBuf,

    /// The actual data that is in this generator file
    data: AthalarGeneratorData,
}

impl AthalarGenerator {
    /// The directory in which this partial will be found, relative to partial directory
    pub fn source(&self) -> &PathBuf {
        &self.source
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum AthalarGeneratorContent {
    IncludePartial(String),
}

#[derive(Debug, PartialEq, Builder, Clone)]
#[builder(derive(Debug, Serialize, Deserialize))]
pub struct AthalarGeneratorData {
    /// Information about which bindings need to be generated
    #[builder(setter(into, strip_option), default)]
    bindings: Vec<AthalarBinding>,

    /// The actual data in the file
    #[builder(setter(into, strip_option), default)]
    config: Vec<AthalarGeneratorContent>,
}

impl AthalarGeneratorData {
    pub fn partial_from_yaml_string(yaml_string: &str) -> Self {
        serde_yaml::from_str::<AthalarGeneratorDataBuilder>(yaml_string)
            .unwrap()
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::binding::{
        AthalarAdapter, AthalarBindingBuilder, ClassValidatorAdapterProfileBuilder,
    };

    #[test]
    fn empty_config_should_have_correct_length() {
        let agd = AthalarGeneratorDataBuilder::default().build().unwrap();
        assert_eq!(agd.config.len(), 0);
    }

    #[test]
    fn empty_bindings_should_have_correct_length() {
        let agd = AthalarGeneratorDataBuilder::default().build().unwrap();
        assert_eq!(agd.bindings.len(), 0);
    }

    #[test]
    fn config_should_have_correct_length() {
        let agd = AthalarGeneratorDataBuilder::default()
            .config(vec![AthalarGeneratorContent::IncludePartial("mail".into())])
            .build()
            .unwrap();
        assert_eq!(agd.config.len(), 1);
    }

    #[test]
    fn bindings_should_have_correct_length() {
        let agd = AthalarGeneratorDataBuilder::default()
            .bindings(vec![AthalarBindingBuilder::default()
                .output(PathBuf::from("some"))
                .profile(AthalarAdapter::ClassValidator(
                    ClassValidatorAdapterProfileBuilder::default()
                        .build()
                        .unwrap(),
                ))
                .build()
                .unwrap()])
            .build()
            .unwrap();
        assert_eq!(agd.bindings.len(), 1);
    }

    #[test]
    fn config_should_have_correct_value_inside_include_partial() {
        let agd = AthalarGeneratorDataBuilder::default()
            .config(vec![AthalarGeneratorContent::IncludePartial("mail".into())])
            .build()
            .unwrap();
        match agd.config.get(0).unwrap() {
            AthalarGeneratorContent::IncludePartial(x) => assert_eq!(x, "mail"),
        }
    }
}
