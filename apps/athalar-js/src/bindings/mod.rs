use napi::bindgen_prelude::*;
use napi_derive::napi;

mod class_validator;

pub use class_validator::{ClassValidator, ClassValidatorProfile};

#[derive(Debug)]
#[napi]
pub enum AthalarJsBindingType {
    ClassValidator,
}
