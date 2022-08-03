mod atom;
mod binding;
mod config;
mod generator;
mod partial;
mod utils;

use generator::AthalarGenerator;
use partial::AthalarPartial;
use utils::{load_generators, load_partials};

/// The root instance that manipulates and stores data about an Athalar project.
#[derive(Debug, PartialEq)]
pub struct Athalar {
    /// The configuration to use for the Athalar instance
    pub config: AthalarConfig,

    /// The variable partials that were discovered in this run
    pub partials: Vec<AthalarPartial>,

    /// The generators that were discovered in this run
    pub generators: Vec<AthalarGenerator>,
}

impl Athalar {
    pub fn new(config: AthalarConfig) -> Self {
        let partials = load_partials(&config.partials());
        let generators = load_generators(&config.generators());
        Self {
            config,
            partials,
            generators,
        }
    }
}

pub use config::AthalarConfig;
pub mod constants;
