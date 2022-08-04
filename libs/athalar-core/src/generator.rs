use crate::{binding::AthalarBinding, utils::get_name_from_path};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Contains information about a discovered generator in the project.
#[derive(Debug, PartialEq, Builder, Clone)]
pub struct AthalarGenerator {
    /// The name of this generator, based on the file name. Can be considered to be it's
    /// unique identifier.
    #[builder(default = "self.get_name()?")]
    pub name: String,

    /// The path to this partial relative to the current directory
    pub source: PathBuf,

    /// The actual data that is in this generator file
    pub data: AthalarGeneratorData,
}

impl AthalarGenerator {
    /// The directory in which this partial will be found, relative to partial directory
    pub fn source(&self) -> &PathBuf {
        &self.source
    }
}

impl AthalarGeneratorBuilder {
    fn get_name(&self) -> Result<String, String> {
        let source = get_name_from_path(&self.source.clone().unwrap());
        Ok(source)
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
    pub bindings: Vec<AthalarBinding>,

    /// The actual data in the file
    #[builder(setter(into, strip_option), default)]
    pub config: Vec<AthalarGeneratorContent>,
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
