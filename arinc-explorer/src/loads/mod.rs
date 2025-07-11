use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;

use binrw::{binrw, BinRead};
use lsp::Lsp;

use crate::error::LoadsLumError;
use crate::utils::{combine_words, vec16_to_string};

mod lsp;
mod target_hw;

#[derive(Debug, Clone, Eq, PartialEq)]
enum FileClass {
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

#[binrw]
#[brw(big)]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct LoadsLum {
    file_length_msb: u16,
    file_length_lsb: u16,
    media_file_format_verion: u16,
    //spare: u16, // Seems like the spare doesn't exist
    pointer_to_media_set_pn_length_msb: u16,
    pointer_to_media_set_pn_length_lsb: u16,
    pointer_to_number_of_loads_msb: u16,
    pointer_to_number_of_loads_lsb: u16,
    pointer_to_user_defined_data_msb: u16,
    pointer_to_user_defined_data_lsb: u16,
    // Expansion point no1
    media_set_pn_length: u16, // number of chars

    #[br(count = media_set_pn_length.div_ceil(2))]
    media_set_pn: Vec<u16>, // It is always an even length
    media_sequence_number_x: u8,
    number_of_media_set_members_y: u8,
    number_of_loads: u16,

    #[br(count = number_of_loads)]
    loads: Vec<Lsp>,
    // Expansion point no3
    #[br(if(combine_words(pointer_to_user_defined_data_msb, pointer_to_user_defined_data_lsb)!=0), count = (combine_words(file_length_msb, file_length_lsb) - combine_words(pointer_to_user_defined_data_msb, pointer_to_user_defined_data_lsb) - 1).div_ceil(2))]
    user_defined_data: Option<Vec<u16>>,

    file_crc: u16,
}
impl LoadsLum {
    /// Constructs a new [`LoadsLum`] object.
    ///
    /// The function doesn't have any protection against big file size and
    /// attemps to read the whole file into the memory.
    ///
    /// # Arguments
    ///
    /// - `path`: the path to the `LOADS.LUM` file.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the file is missing or if there is a problem reading
    /// the file into the [`LoadsLum`] struct.
    pub fn new(path: &Path) -> Result<Self, LoadsLumError> {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let loads_lum = LoadsLum::read(&mut Cursor::new(buf))?;
        Ok(loads_lum)
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
    pub fn get_pointer_to_number_of_loads(&self) -> u32 {
        combine_words(
            self.pointer_to_number_of_loads_msb,
            self.pointer_to_number_of_loads_lsb,
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
impl Display for LoadsLum {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} Bytes\n{}\n{} | {} | {} Offsets\n{} Chars in Media Set PN\n{} Media Set PN\n{} \
             Media number\n{} Total Media sets\n{} Total loads\n{}\n{:?} User Data\n0x{:x} CRC",
            self.get_file_length() * 2,
            self.get_file_type_string(),
            self.get_pointer_to_media_set_pn_length(),
            self.get_pointer_to_number_of_loads(),
            self.get_pointer_to_user_defined_data(),
            self.media_set_pn_length,
            self.get_media_set_pn(),
            self.media_sequence_number_x,
            self.number_of_media_set_members_y,
            self.number_of_loads,
            self.loads[0],
            self.user_defined_data,
            self.file_crc
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_simple_loads_lum() {
        // 1 hex = 4 bits
        // 4 hex = 16 bits = 2 bytes
        let file = PathBuf::from("../test-data/LOADS.LUM");
        let loads_lum = LoadsLum::new(file.as_path()).unwrap();
        println!("{loads_lum}");

        assert_eq!(loads_lum.get_file_length(), 39);
        assert_eq!(loads_lum.media_file_format_verion, 0x8002);
        assert_eq!(loads_lum.get_pointer_to_media_set_pn_length(), 0x9);
        assert_eq!(loads_lum.get_pointer_to_number_of_loads(), 0x10);
        assert_eq!(loads_lum.get_pointer_to_user_defined_data(), 0x0);
        assert_eq!(loads_lum.media_set_pn_length, 10);
        assert_eq!(loads_lum.media_set_pn.len(), 5);
        assert_eq!(loads_lum.get_media_set_pn(), "ABCDEFGH12");
        assert_eq!(loads_lum.media_sequence_number_x, 1);
        assert_eq!(loads_lum.number_of_media_set_members_y, 1);
        assert_eq!(loads_lum.number_of_loads, 1);
        assert_eq!(loads_lum.loads.len(), 1);
        assert_eq!(loads_lum.user_defined_data, None);
        assert_eq!(loads_lum.file_crc, 0x5246);
    }
}
