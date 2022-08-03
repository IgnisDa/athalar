use athalar_core::utils::{load_config, load_generators, load_partials};
use std::env;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let d = load_config(&current_dir);
    load_partials(&d.partials());
    load_generators(&d.generators());
}
