use config::AthalarConfig;
use partial::AthalarPartial;
use serde::{Deserialize, Serialize};

mod config;
mod config_atom;
mod partial;

#[derive(Debug, PartialEq)]
pub struct Athalar {
    /// The configuration to use for the Athalar instance
    config: AthalarConfig,

    /// The variable partials that were discovered in this run
    partials: Vec<AthalarPartial>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AthalarConfigKind {
    Variable,
    Generator,
}

impl AthalarConfigKind {
    fn variable() -> Self {
        Self::Variable
    }
}

#[cfg(test)]
mod test {}
