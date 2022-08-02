mod class_validator;

use class_validator::ClassValidatorAdapterProfile;
pub use class_validator::ClassValidatorAdapterProfileBuilder;
use derive_builder::Builder;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone)]
pub enum AthalarAdapter {
    ClassValidator(ClassValidatorAdapterProfile),
}

/// A binding is a set of configuration for a specific language.
#[derive(Debug, PartialEq, Clone, Builder)]
pub struct AthalarBinding {
    // The user will declare this path to be relative to `athalar.toml` but we will fully
    // qualify that path while constructing this.
    /// The fully qualified path where this binding output should be placed
    pub output: PathBuf,

    /// The profile to use for this adapter
    pub profile: AthalarAdapter,
}
