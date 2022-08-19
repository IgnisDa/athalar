use crate::utils::get_uuid;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use uuid::Uuid;

// Note: It is the job of the client libraries to actually perform the validation and apply
// the correct modification to the code (for example decorators for class-validator
// bindings).
/// The different validators that can be applied to the different configuration variables
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Display)]
pub enum AtomValidator {
    /// Checks if the value is a number.
    Number,
    /// Checks if the string is a string.
    String,
    /// Checks if the string is an URL.
    Url,
    /// Checks if the string is a valid port number.
    Port,
    /// A validator that lets all values pass through it unmodified. It can be used when
    /// the validators is left empty.
    Noop,
}

/// The type that the configuration variable will have based on it's different properties.
/// It can either be user defined otr be inferred from the validators applied.
#[derive(Debug, Default, PartialEq, Clone, Copy, Serialize, Deserialize, Display)]
pub enum AtomKind {
    Number,
    #[default]
    String,
    Any,
}

#[derive(Debug, PartialEq, Builder, Clone, Default, Serialize, Deserialize)]
#[builder(derive(Debug, Serialize, Deserialize))]
pub struct AthalarAtom {
    /// A unique ID assigned to this atom, should be used as an identifier
    #[builder(field(type = "Uuid"))]
    #[builder_field_attr(serde(default = "get_uuid"))]
    pub(crate) id: Uuid,

    /// The name of this configuration variable
    #[builder(setter(into))]
    pub name: String,

    // the final value of this will be set right in the build step
    #[builder(field(type = "Option<AtomKind>", build = "self.get_kind()?"))]
    pub kind: AtomKind,

    /// The validators that should be applied to this configuration variable
    #[builder(setter(into, strip_option), default)]
    #[serde(default)]
    pub validators: Vec<AtomValidator>,

    /// An optional description that will get included in the generated code
    #[builder(setter(into), default)]
    pub description: Option<String>,
}

impl AthalarAtomBuilder {
    // determine and set the `kind` either by using the supplied kind, or going through the
    // validators
    fn get_kind(&self) -> Result<AtomKind, String> {
        match self.kind {
            Some(x) => Ok(x),
            None => {
                // TODO: Iterate over the validators and determine the correct `kind` since
                // the user has not specified one
                Ok(AtomKind::String)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_validators_in_yaml_yields_validator_of_zero_len() {
        let aca = AthalarAtomBuilder::default().name("mail").build().unwrap();
        assert_eq!(aca.validators.len(), 0);
    }

    #[test]
    fn correct_number_of_validators() {
        let aca = AthalarAtomBuilder::default()
            .name("mail")
            .validators(vec![AtomValidator::String])
            .build()
            .unwrap();
        assert_eq!(aca.validators.len(), 1);
    }
}
