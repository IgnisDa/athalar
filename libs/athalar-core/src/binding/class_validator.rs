use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Settings that are specific to the class validator adapter
#[derive(Debug, PartialEq, Clone, Builder, Serialize, Deserialize)]
pub struct ClassValidatorAdapterProfile {
    /// The name of the class generated
    #[builder(setter(into, strip_option), default)]
    class_name: Option<String>,
}
