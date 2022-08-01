use super::profile::AthalarProfile;
use serde::{Deserialize, Serialize};

/// Each typescript adapter must implement this trait.
pub trait TypescriptAdapterProfile {}

/// The different adapter that can be used to generate different schemas.
#[derive(Debug, Serialize, Deserialize)]
pub enum TypescriptAdapter {
    ClassValidator,
}

/// Settings that are specific to the class validator adapter
struct ClassValidatorAdapterProfile {
    /// The name of the class generated
    class_name: Option<String>,
}

/// The settings specific to the typescript bindings.
#[derive(Debug, Serialize, Deserialize)]
pub struct TypescriptProfile<T: TypescriptAdapterProfile> {
    /// The adapter for this particular binding
    adapter: TypescriptAdapter,

    /// Settings that are specific to this typescript adapter
    #[serde(flatten)]
    profile: T,
}

impl AthalarProfile for TypescriptProfile<ClassValidatorAdapterProfile> {}
impl TypescriptAdapterProfile for ClassValidatorAdapterProfile {}
