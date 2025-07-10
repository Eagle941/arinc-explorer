use std::fmt::{self, Display, Formatter};

use binrw::binrw;

use super::target_hw::TargetHW;
use crate::utils::vec16_to_string;

#[binrw]
#[brw(big)]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Lsp {
    load_pointer: u16,
    load_pn_length: u16, // number of chars

    #[br(count = load_pn_length.div_ceil(2))]
    load_pn: Vec<u16>,

    header_filename_length: u16, // number of chars

    #[br(count = header_filename_length.div_ceil(2))]
    header_filename: Vec<u16>,

    member_sequence_number: u16,
    number_of_target_hw_ids: u16,

    #[br(count = number_of_target_hw_ids)]
    target_hw_ids: Vec<TargetHW>,
    // Expansion point no2
}
impl Lsp {
    pub fn get_load_pn(&self) -> String {
        // If load_pn_length is odd, an extra NUL byte is added at the end.
        // This function removes the NUL byte if it exists.
        vec16_to_string(&self.load_pn, self.load_pn_length as usize)
    }

    pub fn get_header_filename(&self) -> String {
        // If header_filename_length is odd, an extra NUL byte is added at the end.
        // This function removes the NUL byte if it exists.
        vec16_to_string(&self.header_filename, self.header_filename_length as usize)
    }
}
impl Display for Lsp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "\t{} Offsets\n\t{} Chars in Load PN\n\t{} Load PN\n\t{} Chars in Header \
             filename\n\t{} Header filename\n\t{} Sequence number\n\t{} Total target HW IDs\n{}",
            self.load_pointer,
            self.load_pn_length,
            self.get_load_pn(),
            self.header_filename_length,
            self.get_header_filename(),
            self.member_sequence_number,
            self.number_of_target_hw_ids,
            self.target_hw_ids[0]
        )
    }
}
