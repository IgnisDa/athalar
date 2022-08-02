use derive_builder::Builder;

/// Settings that are specific to the class validator adapter
#[derive(Debug, PartialEq, Clone, Builder)]
pub struct ClassValidatorAdapterProfile {
    /// The name of the class generated
    #[builder(setter(into, strip_option), default)]
    class_name: Option<String>,
}
