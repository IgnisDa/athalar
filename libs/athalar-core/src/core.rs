use crate::{
    atom::AthalarAtom,
    config::AthalarConfig,
    generator::{AthalarGenerator, AthalarGeneratorContent},
    partial::AthalarPartial,
    report::{
        GeneratorBindingReport, GeneratorConfigReport, PartialConfigReport, ReportLevel,
        ValidationReport,
    },
    utils::{load_generators, load_partials},
};
use std::fs;

/// The root instance that manipulates and stores data about an Athalar project. When
/// generating information about a project, it does so in the following phases:
///
/// - _scouting_: Reading the project configuration and storing it in [AthalarConfig]. This
///   is done by the consuming library (and not internally).
/// - _parsing_: All the relevant athalar files in the project are read and stored in this
///   struct. This is done by [this](Self::from_config) method.
/// - _validation_: Validates all the information collected above. If it finds something
///   anomalous (for eg: a configuration variable that is repeated, a generator output path
///   that can not be created etc), and then returns that information. This is done by
///   [this](Self::get_validation_report) method.
///
/// Once these phases are complete, it returns an information table (via
/// [this](Self::get_information) method) that can be used by the consuming library to
/// generate the desired binding.
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
    /// and returns it so that it can be displayed to the end user. It is up to the
    /// consuming library on how it decides to handle this report and whether to force the
    /// user to rectify these errors or allow them to continue.
    pub fn get_validation_report(&self) -> ValidationReport {
        let mut reporter = ValidationReport::default();
        // handle generators
        self.set_generator_binding_errors(&mut reporter);
        self.set_generator_config_errors(&mut reporter);
        // handle partials
        self.set_partial_config_errors(&mut reporter);
        reporter
    }

    fn set_generator_binding_errors<'a>(&'a self, reporter: &mut ValidationReport<'a>) {
        self.generators.iter().for_each(|g| {
            g.data.bindings.iter().for_each(|b| {
                if b.output.exists() {
                    reporter.add_generator_binding_report(
                        b,
                        GeneratorBindingReport::FileAlreadyExists,
                        ReportLevel::Warning,
                    );
                } else {
                    // if file already exists, we can assume it can be created
                    if fs::write(&b.output, "temp").is_ok() {
                        fs::remove_file(&b.output).unwrap();
                    } else {
                        reporter.add_generator_binding_report(
                            b,
                            GeneratorBindingReport::CanNotCreateFile,
                            ReportLevel::Severe,
                        );
                    };
                }
                if g.data
                    .bindings
                    .iter()
                    .any(|ib| ib.output == b.output && ib.id != b.id)
                {
                    reporter.add_generator_binding_report(
                        b,
                        GeneratorBindingReport::FileConflict,
                        ReportLevel::Warning,
                    );
                }
            })
        });
    }

    fn set_generator_config_errors<'a>(&'a self, reporter: &mut ValidationReport<'a>) {
        self.generators.iter().for_each(|g| {
            g.data.config.iter().for_each(|c| match c {
                AthalarGeneratorContent::IncludePartial(partial_name) => {
                    if !self.partials.iter().any(|p| &p.name == partial_name) {
                        reporter.add_generator_config_report(
                            c,
                            GeneratorConfigReport::PartialDoesNotExist,
                            ReportLevel::Severe,
                        );
                    }
                }
            })
        });
    }

    fn set_partial_config_errors<'a>(&'a self, reporter: &mut ValidationReport<'a>) {
        self.partials.iter().for_each(|p| {
            p.data.config.iter().for_each(|c| {
                if p.data
                    .config
                    .iter()
                    .any(|ip| ip.name == p.name && ip.id != p.id)
                {
                    reporter.add_partial_config_report(
                        c,
                        PartialConfigReport::NameConflict,
                        ReportLevel::Warning,
                    );
                }
            });
        });
    }

    /// Get an information table that can be used to generate bindings. This method _might_
    /// fail if there are any [severe](ReportLevel::Severe) errors. Ideally it should be
    /// called only after the report have been taken care of.
    pub fn get_information(&self) -> Result<AthalarInformation, String> {
        let mut info: Vec<(&AthalarGenerator, Vec<AthalarAtom>)> = vec![];
        for generator in self.generators.iter() {
            let mut partials = vec![];
            for config in generator.data.config.iter() {
                match config {
                    AthalarGeneratorContent::IncludePartial(name) => {
                        match self.partials.iter().find(|p| &p.name == name) {
                            Some(p) => {
                                p.data.config.iter().for_each(|c| partials.push(c.clone()));
                            }
                            None => return Err(format!("Could not find partial: {:?}", name)),
                        };
                    }
                };
            }
            info.push((generator, partials));
        }
        Ok(AthalarInformation(info))
    }
}

/// This will contain all the structured information that will be needed to generate
/// bindings.
#[derive(Debug)]
pub struct AthalarInformation<'a>(Vec<(&'a AthalarGenerator, Vec<AthalarAtom>)>);
