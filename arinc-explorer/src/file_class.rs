use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FileClass {
    Load,
    Batch,
    Media,
}
impl Display for FileClass {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            FileClass::Load => write!(f, "Load File"),
            FileClass::Batch => write!(f, "Batch File"),
            FileClass::Media => write!(f, "Media File"),
        }
    }
}
