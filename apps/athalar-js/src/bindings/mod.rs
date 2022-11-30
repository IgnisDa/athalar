use athalar_core::AthalarAdapter;
use napi::bindgen_prelude::*;
use napi_derive::napi;

mod class_validator;

pub use class_validator::{ClassValidator, ClassValidatorProfile};

#[derive(Debug)]
#[napi]
pub enum AthalarJsBindingType {
    ClassValidator,
}

impl From<AthalarAdapter> for AthalarJsBindingType {
    fn from(aa: AthalarAdapter) -> Self {
        match aa {
            AthalarAdapter::ClassValidator(_) => Self::ClassValidator,
            AthalarAdapter::Pydantic(_) => unimplemented!(),
        }
    }
}
