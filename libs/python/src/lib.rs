mod atoms;

use athalar_core::{AthalarAtom, AthalarBinding};

pub fn add_python_final_files(
    binding: &AthalarBinding,
    atoms: &Vec<AthalarAtom>,
) -> anyhow::Result<String> {
    Ok("".to_string())
}
