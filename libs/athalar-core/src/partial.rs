use crate::{atom::AthalarAtom, AthalarConfigKind};
use derive_builder::Builder;
use std::path::PathBuf;

/// Contains information about a discovered partial in the project.
#[derive(Debug, PartialEq, Builder)]
pub struct AthalarPartial {
    /// The path to this partial relative to the current directory
    source: PathBuf,

    /// The actual data that is in this generator file
    data: AthalarPartialData,
}

impl AthalarPartial {
    /// The directory in which this partial will be found, relative to partial directory
    pub fn source(&self) -> &PathBuf {
        &self.source
    }
}

#[derive(Debug, PartialEq, Builder, Clone)]
pub struct AthalarPartialData {
    /// The type of partial
    pub kind: AthalarConfigKind,
    /// The actual data in the file
    #[builder(setter(into, strip_option), default)]
    pub config: Vec<AthalarAtom>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_kind_gets_default_value() {
        let apd = AthalarPartialDataBuilder::default()
            .kind(AthalarConfigKind::Variable)
            .build()
            .unwrap();
        assert_eq!(apd.kind, AthalarConfigKind::Variable);
    }

    #[test]
    fn no_validators() {
        let apd = AthalarPartialDataBuilder::default()
            .kind(AthalarConfigKind::Variable)
            .config(vec![AthalarAtom::default()])
            .build()
            .unwrap();
        assert_eq!(apd.config.len(), 1);
        assert_eq!(apd.config[0].validators.len(), 0);
    }

    #[test]
    fn specifying_kind_as_variable_sets_correct_value() {
        let apd = AthalarPartialDataBuilder::default()
            .kind(AthalarConfigKind::Variable)
            .config(vec![AthalarAtom::default()])
            .build()
            .unwrap();
        assert_eq!(apd.kind, AthalarConfigKind::Variable);
    }

    #[test]
    #[should_panic]
    fn specifying_generator_kind_should_panic() {
        AthalarPartialDataBuilder::default().build().unwrap();
    }
}
