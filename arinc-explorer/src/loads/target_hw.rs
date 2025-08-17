use std::fmt::{self, Display, Formatter};

use binrw::binrw;

use crate::utils::vec16_to_string;

#[binrw]
#[brw(big)]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct TargetHW {
    target_hw_id_length: u16, // number of chars

    #[br(count = target_hw_id_length.div_ceil(2))]
    target_hw_id: Vec<u16>,
}
impl TargetHW {
    pub fn get_target_hw_id(&self) -> String {
        // If target_hw_id_length is odd, an extra NUL byte is added at the end.
        // This function removes the NUL byte if it exists.
        vec16_to_string(&self.target_hw_id, self.target_hw_id_length as usize)
    }
}
impl Display for TargetHW {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            " \
            \t\t{} Chars in Target HW ID\n \
            \t\t{} Target HW ID\n \
            \n",
            self.target_hw_id_length,
            self.get_target_hw_id(),
        )
    }
}
