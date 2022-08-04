mod atom;
mod binding;
mod config;
mod generator;
mod partial;
mod utils;

use generator::AthalarGenerator;
use partial::AthalarPartial;
use utils::{load_generators, load_partials};

/// The root instance that manipulates and stores data about an Athalar project. When
/// generating information about a project, it does so in the following phases:
///
/// - _scouting_: Reading the project configuration. This then stored in the
///   [AthalarConfig].
/// - _parsing_: All the relevant athalar files in the project are read and stored in this
///   struct. This is done by the [new](Self::from_config) method.
/// - _validation_: Validates all the information collected above. If it finds something
///   anomalous (for eg: a configuration variable that is repeated, a generator output path
///   that can not be created etc), and then returns that information. This is done by the
///   [validate](Self::validate) method.
#[derive(Debug, PartialEq)]
pub struct Athalar {
    /// The configuration to use for the Athalar instance
    config: AthalarConfig,

    /// The variable partials that were discovered in this run
    partials: Vec<AthalarPartial>,

    /// The generators that were discovered in this run
    generators: Vec<AthalarGenerator>,
}

impl Athalar {
    /// Takes the project configuration and finds and loads all the relevant athalar
    /// configuration files.
    pub fn from_config(config: AthalarConfig) -> Self {
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
