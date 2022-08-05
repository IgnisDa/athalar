use crate::{
    binding::{AthalarBinding, AthalarBindingBuilder},
    utils::get_name_from_path,
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Contains information about a discovered generator in the project.
#[derive(Debug, PartialEq, Builder, Clone, Eq)]
pub struct AthalarGenerator {
    /// A unique ID assigned to this atom, should be used as an identifier
    #[builder(setter(skip), default = "Uuid::new_v4()")]
    pub(crate) id: Uuid,

    /// The name of this generator, based on the file name.
    #[builder(setter(into), default = "self.get_name()?")]
    pub name: String,

    /// The path to this partial relative to the current directory
    pub source: PathBuf,

    /// The actual data that is in this generator file
    pub data: AthalarGeneratorData,
}

impl AthalarGeneratorBuilder {
    fn get_name(&self) -> Result<String, String> {
        let source = get_name_from_path(&self.source.clone().unwrap());
        Ok(source)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq)]
pub enum AthalarGeneratorContent {
    IncludePartial(String),
}

#[derive(Debug, PartialEq, Builder, Clone, Eq)]
#[builder(derive(Debug, Serialize, Deserialize))]
pub struct AthalarGeneratorData {
    /// Information about which bindings need to be generated
    #[builder(field(
        type = "Vec<AthalarBindingBuilder>",
        build = "self.bindings.iter().map(|b| b.build().unwrap()).collect()"
    ))]
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
                .clone()])
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
