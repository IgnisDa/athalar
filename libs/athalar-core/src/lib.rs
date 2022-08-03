mod atom;
mod binding;
mod constants;
mod generator;
mod partial;

use config::AthalarConfig;
use derive_builder::Builder;
use generator::AthalarGenerator;
use partial::AthalarPartial;

/// The root instance that manipulates and stores data about an Athalar project.
#[derive(Debug, PartialEq, Builder)]
pub struct Athalar {
    /// The configuration to use for the Athalar instance
    pub config: AthalarConfig,

    /// The variable partials that were discovered in this run
    pub partials: Vec<AthalarPartial>,

    /// The generators that were discovered in this run
    pub generators: Vec<AthalarGenerator>,
}

pub mod config;
pub mod utils;
