use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AtomValidator {
    Number,
    String,
    Url,
    Port,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum AtomKind {
    Number,
    String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AthalarAtom {
    /// The name of this configuration variable
    name: String,

    /// the type that the final generated config variable should have
    kind: Option<AtomKind>,

    /// The validators that should be applied to this configuration variable
    #[serde(default)]
    pub validators: Vec<AtomValidator>,

    /// An optional description that will get included in the generated code
    description: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_validators_in_yaml_yields_validator_of_zero_len() {
        let s = r#"
        name: MAIL_PORT
        "#;
        let aca = serde_yaml::from_str::<AthalarAtom>(s).unwrap();
        assert_eq!(aca.validators.len(), 0);
    }

    #[test]
    fn correct_number_of_validators() {
        let s = r#"
        name: MAIL_PORT
        validators:
            - !Port
        "#;
        let aca = serde_yaml::from_str::<AthalarAtom>(s).unwrap();
        assert_eq!(aca.validators.len(), 1);
    }
}
