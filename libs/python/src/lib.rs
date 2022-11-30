mod atoms;
mod bindings;
mod constants;

use crate::{bindings::pydantic::PydanticProfile, constants::DEFAULT_CLASS_NAME};
use athalar_core::{AthalarAdapter, AthalarAtom, AthalarBinding};

pub fn add_python_final_files(
    binding: &AthalarBinding,
    atoms: &Vec<AthalarAtom>,
) -> anyhow::Result<String> {
    let details = match &binding.profile {
        AthalarAdapter::ClassValidator(_) => unimplemented!(),
        AthalarAdapter::Pydantic(x) => PydanticProfile {
            class_name: x
                .class_name
                .clone()
                .unwrap_or_else(|| DEFAULT_CLASS_NAME.to_string()),
        },
    };
    dbg!(&details);
    Ok("".to_string())
}
