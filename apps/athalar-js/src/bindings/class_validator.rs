use athalar_core::AtomValidator;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use strum_macros::Display;

#[derive(Debug, Display)]
#[napi]
pub enum ClassValidator {
    #[strum(serialize = "IsNumber")]
    Number,

    #[strum(serialize = "IsString")]
    String,

    #[strum(serialize = "IsUrl")]
    Url,

    #[strum(serialize = "IsPort")]
    Port,

    Allow,
}

#[derive(Debug)]
#[napi(object)]
pub struct ClassValidatorProfile {
    pub class_name: String,
}

impl From<AtomValidator> for ClassValidator {
    fn from(av: AtomValidator) -> Self {
        match av {
            AtomValidator::Number => Self::Number,
            AtomValidator::String => Self::String,
            AtomValidator::Url => Self::Url,
            AtomValidator::Port => Self::Port,
            AtomValidator::Noop => Self::Allow,
        }
    }
}
