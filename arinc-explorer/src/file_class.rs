use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FileClass {
    Load,
    Batch,
    Media,
}
impl FileClass {
    pub fn get_file_type(media_file_format_verion: u16) -> Option<FileClass> {
        match media_file_format_verion {
            // 0x8002 for ARINC665-1. 0x8004 for ARINC665-5.
            0x8002 | 0x8004 => Some(FileClass::Load),

            // ARINC665-5 only
            0x9004 => Some(FileClass::Batch),

            // ARINC665-5 only
            0xA004 => Some(FileClass::Media),
            _ => None,
        }
    }
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
