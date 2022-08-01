use derive_builder::Builder;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AthalarConfigVersion {
    One = 1,
}

/// The container for configuring the Athalar instance.
#[derive(Debug, PartialEq, Builder)]
pub struct AthalarConfig {
    version: AthalarConfigVersion,

    source: PathBuf,

    #[builder(default = "self.default_partials()?")]
    pub partials: PathBuf,

    #[builder(default = "self.default_generators()?")]
    pub generators: PathBuf,
}

impl AthalarConfigBuilder {
    fn default_partials(&self) -> Result<PathBuf, String> {
        dbg!(&self.source);
        let source = self.source.clone().unwrap();
        Ok(source.join("partials"))
    }

    fn default_generators(&self) -> Result<PathBuf, String> {
        let source = self.source.clone().unwrap();
        Ok(source.join("generators"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_correct_toml() {
        let ac = AthalarConfigBuilder::default()
            .version(AthalarConfigVersion::One)
            .source(PathBuf::from("other_source"))
            .build()
            .unwrap();
        assert_eq!(ac.version, AthalarConfigVersion::One);
        assert_eq!(ac.source, PathBuf::from("other_source"));
    }

    #[test]
    fn gets_correct_value_of_config_dirs() {
        let ac = AthalarConfigBuilder::default()
            .version(AthalarConfigVersion::One)
            .source(PathBuf::from("src"))
            .build()
            .unwrap();
        assert_eq!(ac.partials, PathBuf::from("src").join("partials"));
        assert_eq!(ac.generators, PathBuf::from("src").join("generators"));
    }

    #[test]
    #[should_panic]
    fn panic_on_no_version() {
        AthalarConfigBuilder::default().build().unwrap();
    }

    #[test]
    #[should_panic]
    fn panic_on_no_source() {
        AthalarConfigBuilder::default()
            .version(AthalarConfigVersion::One)
            .build()
            .unwrap();
    }
}
