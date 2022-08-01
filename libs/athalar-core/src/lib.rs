mod atom;
mod binding;
mod config;
mod generator;
mod partial;

use config::AthalarConfig;
use generator::AthalarGenerator;
use partial::AthalarPartial;

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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AthalarConfigKind {
    Variable,
}

impl AthalarConfigKind {
    fn variable() -> Self {
        Self::Variable
    }
}

#[cfg(test)]
mod test {}
