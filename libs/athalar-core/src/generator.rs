use crate::{
    binding::{AthalarBinding, AthalarBindingBuilder},
    utils::get_name_from_path,
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_yaml::{value::TaggedValue, Sequence};
use std::{collections::HashMap, path::PathBuf};
use uuid::Uuid;

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
    /// A unique ID assigned to this atom, should be used as an identifier
    #[builder(setter(skip), default = "Uuid::new_v4()")]
    #[builder_field_attr(serde(skip))]
    pub(crate) id: Uuid,

    /// Information about which bindings need to be generated
    #[builder(setter(into, strip_option), default)]
    pub bindings: Vec<AthalarBinding>,

    /// The actual data in the file
    #[builder(setter(into, strip_option), default)]
    pub config: Vec<AthalarGeneratorContent>,
}

impl AthalarGeneratorData {
    // This is a very bad implementation and likely to break if we change
    // the schema. However it is being blocked by
    // https://github.com/dtolnay/serde-yaml/issues/302.
    pub fn partial_from_yaml_string(yaml_string: &str) -> Self {
        #[derive(Serialize, Deserialize)]
        struct _A {
            pub config: Vec<AthalarGeneratorContent>,
        }
        #[derive(Serialize, Deserialize, Debug)]
        struct _B {
            profile: TaggedValue,
        }
        #[derive(Serialize, Deserialize, Debug)]
        struct _C {
            pub bindings: Vec<_B>,
        }
        let c = serde_yaml::from_str::<_C>(yaml_string).unwrap();
        let contents = serde_yaml::from_str::<HashMap<String, Sequence>>(yaml_string).unwrap();
        let bindings = contents
            .get("bindings")
            .into_iter()
            .zip(c.bindings)
            .flat_map(|(b1, b2)| {
                let tag_name = if b2.profile.tag == "!ClassValidator" {
                    " !ClassValidator"
                } else {
                    " !NotPossible"
                };
                b1.iter().map(|b| {
                    let mut new_str = serde_yaml::to_string(b).unwrap();
                    let insert_after = "profile:";
                    let pos = new_str.find(insert_after).unwrap();
                    new_str.insert_str(pos + insert_after.len(), tag_name);
                    serde_yaml::from_str::<AthalarBindingBuilder>(&new_str)
                        .unwrap()
                        .build()
                        .unwrap()
                })
            })
            .collect::<Vec<_>>();
        let _t = serde_yaml::from_str::<_A>(yaml_string).unwrap();
        AthalarGeneratorDataBuilder::default()
            .bindings(bindings)
            .config(_t.config)
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
