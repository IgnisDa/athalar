use athalar_core::AtomKind;
use strum_macros::Display;

#[derive(Debug, Display)]
pub enum AthalarPythonKind {
    #[strum(serialize = "int")]
    Number,

    #[strum(serialize = "str")]
    String,

    #[strum(serialize = "Any")]
    Any,
}

impl From<AtomKind> for AthalarPythonKind {
    fn from(av: AtomKind) -> Self {
        match av {
            AtomKind::Number => Self::Number,
            AtomKind::String => Self::String,
            AtomKind::Any => Self::Any,
        }
    }
}
