use derive_builder::Builder;
use serde::{Deserialize, Serialize};

// TODO: Move this file to the python crate
/// Settings that are specific to the pydantic adapter
#[derive(Debug, PartialEq, Clone, Builder, Serialize, Deserialize, Eq)]
pub struct PydanticAdapterProfile {
    /// The name of the class generated
    #[builder(setter(into, strip_option), default)]
    pub class_name: Option<String>,
}
