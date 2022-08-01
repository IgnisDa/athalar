mod profile;
mod typescript;

pub use self::profile::AthalarProfile;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The different languages in which bindings are available.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AthalarBindingLanguage {
    Typescript,
}

/// A binding is a set of configuration for a specific language.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AthalarBinding<T: AthalarProfile> {
    /// The language in which the bindings are to be generated
    pub language: AthalarBindingLanguage,

    // The user will declare this path to be relative to `athalar.toml` but we will fully
    // qualify that path while constructing this.
    /// The fully qualified path where this binding output should be placed
    pub output: PathBuf,

    /// The configuration to this specific binding
    pub profile: T,
}

#[cfg(test)]
mod test {}
