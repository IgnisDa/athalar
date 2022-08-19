use athalar_core::AtomKind;
use strum_macros::Display;

#[derive(Debug, Display)]
pub enum AthalarJsKind {
    #[strum(serialize = "number")]
    Number,

    #[strum(serialize = "string")]
    String,

    #[strum(serialize = "any")]
    Any,
}

impl From<AtomKind> for AthalarJsKind {
    fn from(av: AtomKind) -> Self {
        match av {
            AtomKind::Number => Self::Number,
            AtomKind::String => Self::String,
            AtomKind::Any => Self::Any,
        }
    }
}
