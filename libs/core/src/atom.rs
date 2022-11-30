use crate::utils::get_uuid;
use derive_builder::Builder;
use hashbag::HashBag;
use serde::{Deserialize, Serialize};
use std::iter::FromIterator;
use strum_macros::Display;
use uuid::Uuid;

// Note: It is the job of the client libraries to actually perform the validation and apply
// the correct modification to the code (for example decorators for class-validator
// bindings).
/// The different validators that can be applied to the different configuration variables
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Display)]
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
#[derive(Debug, Default, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq)]
pub enum AtomKind {
    /// Represents a number
    Number,

    /// Represents a string
    #[default]
    String,

    /// Can represent any datatype, most probably because the kind could not be derived
    Any,
}

impl From<AtomValidator> for AtomKind {
    fn from(av: AtomValidator) -> Self {
        match av {
            AtomValidator::Noop => AtomKind::Any,
            AtomValidator::Number => AtomKind::Number,
            AtomValidator::Port => AtomKind::Number,
            AtomValidator::String => AtomKind::String,
            AtomValidator::Url => AtomKind::String,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Builder, Clone, Default, Serialize, Deserialize)]
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
            // the user has not specified one
            None => {
                let counts: HashBag<AtomKind> = HashBag::from_iter(
                    self.validators
                        .clone()
                        .unwrap()
                        .into_iter()
                        .map(AtomKind::from),
                );
                let max = counts.set_iter().max_by_key(|x| x.1).unwrap();
                Ok(*max.0)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
