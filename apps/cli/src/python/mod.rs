use athalar_core::from_path;
use std::path::Path;

pub fn run(path: &Path) {
    let a = from_path(path.to_str().unwrap().to_owned());
    dbg!(a);
}
