use std::fmt::{self, Display, Formatter};
use std::fs::File as FsFile; // Rename to avoid conflict with file::File
use std::io::{Cursor, Read};
use std::path::Path;

use binrw::{binrw, BinRead};
use file::File;

use crate::error::FilesLumError;
use crate::file_class::FileClass;
use crate::utils::{combine_words, vec16_to_string};

mod file;

#[binrw]
#[brw(big)]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct FilesLum {
    file_length_msb: u16,
    file_length_lsb: u16,
    media_file_format_verion: u16,

    #[br(if(media_file_format_verion!=0x8002))]
    spare: u16,
    pointer_to_media_set_pn_length_msb: u16,
    pointer_to_media_set_pn_length_lsb: u16,
    pointer_to_number_of_media_set_files_msb: u16,
    pointer_to_number_of_media_set_files_lsb: u16,
    pointer_to_user_defined_data_msb: u16,
    pointer_to_user_defined_data_lsb: u16,

    #[br(if(media_file_format_verion!=0x8002))]
    pointer_to_file_check_value_length_msb: u16,

    #[br(if(media_file_format_verion!=0x8002))]
    pointer_to_file_check_value_length_lsb: u16,
    // Expansion point no1
    media_set_pn_length: u16, // number of chars

    #[br(count = media_set_pn_length.div_ceil(2))]
    media_set_pn: Vec<u16>, // It is always an even length
    media_sequence_number_x: u8,
    number_of_media_set_members_y: u8,
    number_of_media_set_files: u16,

    #[br(count = number_of_media_set_files)]
    media_set_files: Vec<File>,
    // Expansion point no3
    #[br(if(combine_words(pointer_to_user_defined_data_msb, pointer_to_user_defined_data_lsb)!=0), count = (combine_words(file_length_msb, file_length_lsb) - combine_words(pointer_to_user_defined_data_msb, pointer_to_user_defined_data_lsb) - 1).div_ceil(2))]
    user_defined_data: Option<Vec<u16>>,

    #[br(if(media_file_format_verion!=0x8002 && combine_words(pointer_to_file_check_value_length_msb, pointer_to_file_check_value_length_lsb)!=0))]
    file_check_value_length: Option<u16>,

    #[br(if(media_file_format_verion!=0x8002 && file_check_value_length.is_some()))]
    file_check_value_type: Option<u16>,

    #[br(if(media_file_format_verion!=0x8002 && file_check_value_length.is_some()), count = file_check_value_length.unwrap_or_default().div_ceil(2))]
    file_check_value: Option<Vec<u16>>,

    file_crc: u16,
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
        let mut file = FsFile::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let files_lum = FilesLum::read(&mut Cursor::new(buf))?;
        Ok(files_lum)
    }

    fn get_file_type_string(&self) -> String {
        match FileClass::get_file_type(self.media_file_format_verion) {
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

    #[must_use]
    pub fn get_media_set_pn(&self) -> String {
        // If media_set_pn_length is odd, an extra NUL byte is added at the end.
        // This function removes the NUL byte if it exists.
        vec16_to_string(&self.media_set_pn, self.media_set_pn_length as usize)
    }
}
impl Display for FilesLum {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            " \
            {} Bytes\n \
            {}\n \
            {} | {} | {}  Offsets\n \
            {} Chars in Media Set PN\n \
            {} Media Set PN\n \
            {} Media number\n \
            {} Total Media sets\n \
            {} Total Media set files\n \
            {}\n \
            0x{:x} CRC\n \
            ",
            self.get_file_length() * 2,
            self.get_file_type_string(),
            self.get_pointer_to_media_set_pn_length(),
            self.get_pointer_to_number_of_media_set_files(),
            self.get_pointer_to_user_defined_data(),
            self.media_set_pn_length,
            self.get_media_set_pn(),
            self.media_sequence_number_x,
            self.number_of_media_set_members_y,
            self.number_of_media_set_files,
            self.media_set_files
                .iter()
                .map(|f| format!("{f}"))
                .fold(String::new(), |acc, arg| acc + arg.as_str()),
            self.file_crc
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_simple_files_lum() {
        // 1 hex = 4 bits
        // 4 hex = 16 bits = 2 bytes
        let file = PathBuf::from("../test-data/FILES.LUM");
        let files_lum = FilesLum::new(file.as_path()).unwrap();
        println!("{files_lum}");

        assert_eq!(files_lum.get_file_length(), 300);
        assert_eq!(files_lum.media_file_format_verion, 0x8002);
        assert_eq!(files_lum.get_pointer_to_media_set_pn_length(), 0x9);
        assert_eq!(files_lum.get_pointer_to_number_of_media_set_files(), 0x10);
        assert_eq!(files_lum.get_pointer_to_user_defined_data(), 0x0);
        assert_eq!(files_lum.media_set_pn_length, 10);
        assert_eq!(files_lum.media_set_pn.len(), 5);
        assert_eq!(files_lum.get_media_set_pn(), "ABC1813001");
        assert_eq!(files_lum.media_sequence_number_x, 1);
        assert_eq!(files_lum.number_of_media_set_members_y, 1);
        assert_eq!(files_lum.number_of_media_set_files, 14);
        assert_eq!(files_lum.media_set_files.len(), 14);
        assert_eq!(files_lum.user_defined_data, None);
        assert_eq!(files_lum.file_crc, 0x3BE8);
    }
}
