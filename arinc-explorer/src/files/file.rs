use std::fmt::{self, Display, Formatter};

use binrw::binrw;

use crate::utils::vec16_to_string;

// TODO: pass number_of_media_set_files to support ARINC665-5
#[binrw]
#[brw(big)]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
#[allow(clippy::struct_field_names)]
pub struct File {
    file_pointer: u16,
    file_name_length: u16, // number of chars

    #[br(count = file_name_length.div_ceil(2))]
    file_name: Vec<u16>,

    file_pathname_length: u16, // number of chars

    #[br(count = file_pathname_length.div_ceil(2))]
    file_pathname: Vec<u16>,

    member_sequence_number: u16,
    file_crc: u16,
    // Seems like the following fields don't exist
    // file_check_value_length: u16,
    // #[br(if(file_check_value_length!=0))]
    // file_check_value_type: Option<u16>,

    // #[br(if(file_check_value_length!=0), count = file_check_value_length.div_ceil(2))]
    // file_check_value: Option<Vec<u16>>,
    // Expansion point no2
}
impl File {
    pub fn get_file_name(&self) -> String {
        // If filename_length is odd, an extra NUL byte is added at the end.
        // This function removes the NUL byte if it exists.
        vec16_to_string(&self.file_name, self.file_name_length as usize)
    }

    pub fn get_file_pathname(&self) -> String {
        // If file_pathname_length is odd, an extra NUL byte is added at the end.
        // This function removes the NUL byte if it exists.
        vec16_to_string(&self.file_pathname, self.file_pathname_length as usize)
    }
}
impl Display for File {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            " \
            \t{} Offsets\n \
            \t{} Chars in Filename\n \
            \t{} Filename\n \
            \t{} Chars in File Pathname\n \
            \t{} File Pathname\n \
            \t{} Sequence number\n \
            \t0x{:x} CRC\n \
            \n",
            self.file_pointer,
            self.file_name_length,
            self.get_file_name(),
            self.file_pathname_length,
            self.get_file_pathname(),
            self.member_sequence_number,
            self.file_crc
        )
    }
}
