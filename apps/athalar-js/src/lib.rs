mod atoms;
mod bindings;
mod constants;

use athalar_core::{from_path, Athalar, AthalarAdapter};
use atoms::AthalarJsKind;
use bindings::{AthalarJsBindingType, ClassValidator, ClassValidatorProfile};
use constants::DEFAULT_CLASS_NAME;
use itertools::{EitherOrBoth, Itertools};
use napi::{Error, Result, Status};
use napi_derive::napi;

#[derive(Debug)]
#[napi(object)]
pub struct AthalarJsReport {
    /// The severity level of the report
    pub level: String,

    /// The error message of the report
    pub message: String,
}

#[napi(object)]
pub struct AthalarJsValidationReport {
    pub generators: Vec<AthalarJsReport>,

    pub partials: Vec<AthalarJsReport>,
}

#[napi]
pub struct AthalarJs(Athalar);

#[napi(object)]
pub struct AthalarJsBindingInformation {
    /// The fully qualified path to where the binding needs to be generated
    pub output: String,
}

#[napi]
impl AthalarJs {
    /// Create an instance from a path
    /// # Arguments
    /// * path: A fully qualified valid path. Panics if the path is not valid.
    #[napi(factory)]
    pub fn from_path(path: String) -> Result<Self> {
        from_path(path)
            .map(Self)
            .map_err(|e| Error::new(Status::GenericFailure, e))
    }

    /// Get validation reports for the project
    #[napi]
    pub fn get_validation_reports(&self) -> AthalarJsValidationReport {
        let report = self.0.get_validation_report();
        let mut generators = vec![];
        let mut partials = vec![];
        for msg in report
            .generators
            .bindings
            .iter()
            .zip_longest(report.generators.config.iter())
        {
            if let EitherOrBoth::Left(l) = msg {
                generators.push(AthalarJsReport {
                    level: l.1.level.to_string(),
                    message: l.1.message.clone(),
                });
            }
            if let EitherOrBoth::Right(r) = msg {
                generators.push(AthalarJsReport {
                    level: r.1.level.to_string(),
                    message: r.1.message.clone(),
                });
            }
        }
        for msg in report.partials.config.iter() {
            partials.push(AthalarJsReport {
                level: msg.1.level.to_string(),
                message: msg.1.message.clone(),
            });
        }
        AthalarJsValidationReport {
            generators,
            partials,
        }
    }

    /// Get the final information that will be used to generate the bindings
    #[napi]
    pub fn get_information(&self) -> Result<Vec<AthalarJsBinding>> {
        let mut bindings = vec![];
        let information = self.0.get_information().unwrap();
        for (generator, atoms) in information.generators.iter() {
            for binding in generator.data.bindings.iter() {
                let details = match &binding.profile {
                    AthalarAdapter::ClassValidator(x) => ClassValidatorProfile {
                        class_name: x
                            .class_name
                            .clone()
                            .unwrap_or_else(|| DEFAULT_CLASS_NAME.to_string()),
                    },
                    AthalarAdapter::Pydantic(_) => continue,
                };
                // the final path where the output of this binding must be placed
                let output = binding
                    .output(&information.config.project_source())
                    .into_os_string()
                    .into_string()
                    .unwrap();
                let mut _atoms = vec![];
                for atom in atoms.iter() {
                    let mut validators = atom
                        .validators
                        .iter()
                        .map(|f| ClassValidator::from(f.clone()))
                        .collect::<Vec<_>>();
                    if validators.is_empty() {
                        validators.push(ClassValidator::Allow)
                    }
                    let _atom = AthalarJsAtom {
                        name: atom.name.clone(),
                        kind: AthalarJsKind::from(atom.kind).to_string(),
                        validators: validators.into_iter().map(|f| f.to_string()).collect(),
                        description: atom.description.clone(),
                    };
                    _atoms.push(_atom);
                }
                let variety = AthalarJsBindingType::from(binding.profile.clone());
                let _binding = AthalarJsBinding {
                    output,
                    atoms: _atoms,
                    details,
                    variety,
                };
                bindings.push(_binding);
            }
        }
        Ok(bindings)
    }
}

#[napi(object)]
#[derive(Debug)]
pub struct AthalarJsAtom {
    pub name: String,

    pub kind: String,

    pub validators: Vec<String>,

    pub description: Option<String>,
}

#[napi(object)]
#[derive(Debug)]
pub struct AthalarJsBinding {
    pub output: String,

    pub variety: AthalarJsBindingType,

    pub details: ClassValidatorProfile,

    pub atoms: Vec<AthalarJsAtom>,
}
