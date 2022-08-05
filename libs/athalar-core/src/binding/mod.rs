mod class_validator;

use crate::utils::get_uuid;
use class_validator::ClassValidatorAdapterProfile;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum AthalarAdapter {
    ClassValidator(ClassValidatorAdapterProfile),
}

/// A binding is a set of configuration for a specific language.
#[derive(Debug, PartialEq, Clone, Builder, Serialize, Deserialize)]
#[builder(derive(Debug, Serialize, Deserialize))]
pub struct AthalarBinding {
    /// A unique ID assigned to this binding, should be used as an identifier
    #[builder(field(type = "Uuid"))]
    #[builder_field_attr(serde(default = "get_uuid"))]
    pub(crate) id: Uuid,

    // The user will declare this path to be relative to `athalar.toml` but we will fully
    // qualify that path while constructing this.
    /// The fully qualified path where this binding output should be placed
    pub output: PathBuf,

    /// The profile to use for this adapter
    pub profile: AthalarAdapter,
}

pub use class_validator::ClassValidatorAdapterProfileBuilder;
