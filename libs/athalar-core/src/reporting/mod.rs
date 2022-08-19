//! This module is responsible for generating reports during the validation phase of an
//! athalar project.

use crate::{atom::AthalarAtom, binding::AthalarBinding, generator::AthalarGeneratorContent};
use strum_macros::Display;

/// The different levels of errors that can be present in a validation report.
#[derive(Debug, PartialEq, Eq, Display)]
pub enum ReportLevel {
    /// If this error is encountered, the generation phase is guaranteed to fail.
    Severe,
    /// If this error is encountered, the generation phase should succeed but might have
    /// unexpected results. It is recommended to not ignore them.
    Warning,
}

// generators

#[derive(Debug)]
pub enum ReportMessageOrigin {
    GeneratorBinding,
    GeneratorConfig,
    PartialConfig,
}

/// An error code combined with a message that can be displayed to the end users
#[derive(Debug)]
pub struct ReportMessage {
    /// The origin of the report
    pub origin: ReportMessageOrigin,

    /// A unique code for the report
    pub code: &'static str,

    /// The severity of the report
    pub level: ReportLevel,

    /// The actual error string
    pub message: String,
}

/// This struct will validation information about generators in an athalar project.
#[derive(Debug)]
pub struct GeneratorReport<'a> {
    pub bindings: Vec<(&'a AthalarBinding, ReportMessage)>,
    pub config: Vec<(&'a AthalarGeneratorContent, ReportMessage)>,
}

// partials

/// This struct will validation information about partials in an athalar project.
#[derive(Debug)]
pub struct PartialReport<'a> {
    pub config: Vec<(&'a AthalarAtom, ReportMessage)>,
}

/// This contains all the information about the different problems that were detected
/// during the validation phase.
#[derive(Debug)]
pub struct ValidationReport<'a> {
    pub generators: GeneratorReport<'a>,
    pub partials: PartialReport<'a>,
}

impl<'a> ValidationReport<'a> {
    fn new() -> Self {
        let generators = GeneratorReport {
            bindings: vec![],
            config: vec![],
        };
        let partials = PartialReport { config: vec![] };
        Self {
            generators,
            partials,
        }
    }

    /// Whether there are errors present in the report.
    pub fn has_errors(&self) -> bool {
        !(self.generators.bindings.is_empty() && self.generators.config.is_empty())
    }

    /// Whether any of the errors present in the reporter are of the supplied level. Can be
    /// used to detect errors and terminate early.
    pub fn has_errors_with_level(&self, level: ReportLevel) -> bool {
        for b in self.generators.bindings.iter() {
            if b.1.level == level {
                return true;
            }
        }
        for c in self.generators.config.iter() {
            if c.1.level == level {
                return true;
            }
        }
        for c in self.partials.config.iter() {
            if c.1.level == level {
                return true;
            }
        }
        false
    }

    pub fn add_generator_binding_report(
        &mut self,
        binding: &'a AthalarBinding,
        report: ReportMessage,
    ) {
        self.generators.bindings.push((binding, report));
    }

    pub fn add_generator_config_report(
        &mut self,
        generator_content: &'a AthalarGeneratorContent,
        report: ReportMessage,
    ) {
        self.generators.config.push((generator_content, report));
    }

    pub fn add_partial_config_report(&mut self, atom: &'a AthalarAtom, report: ReportMessage) {
        self.partials.config.push((atom, report));
    }
}

impl<'a> Default for ValidationReport<'a> {
    fn default() -> Self {
        Self::new()
    }
}

pub mod data;
