mod class_validator;
mod pydantic;

use crate::utils::get_uuid;
use class_validator::ClassValidatorAdapterProfile;
use derive_builder::Builder;
use relative_path::RelativePath;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq)]
pub enum AthalarAdapter {
    ClassValidator(ClassValidatorAdapterProfile),
    Pydantic(PydanticAdapterProfile),
}

/// A binding is a set of configuration for a specific language.
#[derive(Debug, PartialEq, Clone, Builder, Serialize, Deserialize, Eq)]
#[builder(derive(Debug, Serialize, Deserialize))]
pub struct AthalarBinding {
    /// A unique ID assigned to this binding, should be used as an identifier
    #[builder(field(type = "Uuid"))]
    #[builder_field_attr(serde(default = "get_uuid"))]
    pub(crate) id: Uuid,

    // The user will declare this path to be relative to `athalar.toml` but we will fully
    // qualify that path while constructing this.
    /// The fully qualified path where this binding output should be placed
    output: PathBuf,

    /// The profile to use for this adapter
    pub profile: AthalarAdapter,
}

impl AthalarBinding {
    /// Takes a source path (i.e. the location of the generator) and returns the logical
    /// path of where the output of the binding must be placed.
    pub fn output(&self, source: &Path) -> PathBuf {
        let b = RelativePath::from_path(&self.output).unwrap();
        b.to_logical_path(source)
    }
}

pub use class_validator::ClassValidatorAdapterProfileBuilder;
pub use pydantic::PydanticAdapterProfile;
