use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Settings that are specific to the class validator adapter
#[derive(Debug, PartialEq, Clone, Builder, Serialize, Deserialize, Eq)]
pub struct ClassValidatorAdapterProfile {
    /// The name of the class generated
    #[builder(setter(into, strip_option), default)]
    pub class_name: Option<String>,
}

/// Settings that are specific to the pydantic adapter
#[derive(Debug, PartialEq, Clone, Builder, Serialize, Deserialize, Eq)]
pub struct PydanticAdapterProfile {
    /// The name of the class generated
    #[builder(setter(into, strip_option), default)]
    pub class_name: Option<String>,
}
