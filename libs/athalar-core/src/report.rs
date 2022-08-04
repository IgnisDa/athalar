use crate::{binding::AthalarBinding, generator::AthalarGeneratorContent};

#[derive(Debug)]
pub enum ReportLevel {
    Severe,
    Warning,
}

#[derive(Debug)]
pub enum GeneratorBindingReport {
    CanNotCreateFile,
    FileAlreadyExists,
}

#[derive(Debug)]
pub enum GeneratorConfigReport {
    PartialDoesNotExist,
}

/// This struct will contain a list of validations
#[derive(Debug)]
pub struct GeneratorReport<'a> {
    bindings: Vec<(&'a AthalarBinding, GeneratorBindingReport, ReportLevel)>,
    config: Vec<(
        &'a AthalarGeneratorContent,
        GeneratorConfigReport,
        ReportLevel,
    )>,
}

#[derive(Debug)]
pub struct ValidationReport<'a> {
    pub generators: GeneratorReport<'a>,
}

impl<'a> ValidationReport<'a> {
    fn new() -> Self {
        let generators = GeneratorReport {
            bindings: vec![],
            config: vec![],
        };
        Self { generators }
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
        config: &'a AthalarGeneratorContent,
        report: GeneratorConfigReport,
        level: ReportLevel,
    ) {
        self.generators.config.push((config, report, level));
    }
}

impl<'a> Default for ValidationReport<'a> {
    fn default() -> Self {
        Self::new()
    }
}
