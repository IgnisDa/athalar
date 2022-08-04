use crate::generator::AthalarGenerator;

#[derive(Debug)]
pub enum GeneratorBindingReport {
    CanNotCreateFile,
    FileAlreadyExists,
}

#[derive(Debug)]
pub struct GeneratorReport<'a> {
    bindings: Vec<(&'a AthalarGenerator, GeneratorBindingReport)>,
}

#[derive(Debug)]
pub struct ValidationReport<'a> {
    pub generators: GeneratorReport<'a>,
}

impl<'a> ValidationReport<'a> {
    fn new() -> Self {
        let generators = GeneratorReport { bindings: vec![] };
        Self { generators }
    }

    pub fn add_binding_report(
        &mut self,
        generator: &'a AthalarGenerator,
        report: GeneratorBindingReport,
    ) {
        self.generators.bindings.push((generator, report));
    }
}

impl<'a> Default for ValidationReport<'a> {
    fn default() -> Self {
        Self::new()
    }
}
