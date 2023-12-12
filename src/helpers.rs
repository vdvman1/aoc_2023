pub fn try_ascii_to_digit(c: u8) -> Option<u32> {
    let digit = c.wrapping_sub(b'0');
    if digit > 9 {
        None
    } else {
        Some(digit as u32)
    }
}

pub fn ascii_to_digit(c: u8) -> u32 {
    (c - b'0').into()
}
