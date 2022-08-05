use crate::{atom::AthalarAtom, binding::AthalarBinding, generator::AthalarGeneratorContent};

#[derive(Debug)]
pub enum ReportLevel {
    Severe,
    Warning,
}

// generators
//
#[derive(Debug)]
pub enum GeneratorBindingReport {
    CanNotCreateFile,
    FileAlreadyExists,
    FileConflict,
}

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

#[derive(Debug)]
pub enum PartialConfigReport {
    NameConflict,
}

/// This struct will validation information about generators in an athalar project.
#[derive(Debug)]
pub struct PartialReport<'a> {
    config: Vec<(&'a AthalarAtom, PartialConfigReport, ReportLevel)>,
}

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

    /// returns whether there are errors present in the report
    pub fn has_errors(&self) -> bool {
        !(self.generators.bindings.is_empty() && self.generators.config.is_empty())
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
