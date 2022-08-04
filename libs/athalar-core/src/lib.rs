mod atom;
mod binding;
mod config;
mod generator;
mod partial;
mod report;
mod utils;

use std::fs;

use generator::AthalarGenerator;
use partial::AthalarPartial;
use report::{GeneratorBindingReport, ValidationReport};
use utils::{load_generators, load_partials};

/// The root instance that manipulates and stores data about an Athalar project. When
/// generating information about a project, it does so in the following phases:
///
/// - _scouting_: Reading the project configuration. This then stored in the
///   [AthalarConfig].
/// - _parsing_: All the relevant athalar files in the project are read and stored in this
///   struct. This is done by [this](Self::from_config) method.
/// - _validation_: Validates all the information collected above. If it finds something
///   anomalous (for eg: a configuration variable that is repeated, a generator output path
///   that can not be created etc), and then returns that information. This is done by
///   [this](Self::get_validation_report) method.
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

    /// Once the project files are loaded, this runs a validation on all the collected data
    /// and returns it so that it can be displayed to the end user.
    pub fn get_validation_report(&self) -> ValidationReport {
        let mut report = ValidationReport::default();
        self.set_binding_errors(&mut report);
        report
    }

    fn set_binding_errors<'a>(&'a self, report: &mut ValidationReport<'a>) {
        self.generators.iter().for_each(|g| {
            g.data.bindings.iter().for_each(|b| {
                if b.output.exists() {
                    report.add_binding_report(g, GeneratorBindingReport::FileAlreadyExists);
                } else {
                    // if file already exists, we can assume it can be created
                    if fs::write(&b.output, "temp").is_ok() {
                        fs::remove_file(&b.output).unwrap();
                    } else {
                        report.add_binding_report(g, GeneratorBindingReport::CanNotCreateFile);
                    };
                }
            })
        });
    }
}

pub use config::AthalarConfig;
pub mod constants;
