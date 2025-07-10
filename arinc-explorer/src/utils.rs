pub fn combine_words(msb: u16, lsb: u16) -> u32 {
    let msb: u32 = msb.into();
    let lsb: u32 = lsb.into();
    (msb << 16) | lsb
}

pub fn vec16_to_string(input: &[u16], length: usize) -> String {
    let vec_u8: Vec<[u8; 2]> = input.iter().map(|w| w.to_be_bytes()).collect();
    let vec_u8: Vec<u8> = vec_u8.concat();
    let slice = &vec_u8[0..length];
    String::from_utf8_lossy(slice).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_words() {
        let msb = 0x0;
        let lsb = 0x0;
        assert_eq!(combine_words(msb, lsb), 0x0);

        let msb = 0x0;
        let lsb = 0xa;
        assert_eq!(combine_words(msb, lsb), 0xa);

        let msb = 0xa;
        let lsb = 0x0;
        assert_eq!(combine_words(msb, lsb), 0xa0000);

        let msb = 0xa;
        let lsb = 0xa;
        assert_eq!(combine_words(msb, lsb), 0xa000a);

        let msb = 0xffff;
        let lsb = 0xeeee;
        assert_eq!(combine_words(msb, lsb), 0xffff_eeee);
    }

    #[test]
    fn test_vec16_to_string() {
        let input = vec![0x4142_u16, 0x4344_u16];
        let length = 4;
        assert_eq!(vec16_to_string(input.as_slice(), length), "ABCD");

        let input = vec![0x4142_u16, 0x4344_u16];
        let length = 3;
        assert_eq!(vec16_to_string(input.as_slice(), length), "ABC");

        let input = vec![0x4142_u16];
        let length = 2;
        assert_eq!(vec16_to_string(input.as_slice(), length), "AB");

        let input = vec![0x4142_u16];
        let length = 1;
        assert_eq!(vec16_to_string(input.as_slice(), length), "A");
    }
}
