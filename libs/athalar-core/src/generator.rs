use derive_builder::Builder;
use std::path::PathBuf;

/// Contains information about a discovered generator in the project.
#[derive(Debug, PartialEq, Builder)]
pub struct AthalarGenerator {
    /// The path to this partial relative to the current directory
    source: PathBuf,

    /// The actual data that is in this generator file
    data: AthalarGeneratorContent,
}

impl AthalarGenerator {
    /// The directory in which this partial will be found, relative to partial directory
    pub fn source(&self) -> &PathBuf {
        &self.source
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AthalarGeneratorContent {
    IncludePartial(String),
}

#[derive(Debug, PartialEq, Builder)]
struct AthalarGeneratorData {
    /// The actual data in the file
    #[builder(setter(into, strip_option), default)]
    config: Vec<AthalarGeneratorContent>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_config_should_have_correct_length() {
        let agd = AthalarGeneratorDataBuilder::default().build().unwrap();
        assert_eq!(agd.config.len(), 0);
    }

    #[test]
    fn config_should_have_correct_length() {
        let agd = AthalarGeneratorDataBuilder::default()
            .config(vec![AthalarGeneratorContent::IncludePartial("mail".into())])
            .build()
            .unwrap();
        assert_eq!(agd.config.len(), 1);
    }

    #[test]
    fn config_should_have_correct_value_inside_include_partial() {
        let agd = AthalarGeneratorDataBuilder::default()
            .config(vec![AthalarGeneratorContent::IncludePartial("mail".into())])
            .build()
            .unwrap();
        match agd.config.get(0).unwrap() {
            AthalarGeneratorContent::IncludePartial(x) => assert_eq!(x, "mail"),
        }
    }
}
