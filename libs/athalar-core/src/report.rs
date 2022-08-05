//! This module is responsible for generating reports during the validation phase of an
//! athalar project.

use crate::{atom::AthalarAtom, binding::AthalarBinding, generator::AthalarGeneratorContent};

/// The different levels of errors that can be present in a validation report.
#[derive(Debug, PartialEq, Eq)]
pub enum ReportLevel {
    /// If this error is encountered, the generation phase is guaranteed to fail.
    Severe,
    /// If this error is encountered, the generation phase should succeed but might have
    /// unexpected results. It is recommended to not ignore them.
    Warning,
}

// generators

/// Errors related to a binding defined in a generator
#[derive(Debug)]
pub enum GeneratorBindingReport {
    CanNotCreateFile,
    FileAlreadyExists,
    FileConflict,
}

/// Errors related to a config defined in a generator
#[derive(Debug)]
pub enum GeneratorConfigReport {
    PartialDoesNotExist,
}

/// This struct will validation information about generators in an athalar project.
#[derive(Debug)]
pub struct GeneratorReport<'a> {
    bindings: Vec<(&'a AthalarBinding, GeneratorBindingReport, ReportLevel)>,
    config: Vec<(
        &'a AthalarGeneratorContent,
        GeneratorConfigReport,
        ReportLevel,
    )>,
}

// partials

/// Errors related to a config defined in a partial
#[derive(Debug)]
pub enum PartialConfigReport {
    NameConflict,
}

/// This struct will validation information about partials in an athalar project.
#[derive(Debug)]
pub struct PartialReport<'a> {
    config: Vec<(&'a AthalarAtom, PartialConfigReport, ReportLevel)>,
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
            if b.2 == level {
                return true;
            }
        }
        for c in self.generators.config.iter() {
            if c.2 == level {
                return true;
            }
        }
        for c in self.partials.config.iter() {
            if c.2 == level {
                return true;
            }
        }
        false
    }

    pub fn add_generator_binding_report(
        &mut self,
        binding: &'a AthalarBinding,
        report: GeneratorBindingReport,
        level: ReportLevel,
    ) {
        self.generators.bindings.push((binding, report, level));
    }

    pub fn add_generator_config_report(
        &mut self,
        generator_content: &'a AthalarGeneratorContent,
        report: GeneratorConfigReport,
        level: ReportLevel,
    ) {
        self.generators
            .config
            .push((generator_content, report, level));
    }

    pub fn add_partial_config_report(
        &mut self,
        atom: &'a AthalarAtom,
        report: PartialConfigReport,
        level: ReportLevel,
    ) {
        self.partials.config.push((atom, report, level));
    }
}

impl<'a> Default for ValidationReport<'a> {
    fn default() -> Self {
        Self::new()
    }
}
