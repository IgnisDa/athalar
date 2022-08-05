use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum AtomValidator {
    Number,
    String,
    Url,
    Port,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum AtomKind {
    Number,
    String,
}

#[derive(Debug, PartialEq, Builder, Clone, Default, Serialize, Deserialize)]
#[builder(derive(Debug, Serialize, Deserialize))]
pub struct AthalarAtom {
    /// A unique ID assigned to this atom, should be used as an identifier
    #[builder(setter(skip), default = "Uuid::new_v4()")]
    #[builder_field_attr(serde(skip))]
    pub(crate) id: Uuid,

    /// The name of this configuration variable
    pub name: String,

    #[builder(setter(into, strip_option), default = "None")]
    kind: Option<AtomKind>,

    /// The validators that should be applied to this configuration variable
    #[builder(setter(into, strip_option), default)]
    #[serde(default)]
    pub validators: Vec<AtomValidator>,

    /// An optional description that will get included in the generated code
    #[builder(setter(into), default)]
    description: Option<String>,
}

impl AthalarAtom {
    // determine the `kind` either by using the supplied kind, or going through the validators
    pub fn kind(&self) -> AtomKind {
        match &self.kind {
            Some(x) => *x,
            None => {
                // iterate through the validators and fallback to String if none is found
                AtomKind::String
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_validators_in_yaml_yields_validator_of_zero_len() {
        let aca = AthalarAtomBuilder::default()
            .name("mail".into())
            .build()
            .unwrap();
        assert_eq!(aca.validators.len(), 0);
    }

    #[test]
    fn correct_number_of_validators() {
        let aca = AthalarAtomBuilder::default()
            .name("mail".into())
            .validators(vec![AtomValidator::String])
            .build()
            .unwrap();
        assert_eq!(aca.validators.len(), 1);
    }
}
