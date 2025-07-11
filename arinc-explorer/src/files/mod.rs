use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;

use binrw::{binrw, BinRead};

use crate::error::FilesLumError;
use crate::file_class::FileClass;
use crate::utils::combine_words;

#[binrw]
#[brw(big)]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct FilesLum {
    file_length_msb: u16,
    file_length_lsb: u16,
    media_file_format_verion: u16,
    //spare: u16, // Seems like the spare doesn't exist
    pointer_to_media_set_pn_length_msb: u16,
    pointer_to_media_set_pn_length_lsb: u16,
    pointer_to_number_of_media_set_files_msb: u16,
    pointer_to_number_of_media_set_files_lsb: u16,
    pointer_to_user_defined_data_msb: u16,
    pointer_to_user_defined_data_lsb: u16,
    pointer_to_file_check_value_length_msb: u16,
    pointer_to_file_check_value_length_lsb: u16,
    // Expansion point no1
}
impl FilesLum {
    /// Constructs a new [`FilesLum`] object.
    ///
    /// The function doesn't have any protection against big file size and
    /// attemps to read the whole file into the memory.
    ///
    /// # Arguments
    ///
    /// - `path`: the path to the `FILES.LUM` file.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the file is missing or if there is a problem reading
    /// the file into the [`FilesLum`] struct.
    pub fn new(path: &Path) -> Result<Self, FilesLumError> {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let files_lum = FilesLum::read(&mut Cursor::new(buf))?;
        Ok(files_lum)
    }

    fn get_file_type(&self) -> Option<FileClass> {
        match self.media_file_format_verion {
            0x8002..=0x8004 => Some(FileClass::Load),
            0x9004 => Some(FileClass::Batch),
            0xA004 => Some(FileClass::Media),
            _ => None,
        }
    }

    fn get_file_type_string(&self) -> String {
        match self.get_file_type() {
            Some(x) => x.to_string(),
            None => format!("{} unrecognised file class", self.media_file_format_verion),
        }
    }

    // Returns the number of 16-bit words
    #[must_use]
    pub fn get_file_length(&self) -> u32 {
        combine_words(self.file_length_msb, self.file_length_lsb)
    }

    // Returns the offset in 16-bit words from the beginning of the file
    #[must_use]
    pub fn get_pointer_to_media_set_pn_length(&self) -> u32 {
        combine_words(
            self.pointer_to_media_set_pn_length_msb,
            self.pointer_to_media_set_pn_length_lsb,
        )
    }

    // Returns the offset in 16-bit words from the beginning of the file
    #[must_use]
    pub fn get_pointer_to_number_of_media_set_files(&self) -> u32 {
        combine_words(
            self.pointer_to_number_of_media_set_files_msb,
            self.pointer_to_number_of_media_set_files_lsb,
        )
    }

    // Returns the offset in 16-bit words from the beginning of the file
    #[must_use]
    pub fn get_pointer_to_user_defined_data(&self) -> u32 {
        combine_words(
            self.pointer_to_user_defined_data_msb,
            self.pointer_to_user_defined_data_lsb,
        )
    }

    // Returns the offset in 16-bit words from the beginning of the file
    #[must_use]
    pub fn get_pointer_to_file_check_value_length(&self) -> u32 {
        combine_words(
            self.pointer_to_file_check_value_length_msb,
            self.pointer_to_file_check_value_length_lsb,
        )
    }
}
impl Display for FilesLum {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} Bytes\n{}\n{} | {} | {} | {:x} Offsets",
            self.get_file_length() * 2,
            self.get_file_type_string(),
            self.get_pointer_to_media_set_pn_length(),
            self.get_pointer_to_number_of_media_set_files(),
            self.get_pointer_to_user_defined_data(),
            self.get_pointer_to_file_check_value_length()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[ignore = "File not found"]
    #[test]
    fn test_simple_files_lum() {
        // 1 hex = 4 bits
        // 4 hex = 16 bits = 2 bytes
        let file = PathBuf::from("../../FILES.LUM");
        let files_lum = FilesLum::new(file.as_path()).unwrap();
        println!("{files_lum}");
    }
}
