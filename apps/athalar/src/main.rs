use athalar_core::{constants::ATHALAR_CONFIG_FILE, Athalar, AthalarConfig};
use std::{env, fs, str::FromStr};

fn main() {
    let s = fs::read_to_string(env::current_dir().unwrap().join(ATHALAR_CONFIG_FILE)).unwrap();
    let config = AthalarConfig::from_str(&s).unwrap();
    let athalar = Athalar::new(config);
    dbg!(&athalar);
}
